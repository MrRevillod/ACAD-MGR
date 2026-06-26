import { request } from "$lib/shared/http/request"
import type { AcademicCategory, AcademicPlanta } from "$lib/types"

export const categoriesService = {
	list(params?: { name?: string; planta?: AcademicPlanta }): Promise<AcademicCategory[]> {
		return request<AcademicCategory[]>({ method: "GET", url: "/academic-categories", params })
	},

	get(id: string): Promise<AcademicCategory> {
		return request<AcademicCategory>({ method: "GET", url: `/academic-categories/${id}` })
	},

	create(data: { name: string; planta: AcademicPlanta }): Promise<AcademicCategory> {
		return request<AcademicCategory>({ method: "POST", url: "/academic-categories", data })
	},
}
