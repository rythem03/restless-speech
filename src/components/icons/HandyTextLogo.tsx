// RESTLESS SPEECH logo + wordmark (replaces the original Handy wordmark).
import logo from "@/assets/restless-logo.png";

const HandyTextLogo = ({
  width = 120,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <div
      className={className}
      style={{
        width,
        display: "flex",
        alignItems: "center",
        gap: Math.round(width * 0.07),
      }}
      role="img"
      aria-label="RESTLESS SPEECH"
    >
      <img
        src={logo}
        alt=""
        draggable={false}
        style={{
          width: Math.round(width * 0.32),
          height: Math.round(width * 0.32),
          borderRadius: Math.round(width * 0.06),
          flexShrink: 0,
        }}
      />
      <span
        style={{
          fontWeight: 800,
          fontSize: Math.round(width * 0.115),
          lineHeight: 1.05,
          letterSpacing: "0.04em",
        }}
      >
        RESTLESS
        <br />
        SPEECH
      </span>
    </div>
  );
};

export default HandyTextLogo;
