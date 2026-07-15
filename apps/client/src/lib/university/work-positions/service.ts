import { http } from "$lib/shared/http/client"
import type { AcademicWorkPosition, CreatePositionDto } from "./dtos"

export const positionService = {
	list(params?: { name?: string }): Promise<AcademicWorkPosition[]> {
		return http.request<AcademicWorkPosition[]>({
			method: "GET",
			url: "/work-positions",
			params,
		})
	},

	create(data: CreatePositionDto): Promise<AcademicWorkPosition> {
		return http.request<AcademicWorkPosition>({
			method: "POST",
			url: "/work-positions",
			data,
		})
	},
}
