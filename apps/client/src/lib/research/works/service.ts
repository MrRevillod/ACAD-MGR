import type { WorkDTO, WorkDetailDTO, GetWorksParams, SyncResult, WorkOverridesInput } from "./dtos"

import { http } from "$shared/http/client"
import { Work, WorkDetail } from "./entity"

class WorksService {
	public async list(params: GetWorksParams = {}): Promise<Work[]> {
		const data = await http.request<WorkDTO[]>({
			method: "GET",
			url: "/works",
			params,
		})

		return data.map((dto) => Work.fromDTO(dto))
	}

	public async get(id: string): Promise<WorkDetail> {
		const dto = await http.request<WorkDetailDTO>({
			method: "GET",
			url: `/works/${id}`,
		})

		return WorkDetail.fromDTO(dto)
	}

	public async listByAcademic(
		academicId: string,
		params: Omit<GetWorksParams, "academicId"> = {},
	): Promise<Work[]> {
		const data = await http.request<WorkDTO[]>({
			method: "GET",
			url: `/works/academic/${academicId}`,
			params,
		})

		return data.map((dto) => Work.fromDTO(dto))
	}

	public async sync(academicId: string): Promise<SyncResult> {
		return http.request<SyncResult>({
			method: "POST",
			url: `/works/sync/${academicId}`,
		})
	}

	public async syncAll(): Promise<void> {
		await http.request<null>({
			method: "POST",
			url: "/works/sync-all",
		})
	}

	public async updateOverrides(id: string, data: WorkOverridesInput): Promise<void> {
		await http.request<null>({
			method: "PUT",
			url: `/works/${id}/overrides`,
			data,
		})
	}

	public async clearOverrides(id: string): Promise<void> {
		await http.request<null>({
			method: "DELETE",
			url: `/works/${id}/overrides`,
		})
	}
}

export const worksService = new WorksService()
