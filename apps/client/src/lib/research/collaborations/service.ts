import type { CollaborationGraphDTO } from "./dtos"

import { http } from "$shared/http/client"

class CollaborationsService {
	public async get(academicId: string): Promise<CollaborationGraphDTO> {
		return http.request<CollaborationGraphDTO>({
			method: "GET",
			url: `/collaborations/${academicId}`,
		})
	}
}

export const collaborationsService = new CollaborationsService()
