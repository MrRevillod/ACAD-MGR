export const SEX = ["H", "M", "O"] as const
export type Sex = (typeof SEX)[number]

export const ACADEMIC_PLANTA = ["adjunta", "permanente"] as const
export type AcademicPlanta = (typeof ACADEMIC_PLANTA)[number]

export const ACADEMIC_OPTION = ["teaching", "research"] as const
export type AcademicOption = (typeof ACADEMIC_OPTION)[number]

export const DEGREE_KIND = ["base", "advanced"] as const
export type DegreeKind = (typeof DEGREE_KIND)[number]

export interface AcademicView {
	id: string
	names: string
	paternalSurname: string
	maternalSurname: string
	email: string
	orcid: string | null
	sex: Sex
	birthDate: string
	joinedAt: string
	workPosition: string
	workPositionDetails: string | null
	department: string
	career: string | null
	uctWorkingHours: number
	category: string
	planta: AcademicPlanta
	option: AcademicOption
	acadCategoryHours: number
	annualDiscountHours: number
	nationality: string
	city: string
}

export interface Degree {
	id: string
	academicId: string
	name: string
	university: string
	obtainedAt: string
	kind: DegreeKind
	countryCode: string
}
