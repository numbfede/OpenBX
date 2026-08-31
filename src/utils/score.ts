export type ScoreLabel = "CAN BE OPTIMIZED" | "PC READY" | "FULLY OPTIMIZED";

export function scoreLabel(score: number): ScoreLabel {
  if (score >= 90) return "FULLY OPTIMIZED";
  if (score >= 60) return "PC READY";
  return "CAN BE OPTIMIZED";
}

export function computeScore(optimized: number, applicable: number): number {
  if (applicable <= 0) return 100;
  return Math.round((optimized / applicable) * 100);
}
