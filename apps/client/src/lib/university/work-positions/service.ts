import { http } from "$lib/shared/http/client"
import type { AcademicWorkPositionDTO, CreatePositionDto, GetPositionsParams } from "./dtos"
import { AcademicWorkPosition } from "./entity"

class PositionService {
	public async list(params?: GetPositionsParams): Promise<AcademicWorkPosition[]> {
		const data = await http.request<AcademicWorkPositionDTO[]>({
			method: "GET",
			url: "/work-positions",
			params,
		})
		return data.map((dto) => AcademicWorkPosition.fromDTO(dto))
	}

	public async create(data: CreatePositionDto): Promise<AcademicWorkPosition> {
		const dto = await http.request<AcademicWorkPositionDTO>({
			method: "POST",
			url: "/work-positions",
			data,
		})
		return AcademicWorkPosition.fromDTO(dto)
	}
}

export const positionService = new PositionService()
