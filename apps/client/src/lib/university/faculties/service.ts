import { http } from "$lib/shared/http/client"
import type { FacultyDTO, GetFacultiesParams } from "./dtos"
import { Faculty } from "./entity"

class FacultyService {
	public async list(params?: GetFacultiesParams): Promise<Faculty[]> {
		const data = await http.request<FacultyDTO[]>({
			method: "GET",
			url: "/faculties",
			params,
		})
		return data.map((dto) => Faculty.fromDTO(dto))
	}
}

export const facultyService = new FacultyService()
