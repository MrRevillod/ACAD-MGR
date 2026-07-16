import { http } from "$lib/shared/http/client"
import type { AcademicCategoryOptionDTO, CreateOptionDto, GetOptionsParams } from "./dtos"
import { AcademicCategoryOption } from "./entity"

class OptionService {
	public async list(params?: GetOptionsParams): Promise<AcademicCategoryOption[]> {
		const data = await http.request<AcademicCategoryOptionDTO[]>({
			method: "GET",
			url: "/category-options",
			params,
		})
		return data.map((dto) => AcademicCategoryOption.fromDTO(dto))
	}

	public async get(id: string): Promise<AcademicCategoryOption> {
		const dto = await http.request<AcademicCategoryOptionDTO>({
			method: "GET",
			url: `/category-options/${id}`,
		})
		return AcademicCategoryOption.fromDTO(dto)
	}

	public async create(data: CreateOptionDto): Promise<AcademicCategoryOption> {
		const dto = await http.request<AcademicCategoryOptionDTO>({
			method: "POST",
			url: "/category-options",
			data,
		})
		return AcademicCategoryOption.fromDTO(dto)
	}
}

export const optionService = new OptionService()
