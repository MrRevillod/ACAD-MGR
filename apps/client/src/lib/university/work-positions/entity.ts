import type { AcademicWorkPositionDTO } from "./dtos"

export class AcademicWorkPosition {
	constructor(
		public id: string,
		public name: string,
	) {}

	static fromDTO(dto: AcademicWorkPositionDTO): AcademicWorkPosition {
		return new AcademicWorkPosition(dto.id, dto.name)
	}
}
