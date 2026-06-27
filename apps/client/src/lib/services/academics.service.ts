import { request } from "$lib/shared/http/request"
import type {
	AcademicView,
	AcademicSortField,
	AcademicPlanta,
	AcademicOption,
	UpdateAcademicDto,
} from "$lib/types"

export interface GetAcademicsParams {
	search?: string
	sort?: AcademicSortField
	career_id?: string
	department_id?: string
	category_id?: string
	planta?: AcademicPlanta
	option?: AcademicOption
}

export const academicsService = {
	list(params?: GetAcademicsParams): Promise<AcademicView[]> {
		return request<AcademicView[]>({ method: "GET", url: "/academics", params })
	},

	get(id: string): Promise<AcademicView> {
		return request<AcademicView>({ method: "GET", url: `/academics/${id}` })
	},

	update(id: string, data: UpdateAcademicDto): Promise<AcademicView> {
		return request<AcademicView>({ method: "PATCH", url: `/academics/${id}`, data })
	},

	async import(file: File): Promise<void> {
		const formData = new FormData()
		formData.append("file", file)
		return request<void>({ method: "POST", url: "/academics/import", data: formData })
	},
}
