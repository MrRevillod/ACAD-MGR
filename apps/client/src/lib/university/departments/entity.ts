import type { DepartmentDTO } from "./dtos"

export class Department {
	constructor(
		public id: string,
		public name: string,
		public facultyId: string,
	) {}

	static fromDTO(dto: DepartmentDTO): Department {
		return new Department(dto.id, dto.name, dto.facultyId)
	}
}
