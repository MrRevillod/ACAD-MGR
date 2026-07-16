import { http } from "$lib/shared/http/client"
import type { CareerDTO, GetCareersParams } from "./dtos"
import { Career } from "./entity"

class CareerService {
	public async list(params?: GetCareersParams): Promise<Career[]> {
		const data = await http.request<CareerDTO[]>({
			method: "GET",
			url: "/careers",
			params,
		})
		return data.map((dto) => Career.fromDTO(dto))
	}
}

export const careerService = new CareerService()
