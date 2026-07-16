import type { AcademicCategoryDTO } from "./dtos"

import { PlantaValue } from "$academics/value-objects/planta.value"

export class AcademicCategory {
	constructor(
		public readonly id: string,
		public readonly name: string,
		public readonly planta: PlantaValue,
	) {}

	public static fromDTO(dto: AcademicCategoryDTO) {
		return new AcademicCategory(dto.id, dto.name, PlantaValue.from(dto.planta))
	}
}
