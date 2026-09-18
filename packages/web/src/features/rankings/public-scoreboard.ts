const SUPPORTED_CONTEST_TYPES = new Set(['icpc', 'ioi', 'codelink']);

// Custom scoreboard plugins must explicitly implement the public-view contract
// before the application labels their authenticated output as public.
export function supportsPublicScoreboard(contestType?: string | null): boolean {
  return contestType != null && SUPPORTED_CONTEST_TYPES.has(contestType);
}

export function publicScoreboardHref(contestId: number): string {
  return `/contests/${contestId}/rankings?view=public`;
}
