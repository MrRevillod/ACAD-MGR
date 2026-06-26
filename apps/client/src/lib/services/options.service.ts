import { request } from "$lib/shared/http/request"
import type { AcademicCategoryOption, AcademicOption } from "$lib/types"

export const optionsService = {
	list(params?: { category_id?: string }): Promise<AcademicCategoryOption[]> {
		return request<AcademicCategoryOption[]>({ method: "GET", url: "/category-options", params })
	},

	get(id: string): Promise<AcademicCategoryOption> {
		return request<AcademicCategoryOption>({ method: "GET", url: `/category-options/${id}` })
	},

	create(data: {
		categoryId: string
		option: AcademicOption
		hours?: number | null
	}): Promise<AcademicCategoryOption> {
		return request<AcademicCategoryOption>({ method: "POST", url: "/category-options", data })
	},
}
