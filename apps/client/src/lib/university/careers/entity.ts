import type { CareerDTO } from "./dtos"

export class Career {
	constructor(
		public id: string,
		public name: string,
		public departmentId: string,
	) {}

	static fromDTO(dto: CareerDTO): Career {
		return new Career(dto.id, dto.name, dto.departmentId)
	}
}
