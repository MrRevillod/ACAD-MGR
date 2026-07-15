import { http } from "$lib/shared/http/client"
import type { AcademicCategoryOption, CreateOptionDto } from "./dtos"

export const optionService = {
	list(params?: { category_id?: string }): Promise<AcademicCategoryOption[]> {
		return http.request<AcademicCategoryOption[]>({
			method: "GET",
			url: "/category-options",
			params,
		})
	},

	get(id: string): Promise<AcademicCategoryOption> {
		return http.request<AcademicCategoryOption>({
			method: "GET",
			url: `/category-options/${id}`,
		})
	},

	create(data: CreateOptionDto): Promise<AcademicCategoryOption> {
		return http.request<AcademicCategoryOption>({
			method: "POST",
			url: "/category-options",
			data,
		})
	},
}
