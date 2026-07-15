import { http } from "$lib/shared/http/client"
import type { Degree, CreateDegreeDto, UpdateDegreeDto } from "./dtos"

export const degreeService = {
	listByAcademic(academicId: string): Promise<Degree[]> {
		return http.request<Degree[]>({
			method: "GET",
			url: `/degrees/academic/${academicId}`,
		})
	},

	create(data: CreateDegreeDto): Promise<Degree> {
		return http.request<Degree>({
			method: "POST",
			url: "/degrees",
			data,
		})
	},

	update(id: string, data: UpdateDegreeDto): Promise<Degree> {
		return http.request<Degree>({
			method: "PATCH",
			url: `/degrees/${id}`,
			data,
		})
	},
}
