import * as v from "valibot"

// Academic View DTOs ------------------------------------------------

export interface AcademicDTO {
	id: string
	names: string
	paternalSurname: string
	maternalSurname: string
	email: string
	orcid: string | null
	sex: "H" | "M" | "O"
	birthDate: string
	joinedAt: string
	workPosition: string | null
	department: string
	career: string | null
	jce: number
	category: string
	planta: "adjunta" | "permanente"
	option: "teaching" | "research"
	acadCategoryHours: number | null
	annualDiscountHours: number
	nationality: string
	city: string
}

export interface PublicAcademicDTO {
	id: string
	names: string
	paternalSurname: string
	maternalSurname: string
	email: string
	orcid: string | null
	sex: "H" | "M" | "O"
	birthDate: string
	joinedAt: string
	department: string
	career: string | null
	nationality: string
	city: string
}

// Query Academic DTOs ------------------------------------------------

export interface GetAcademicsParams {
	search?: string
	department_id?: string
	career_id?: string
	category_id?: string
	planta?: "adjunta" | "permanente"
	option?: "teaching" | "research"
	sort?: AcademicSortField
}

export const ACADEMIC_SORT_FIELD = [
	"names",
	"paternal_surname",
	"maternal_surname",
	"joined_at",
	"birth_date",
] as const

export type AcademicSortField = (typeof ACADEMIC_SORT_FIELD)[number]

// Shared schemas ------------------------------------------------

const ORCID_REGEX = /^(https?:\/\/orcid\.org\/)?\d{4}-\d{4}-\d{4}-\d{3}[\dX]$/
const RUT_REGEX = /^\d{7,8}-[\dkK]$/

const normalizeDecimal = (v: unknown) => (typeof v === "string" ? v.replace(",", ".") : v)
const coerceNumber = (v: unknown) => (v === "" ? 0 : Number(v))
const textField = (msg: string) => v.pipe(v.string(), v.minLength(1, msg), v.maxLength(255, msg))

const jceSchema = v.optional(
	v.pipe(
		v.unknown(),
		v.transform(normalizeDecimal),
		v.transform(coerceNumber),
		v.number(),
		v.minValue(0, "La JCE debe estar entre 0.0 y 1.0"),
		v.maxValue(1, "La JCE debe estar entre 0.0 y 1.0"),
	),
)

const annualDiscountHoursSchema = v.optional(
	v.pipe(
		v.unknown(),
		v.transform(normalizeDecimal),
		v.transform(coerceNumber),
		v.number(),
		v.minValue(0, "Las horas de descuento anual no pueden ser negativas"),
	),
)

const requiredNumber = v.pipe(
	v.unknown(),
	v.transform(normalizeDecimal),
	v.transform(coerceNumber),
	v.number(),
)

// Create Academic DTOs ------------------------------------------------

export const createAcademicDTOSchema = v.object({
	rut: v.pipe(v.string(), v.regex(RUT_REGEX, "Formato: XXXXXXXX-X")),
	names: textField("Los nombres deben tener entre 1 y 255 caracteres"),
	paternalSurname: textField("El apellido paterno debe tener entre 1 y 255 caracteres"),
	maternalSurname: textField("El apellido materno debe tener entre 1 y 255 caracteres"),
	email: v.pipe(v.string(), v.email("El email debe ser válido")),
	orcid: v.optional(
		v.nullable(
			v.pipe(
				v.string(),
				v.regex(ORCID_REGEX, "El ORCID ID debe tener el formato XXXX-XXXX-XXXX-XXXX"),
			),
		),
	),
	sex: v.picklist(["H", "M", "O"], "Seleccione una opción válida"),
	birthDate: v.pipe(v.string(), v.nonEmpty("La fecha de nacimiento es obligatoria")),
	joinedAt: v.pipe(v.string(), v.nonEmpty("La fecha de ingreso es obligatoria")),
	workPositionId: v.pipe(v.string(), v.nonEmpty("Seleccione un cargo")),
	departmentId: v.pipe(v.string(), v.nonEmpty("Seleccione un departamento")),
	careerId: v.optional(v.nullable(v.string())),
	acadCategoryOptionsId: v.pipe(v.string(), v.nonEmpty("Seleccione una opción de categoría")),
	jce: v.pipe(
		requiredNumber,
		v.minValue(0, "La JCE debe estar entre 0.0 y 1.0"),
		v.maxValue(1, "La JCE debe estar entre 0.0 y 1.0"),
	),
	annualDiscountHours: v.pipe(
		requiredNumber,
		v.minValue(0, "Las horas de descuento anual no pueden ser negativas"),
	),
	nationalityCode: v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
	city: textField("La ciudad debe tener entre 1 y 255 caracteres"),
})

export type CreateAcademicDTOSchema = typeof createAcademicDTOSchema
export type CreateAcademicDTO = v.InferInput<typeof createAcademicDTOSchema>

export const createAcademicDTOInitialInput = {
	rut: "",
	names: "",
	paternalSurname: "",
	maternalSurname: "",
	email: "",
	orcid: null,
	sex: "",
	birthDate: "",
	joinedAt: "",
	workPositionId: "",
	departmentId: "",
	careerId: null,
	acadCategoryOptionsId: "",
	jce: 0,
	annualDiscountHours: 0,
	nationalityCode: "CL",
	city: "",
} as unknown as CreateAcademicDTO

// Update Academic DTOs ------------------------------------------------

export const updateAcademicDTOSchema = v.object({
	names: v.optional(textField("Los nombres deben tener entre 1 y 255 caracteres")),
	paternalSurname: v.optional(
		textField("El apellido paterno debe tener entre 1 y 255 caracteres"),
	),
	maternalSurname: v.optional(
		textField("El apellido materno debe tener entre 1 y 255 caracteres"),
	),
	email: v.optional(v.pipe(v.string(), v.email("El email debe ser válido"))),
	orcid: v.optional(
		v.nullable(
			v.pipe(
				v.string(),
				v.regex(ORCID_REGEX, "El ORCID ID debe tener el formato XXXX-XXXX-XXXX-XXXX"),
			),
		),
	),
	sex: v.optional(v.picklist(["H", "M", "O"], "Seleccione una opción válida")),
	birthDate: v.optional(v.string()),
	city: v.optional(textField("La ciudad debe tener entre 1 y 255 caracteres")),
	nationalityCode: v.optional(
		v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
	),
	jce: jceSchema,
	annualDiscountHours: annualDiscountHoursSchema,
})

export type UpdateAcademicDTOSchema = typeof updateAcademicDTOSchema
export type UpdateAcademicDTO = v.InferInput<typeof updateAcademicDTOSchema>

// Data Import DTOs ------------------------------------------------

export interface ImportResult {
	imported: number
	errors: ImportRowError[]
}

export interface ImportRowError {
	row: number
	reasons: string[]
}
