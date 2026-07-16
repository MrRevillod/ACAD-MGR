import type { GetWorksParams, SyncResult } from "./dtos"

import { worksService } from "./service"
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"

export function useWorksQuery(getParams: () => GetWorksParams) {
	return createQuery(() => ({
		queryKey: ["works", "list", getParams()],
		queryFn: () => worksService.list(getParams()),
		staleTime: 30_000,
		refetchOnWindowFocus: false,
	}))
}

export function useWorkDetailQuery(getId: () => string) {
	return createQuery(() => ({
		queryKey: ["works", "detail", getId()],
		queryFn: () => worksService.get(getId()),
		enabled: Boolean(getId()),
	}))
}

export function useWorksByAcademicQuery(
	getAcademicId: () => string,
	getParams: () => Omit<GetWorksParams, "academicId"> = () => ({}),
) {
	return createQuery(() => ({
		queryKey: ["works", "by-academic", getAcademicId(), getParams()],
		queryFn: () => worksService.listByAcademic(getAcademicId(), getParams()),
		enabled: Boolean(getAcademicId()),
	}))
}

export function useSyncWorksMutation() {
	const qc = useQueryClient()
	return createMutation<SyncResult, Error, string>(() => ({
		mutationFn: (academicId: string) => worksService.sync(academicId),
		onSuccess: () => {
			void qc.invalidateQueries({ queryKey: ["works"] })
		},
	}))
}
