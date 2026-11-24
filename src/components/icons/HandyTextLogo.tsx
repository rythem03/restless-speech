// Restless Speech wordmark (replaces the original Handy wordmark).
const HandyTextLogo = ({
  width,
  height,
  className,
}: {
  width?: number;
  height?: number;
  className?: string;
}) => {
  return (
    <svg
      width={width}
      height={height}
      className={className}
      viewBox="0 0 300 120"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      role="img"
      aria-label="Restless Speech"
    >
      <text
        x="150"
        y="52"
        textAnchor="middle"
        className="logo-primary"
        stroke="var(--color-wordmark-stroke)"
        strokeWidth="2"
        paintOrder="stroke"
        style={{ font: "800 50px system-ui, 'Segoe UI', sans-serif" }}
      >
        Restless
      </text>
      <text
        x="150"
        y="106"
        textAnchor="middle"
        className="logo-primary"
        stroke="var(--color-wordmark-stroke)"
        strokeWidth="2"
        paintOrder="stroke"
        style={{ font: "800 50px system-ui, 'Segoe UI', sans-serif" }}
      >
        Speech
      </text>
    </svg>
  );
};

export default HandyTextLogo;
