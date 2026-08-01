import { createQuery } from "@tanstack/svelte-query"

import { collaborationsService } from "./service"

export function useCollaborationGraphQuery(getAcademicId: () => string) {
	return createQuery(() => ({
		queryKey: ["collaborations", getAcademicId()],
		queryFn: () => collaborationsService.get(getAcademicId()),
		enabled: Boolean(getAcademicId()),
		staleTime: 60_000,
	}))
}
