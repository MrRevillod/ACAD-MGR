import type { FacultyDTO } from "./dtos"

export class Faculty {
	constructor(
		public id: string,
		public name: string,
	) {}

	static fromDTO(dto: FacultyDTO): Faculty {
		return new Faculty(dto.id, dto.name)
	}
}
