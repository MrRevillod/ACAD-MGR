import type { CollaborationGraphDTO } from "./dtos"

import { http } from "$shared/http/client"

export interface CollaborationThresholds {
	scoreThreshold?: number
	minCoincidences?: number
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
				...(thresholds.scoreThreshold != null && {
					score_threshold: thresholds.scoreThreshold,
				}),
				...(thresholds.minCoincidences != null && {
					min_coincidences: thresholds.minCoincidences,
				}),
			},
		})
	}
}

export const collaborationsService = new CollaborationsService()
