import { http } from "$lib/shared/http/client"
import type { DepartmentDTO, GetDepartmentsParams } from "./dtos"
import { Department } from "./entity"

class DepartmentService {
	public async list(params?: GetDepartmentsParams): Promise<Department[]> {
		const data = await http.request<DepartmentDTO[]>({
			method: "GET",
			url: "/departments",
			params,
		})
		return data.map((dto) => Department.fromDTO(dto))
	}
}

export const departmentService = new DepartmentService()
