import { animate, useMotionValue, useMotionValueEvent } from "motion/react";
import { useEffect, useState } from "react";
import { scoreLabel } from "../utils/score";

export function ScoreRing({ score, size = 248 }: { score: number | null; size?: number }) {
  const [display, setDisplay] = useState(score ?? 0);
  const motionScore = useMotionValue(score ?? 0);
  const radius = (size - 18) / 2;
  const circumference = 2 * Math.PI * radius;
  const safeScore = Math.max(0, Math.min(100, score ?? 0));
  const offset = circumference - (safeScore / 100) * circumference;
  const label = score == null ? "IN ATTESA" : scoreLabel(safeScore);
  const tone = safeScore >= 90 ? "text-ready" : safeScore >= 60 ? "text-cream" : "text-warn";

  useEffect(() => {
    const controls = animate(motionScore, safeScore, { duration: 0.7, ease: [0.22, 1, 0.36, 1] });
    return () => controls.stop();
  }, [motionScore, safeScore]);

  useMotionValueEvent(motionScore, "change", (value) => setDisplay(Math.round(value)));

  return (
    <div className="relative grid place-items-center" style={{ width: size, height: size }}>
      <svg width={size} height={size} className="-rotate-90">
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke="rgb(255 255 255 / 0.08)"
          strokeWidth="10"
        />
        <circle
          cx={size / 2}
          cy={size / 2}
          r={radius}
          fill="none"
          stroke="currentColor"
          className={tone}
          strokeWidth="10"
          strokeLinecap="round"
          strokeDasharray={circumference}
          strokeDashoffset={offset}
          style={{ transition: "stroke-dashoffset 700ms cubic-bezier(0.22, 1, 0.36, 1)" }}
        />
      </svg>
      <div className="absolute inset-0 grid place-items-center text-center">
        <div>
          <div className="text-[72px] font-medium leading-none tracking-tight">{score == null ? "—" : display}</div>
          <div className={`mt-3 text-[12px] uppercase tracking-[0.22em] ${tone}`}>{label}</div>
        </div>
      </div>
    </div>
  );
}
