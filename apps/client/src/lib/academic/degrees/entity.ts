import type { DegreeDTO } from "./dtos"
import { DegreeKindValue } from "./value-objects/kind.value"
import { CountryValue } from "$shared/value-objects/country.value"
import { DateValue } from "$shared/value-objects/date.value"

export class Degree {
	constructor(
		public id: string,
		public academicId: string,
		public name: string,
		public university: string,
		public obtainedAt: DateValue,
		public kind: DegreeKindValue,
		public country: CountryValue,
	) {}

	static fromDTO(dto: DegreeDTO): Degree {
		return new Degree(
			dto.id,
			dto.academicId,
			dto.name,
			dto.university,
			DateValue.from(dto.obtainedAt),
			DegreeKindValue.from(dto.kind),
			CountryValue.from(dto.countryCode),
		)
	}
}
