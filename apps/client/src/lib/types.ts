export const SEX = ["H", "M", "O"] as const
export type Sex = (typeof SEX)[number]

export const ACADEMIC_PLANTA = ["adjunta", "permanente"] as const
export type AcademicPlanta = (typeof ACADEMIC_PLANTA)[number]

export const ACADEMIC_OPTION = ["teaching", "research"] as const
export type AcademicOption = (typeof ACADEMIC_OPTION)[number]

export const DEGREE_KIND = ["base", "advanced"] as const
export type DegreeKind = (typeof DEGREE_KIND)[number]

export const ACADEMIC_SORT_FIELD = [
	"names",
	"paternal_surname",
	"maternal_surname",
	"joined_at",
	"birth_date",
] as const
export type AcademicSortField = (typeof ACADEMIC_SORT_FIELD)[number]

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
	workPosition: string | null
	department: string
	career: string | null
	jce: number
	category: string
	planta: AcademicPlanta
	option: AcademicOption
	acadCategoryHours: number | null
	annualDiscountHours: number
	nationality: string
	city: string
}

export interface AcademicCategory {
	id: string
	name: string
	planta: AcademicPlanta
}

export interface AcademicCategoryOption {
	id: string
	categoryId: string
	option: AcademicOption
	hours: number | null
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

export interface AcademicWorkPosition {
	id: string
	name: string
}

export interface ImportResult {
	imported: number
	errors: ImportRowError[]
}

export interface ImportRowError {
	row: number
	reasons: string[]
}
