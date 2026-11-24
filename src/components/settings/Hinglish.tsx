import React from "react";
import { useTranslation } from "react-i18next";
import { ToggleSwitch } from "../ui/ToggleSwitch";
import { useSettings } from "../../hooks/useSettings";

interface HinglishProps {
  descriptionMode?: "inline" | "tooltip";
  grouped?: boolean;
}

/** Restless Speech: write Hindi speech in Roman letters ("Hinglish"). */
export const HinglishSetting: React.FC<HinglishProps> = React.memo(
  ({ descriptionMode = "tooltip", grouped = false }) => {
    const { t } = useTranslation();
    const { getSetting, updateSetting, isUpdating } = useSettings();
    const enabled = getSetting("hinglish_enabled") ?? false;

    const handleChange = async (next: boolean) => {
      await updateSetting("hinglish_enabled", next);
      if (!next) return;
      // Hindi speech needs a Hindi-capable model and the Hindi language, so
      // turning Hinglish on sets both up. Either can be refused (for example
      // mid-recording); the switch itself still applies.
      try {
        const spoken = getSetting("spoken_languages") ?? [];
        if (!spoken.includes("hi")) {
          await updateSetting("spoken_languages", [...spoken, "hi"]);
        }
        await updateSetting("selected_language", "hi");
      } catch {
        // The user can still pick Hindi by hand in the language list.
      }
    };

    return (
      <ToggleSwitch
        checked={enabled}
        onChange={handleChange}
        isUpdating={isUpdating("hinglish_enabled")}
        label={t("settings.languages.hinglish.title")}
        description={t("settings.languages.hinglish.description")}
        descriptionMode={descriptionMode}
        grouped={grouped}
      />
    );
  },
);
