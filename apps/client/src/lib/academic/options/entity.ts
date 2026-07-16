import type { AcademicCategoryOptionDTO } from "./dtos"
import { AcademicOptionValue } from "$options/value-objects/option.value"

export class AcademicCategoryOption {
	constructor(
		public id: string,
		public categoryId: string,
		public option: AcademicOptionValue,
		public hours: number | null,
	) {}

	static fromDTO(dto: AcademicCategoryOptionDTO): AcademicCategoryOption {
		return new AcademicCategoryOption(
			dto.id,
			dto.categoryId,
			AcademicOptionValue.from(dto.option),
			dto.hours,
		)
	}
}
