import { http } from "$lib/shared/http/client"
import { AcademicCategory } from "./entity"

import type { AcademicCategoryDTO, CreateCategoryDTO } from "./dtos"

class CategoryService {
	public list(params?: { name?: string; planta?: string }): Promise<AcademicCategory[]> {
		const category = http.request<AcademicCategoryDTO[]>({
			method: "GET",
			url: "/academic-categories",
			params,
		})

		return category.then((dtos) => dtos.map((dto) => AcademicCategory.fromDTO(dto)))
	}

	public get(id: string): Promise<AcademicCategory> {
		const category = http.request<AcademicCategoryDTO>({
			method: "GET",
			url: `/academic-categories/${id}`,
		})

		return category.then((dto) => AcademicCategory.fromDTO(dto))
	}

	public create(data: CreateCategoryDTO): Promise<AcademicCategory> {
		const category = http.request<AcademicCategoryDTO>({
			method: "POST",
			url: "/academic-categories",
			data,
		})

		return category.then((dto) => AcademicCategory.fromDTO(dto))
	}
}

export const categoryService = new CategoryService()
