import { http } from "$lib/shared/http/client"
import type { DegreeDTO, CreateDegreeDto, UpdateDegreeDto } from "./dtos"
import { Degree } from "./entity"

class DegreeService {
	public async listByAcademic(academicId: string): Promise<Degree[]> {
		const data = await http.request<DegreeDTO[]>({
			method: "GET",
			url: `/degrees/academic/${academicId}`,
		})

		return data.map((dto) => Degree.fromDTO(dto))
	}

	public async create(data: CreateDegreeDto): Promise<Degree> {
		const dto = await http.request<DegreeDTO>({
			method: "POST",
			url: "/degrees",
			data,
		})

		return Degree.fromDTO(dto)
	}

	public async update(id: string, data: UpdateDegreeDto): Promise<Degree> {
		const dto = await http.request<DegreeDTO>({
			method: "PATCH",
			url: `/degrees/${id}`,
			data,
		})

		return Degree.fromDTO(dto)
	}
}

export const degreeService = new DegreeService()
