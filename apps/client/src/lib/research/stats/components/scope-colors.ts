import type { ScopeSeries } from "$stats/dtos"

export const SCOPE_PALETTE = ["#0075B4", "#C9A500", "#0F9D8F", "#7C3AED", "#EA580C", "#0EA5E9"]

export type ColoredScopeSeries = ScopeSeries & { color: string }

export function colorForIndex(index: number): string {
	return SCOPE_PALETTE[index % SCOPE_PALETTE.length]
}

export function withScopeColors(items: ScopeSeries[]): ColoredScopeSeries[] {
	return items.map((s, i) => ({ ...s, color: colorForIndex(i) }))
}
