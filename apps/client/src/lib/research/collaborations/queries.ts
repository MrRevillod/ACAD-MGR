import { createQuery } from "@tanstack/svelte-query"

import { collaborationsService, type CollaborationThresholds } from "./service"

export function useCollaborationGraphQuery(
	getAcademicId: () => string,
	getThresholds: () => CollaborationThresholds,
) {
	return createQuery(() => {
		const academicId = getAcademicId()
		const thresholds = getThresholds()
		return {
			queryKey: [
				"collaborations",
				academicId,
				thresholds.topicThreshold,
				thresholds.keywordThreshold,
			],
			queryFn: () => collaborationsService.get(academicId, thresholds),
			enabled: Boolean(academicId),
			staleTime: 30_000,
		}
	})
}
