import { createQuery } from "@tanstack/svelte-query"

import { classificationService } from "./service"

export function useResearchLinesQuery() {
	return createQuery(() => ({
		queryKey: ["research-lines"],
		queryFn: () => classificationService.researchLines(),
		staleTime: 300_000,
	}))
}

export function useAllSubfieldsQuery() {
	return createQuery(() => ({
		queryKey: ["research-subfields", "all"],
		queryFn: () => classificationService.subfields(undefined, 10_000),
		staleTime: 300_000,
	}))
}
