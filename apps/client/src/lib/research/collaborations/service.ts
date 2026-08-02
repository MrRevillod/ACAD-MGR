import type { CollaborationGraphDTO } from "./dtos"

import { http } from "$shared/http/client"

export interface CollaborationThresholds {
	topicThreshold?: number
	keywordThreshold?: number
}

class CollaborationsService {
	public async get(
		academicId: string,
		thresholds: CollaborationThresholds = {},
	): Promise<CollaborationGraphDTO> {
		return http.request<CollaborationGraphDTO>({
			method: "GET",
			url: `/collaborations/${academicId}`,
			params: {
				...(thresholds.topicThreshold != null && {
					topic_threshold: thresholds.topicThreshold,
				}),
				...(thresholds.keywordThreshold != null && {
					keyword_threshold: thresholds.keywordThreshold,
				}),
			},
		})
	}
}

export const collaborationsService = new CollaborationsService()
