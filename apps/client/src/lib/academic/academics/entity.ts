import type { AcademicDTO } from "./dtos"

import { SexValue } from "$shared/value-objects/sex.value"
import { DateValue } from "$shared/value-objects/date.value"
import { CLf64Value } from "$shared/value-objects/cl-f64.value"
import { PlantaValue } from "./value-objects/planta.value"
import { CountryValue } from "$shared/value-objects/country.value"
import { AcademicOptionValue } from "./value-objects/option.value"

export class Academic {
	constructor(
		public id: string,
		public names: string,
		public paternalSurname: string,
		public maternalSurname: string,
		public email: string,
		public orcid: string | null,
		public sex: SexValue,
		public birthDate: DateValue,
		public joinedAt: DateValue,
		public workPosition: string | null,
		public department: string,
		public career: string | null,
		public jce: CLf64Value,
		public category: string,
		public planta: PlantaValue,
		public option: AcademicOptionValue,
		public acadCategoryHours: number | null,
		public annualDiscountHours: number,
		public nationality: CountryValue,
		public city: string,
	) {}

	public static fromDTO(dto: AcademicDTO): Academic {
		const country = CountryValue.from(dto.nationality)
		const sex = SexValue.from(dto.sex)
		const birthDate = DateValue.from(dto.birthDate)
		const joinedAt = DateValue.from(dto.joinedAt)
		const jce = CLf64Value.from(dto.jce)
		const option = AcademicOptionValue.from(dto.option)
		const planta = PlantaValue.from(dto.planta)

		return new Academic(
			dto.id,
			dto.names,
			dto.paternalSurname,
			dto.maternalSurname,
			dto.email,
			dto.orcid,
			sex,
			birthDate,
			joinedAt,
			dto.workPosition,
			dto.department,
			dto.career,
			jce,
			dto.category,
			planta,
			option,
			dto.acadCategoryHours,
			dto.annualDiscountHours,
			country,
			dto.city,
		)
	}
}
