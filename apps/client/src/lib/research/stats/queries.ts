import { createQuery } from "@tanstack/svelte-query"
import { statsService } from "./service"

import type { AcademicStatsQuery, DepartmentDetailQuery, StatsQuery } from "./dtos"

const STALE_TIME = 5 * 60 * 1000
const GC_TIME = 10 * 60 * 1000

export function useWorksStatsQuery(queryParams: () => StatsQuery) {
	return createQuery(() => ({
		queryKey: ["stats", "works", queryParams()],
		queryFn: () => statsService.getWorksStats(queryParams()),
		staleTime: STALE_TIME,
		gcTime: GC_TIME,
	}))
}

export function useDepartmentDetailQuery(
	id: () => string,
	queryParams: () => DepartmentDetailQuery,
) {
	return createQuery(() => ({
		queryKey: ["stats", "department", id(), queryParams()],
		queryFn: () => statsService.getDepartmentDetail(id(), queryParams()),
		staleTime: STALE_TIME,
		gcTime: GC_TIME,
		enabled: Boolean(id()),
	}))
}

export function useAcademicStatsQuery(id: () => string, queryParams: () => AcademicStatsQuery) {
	return createQuery(() => ({
		queryKey: ["stats", "academic", id(), queryParams()],
		queryFn: () => statsService.getAcademicStats(id(), queryParams()),
		staleTime: STALE_TIME,
		gcTime: GC_TIME,
		enabled: Boolean(id()),
	}))
}
