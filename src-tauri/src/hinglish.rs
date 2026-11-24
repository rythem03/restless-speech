//! Hinglish output: rewrites Devanagari (Hindi) text in everyday Roman
//! letters, the way people type Hindi in chats ("main ghar ja raha hoon").
//!
//! Text that is already in Roman letters (English words, numbers, punctuation)
//! passes through untouched, so mixed Hindi/English speech comes out as one
//! consistent Hinglish sentence.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Vowel {
    /// The consonant's built-in "a" (no vowel sign written).
    Inherent,
    /// Virama: the consonant has no vowel at all (part of a cluster).
    Killed,
    A,
    Aa,
    I,
    Ii,
    U,
    Uu,
    Ri,
    E,
    Ai,
    O,
    Au,
}

#[derive(Debug)]
struct Unit {
    consonant: &'static str,
    vowel: Vowel,
    nasal: bool,
    visarga: bool,
    /// Set by schwa deletion.
    silent: bool,
}

fn consonant(c: char) -> Option<&'static str> {
    Some(match c {
        'क' => "k",
        'ख' => "kh",
        'ग' => "g",
        'घ' => "gh",
        'ङ' => "n",
        'च' => "ch",
        'छ' => "chh",
        'ज' => "j",
        'झ' => "jh",
        'ञ' => "n",
        'ट' => "t",
        'ठ' => "th",
        'ड' => "d",
        'ढ' => "dh",
        'ण' => "n",
        'त' => "t",
        'थ' => "th",
        'द' => "d",
        'ध' => "dh",
        'न' => "n",
        'प' => "p",
        'फ' => "ph",
        'ब' => "b",
        'भ' => "bh",
        'म' => "m",
        'य' => "y",
        'र' => "r",
        'ल' => "l",
        'ळ' => "l",
        'व' => "v",
        'श' => "sh",
        'ष' => "sh",
        'स' => "s",
        'ह' => "h",
        // Precomposed nukta letters.
        '\u{0958}' => "q",
        '\u{0959}' => "kh",
        '\u{095A}' => "gh",
        '\u{095B}' => "z",
        '\u{095C}' => "r",
        '\u{095D}' => "rh",
        '\u{095E}' => "f",
        '\u{095F}' => "y",
        _ => return None,
    })
}

/// A consonant followed by the combining nukta (U+093C).
fn with_nukta(base: &'static str) -> &'static str {
    match base {
        "k" => "q",
        "kh" => "kh",
        "g" => "gh",
        "j" => "z",
        "d" => "r",
        "dh" => "rh",
        "ph" => "f",
        other => other,
    }
}

fn independent_vowel(c: char) -> Option<Vowel> {
    Some(match c {
        'अ' => Vowel::A,
        'आ' => Vowel::Aa,
        'इ' => Vowel::I,
        'ई' => Vowel::Ii,
        'उ' => Vowel::U,
        'ऊ' => Vowel::Uu,
        'ऋ' => Vowel::Ri,
        'ए' | 'ऍ' | 'ऎ' => Vowel::E,
        'ऐ' => Vowel::Ai,
        'ओ' | 'ऑ' | 'ऒ' => Vowel::O,
        'औ' => Vowel::Au,
        _ => return None,
    })
}

fn vowel_sign(c: char) -> Option<Vowel> {
    Some(match c {
        'ा' => Vowel::Aa,
        'ि' => Vowel::I,
        'ी' => Vowel::Ii,
        'ु' => Vowel::U,
        'ू' => Vowel::Uu,
        'ृ' => Vowel::Ri,
        'े' | 'ॅ' | 'ॆ' => Vowel::E,
        'ै' => Vowel::Ai,
        'ो' | 'ॉ' | 'ॊ' => Vowel::O,
        'ौ' => Vowel::Au,
        _ => return None,
    })
}

fn is_devanagari(c: char) -> bool {
    ('\u{0900}'..='\u{097F}').contains(&c)
}

/// True when `text` contains any Devanagari letter.
pub fn has_devanagari(text: &str) -> bool {
    text.chars().any(is_devanagari)
}

/// Rewrites every Devanagari word in `text` in Roman letters; everything else
/// is kept as it is.
pub fn to_hinglish(text: &str) -> String {
    if !has_devanagari(text) {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut word = String::new();
    for c in text.chars() {
        if is_devanagari(c) && !matches!(c, '।' | '॥') {
            word.push(c);
            continue;
        }
        if !word.is_empty() {
            out.push_str(&romanize_word(&word));
            word.clear();
        }
        match c {
            '।' | '॥' => out.push('.'),
            other => out.push(other),
        }
    }
    if !word.is_empty() {
        out.push_str(&romanize_word(&word));
    }
    out
}

fn parse(word: &str) -> (Vec<Unit>, Vec<(usize, char)>) {
    let mut units: Vec<Unit> = Vec::new();
    // Devanagari digits and anything unrecognised, keyed by the unit index
    // they follow, so they stay in place.
    let mut extras: Vec<(usize, char)> = Vec::new();
    for c in word.chars() {
        if let Some(cons) = consonant(c) {
            units.push(Unit {
                consonant: cons,
                vowel: Vowel::Inherent,
                nasal: false,
                visarga: false,
                silent: false,
            });
        } else if let Some(v) = independent_vowel(c) {
            units.push(Unit {
                consonant: "",
                vowel: v,
                nasal: false,
                visarga: false,
                silent: false,
            });
        } else if let Some(v) = vowel_sign(c) {
            let attaches = matches!(
                units.last(),
                Some(u) if !u.consonant.is_empty() && u.vowel == Vowel::Inherent
            );
            if attaches {
                if let Some(u) = units.last_mut() {
                    u.vowel = v;
                }
            } else {
                units.push(Unit {
                    consonant: "",
                    vowel: v,
                    nasal: false,
                    visarga: false,
                    silent: false,
                });
            }
        } else {
            match c {
                '्' => {
                    if let Some(u) = units.last_mut() {
                        if u.vowel == Vowel::Inherent {
                            u.vowel = Vowel::Killed;
                        }
                    }
                }
                '़' => {
                    if let Some(u) = units.last_mut() {
                        u.consonant = with_nukta(u.consonant);
                    }
                }
                'ं' | 'ँ' => {
                    if let Some(u) = units.last_mut() {
                        u.nasal = true;
                    }
                }
                'ः' => {
                    if let Some(u) = units.last_mut() {
                        u.visarga = true;
                    }
                }
                '०'..='९' => {
                    let digit = char::from(b'0' + (c as u32 - '०' as u32) as u8);
                    extras.push((units.len(), digit));
                }
                _ => {}
            }
        }
    }
    (units, extras)
}

/// Hindi drops the built-in "a" at the end of a word ("कर" → "kar") and in
/// the middle between two pronounced vowels ("करना" → "karna").
fn delete_schwas(units: &mut [Unit]) {
    let n = units.len();
    if n < 2 {
        return;
    }
    let last = n - 1;
    if units[last].vowel == Vowel::Inherent
        && !units[last].consonant.is_empty()
        && !units[last].nasal
    {
        units[last].silent = true;
    }
    for i in 1..last {
        let prev_sounded = units[i - 1].vowel != Vowel::Killed && !units[i - 1].silent;
        let next_has_vowel = !units[i + 1].consonant.is_empty()
            && units[i + 1].vowel != Vowel::Killed
            && !units[i + 1].silent;
        let unit = &units[i];
        if unit.vowel == Vowel::Inherent
            && !unit.consonant.is_empty()
            && !unit.nasal
            && prev_sounded
            && next_has_vowel
        {
            units[i].silent = true;
        }
    }
}

fn romanize_word(word: &str) -> String {
    let (mut units, extras) = parse(word);
    delete_schwas(&mut units);
    let n = units.len();
    let mut out = String::new();
    let mut extra_iter = extras.iter().peekable();
    for (i, unit) in units.iter().enumerate() {
        while let Some((at, ch)) = extra_iter.peek() {
            if *at <= i {
                out.push(*ch);
                extra_iter.next();
            } else {
                break;
            }
        }
        out.push_str(unit.consonant);
        let word_final = i + 1 == n;
        let vowel = match unit.vowel {
            Vowel::Inherent if unit.silent => "",
            Vowel::Inherent | Vowel::A => "a",
            Vowel::Killed => "",
            // Chat-style spelling: long vowels are doubled inside a word but
            // written single at the end ("raha", "nahi", "tu").
            Vowel::Aa if word_final && !unit.nasal => "a",
            Vowel::Aa => "aa",
            Vowel::I => "i",
            Vowel::Ii if word_final => "i",
            Vowel::Ii => "ee",
            Vowel::U => "u",
            Vowel::Uu if word_final && !unit.nasal => "u",
            Vowel::Uu => "oo",
            Vowel::Ri => "ri",
            Vowel::E => "e",
            Vowel::Ai => "ai",
            Vowel::O => "o",
            Vowel::Au => "au",
        };
        out.push_str(vowel);
        if unit.nasal {
            out.push('n');
        }
        if unit.visarga {
            out.push('h');
        }
    }
    for (_, ch) in extra_iter {
        out.push(*ch);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn everyday_sentences() {
        assert_eq!(to_hinglish("मैं घर जा रहा हूँ"), "main ghar ja raha hoon");
        assert_eq!(to_hinglish("क्या कर रहे हो?"), "kya kar rahe ho?");
        assert_eq!(to_hinglish("मुझे करना है।"), "mujhe karna hai.");
        assert_eq!(to_hinglish("नहीं"), "nahin");
        assert_eq!(to_hinglish("हाँ"), "haan");
    }

    #[test]
    fn english_words_in_devanagari() {
        assert_eq!(to_hinglish("मीटिंग"), "meeting");
    }

    #[test]
    fn roman_text_is_untouched() {
        assert_eq!(to_hinglish("Hello, kal milte hain"), "Hello, kal milte hain");
        assert_eq!(to_hinglish("मेरा phone"), "mera phone");
    }

    #[test]
    fn digits() {
        assert_eq!(to_hinglish("२०२६"), "2026");
    }
}
