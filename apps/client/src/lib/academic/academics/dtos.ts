import * as v from "valibot"
import {
	SEX,
	type Sex,
	type AcademicPlanta,
	type AcademicOption,
	type AcademicSortField,
} from "./enums"

export interface Academic {
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

export interface GetAcademicsParams {
	search?: string
	department_id?: string
	career_id?: string
	category_id?: string
	planta?: AcademicPlanta
	option?: AcademicOption
	sort?: AcademicSortField
}

const ORCID_REGEX = /^(https?:\/\/orcid\.org\/)?\d{4}-\d{4}-\d{4}-\d{3}[\dX]$/
const RUT_REGEX = /^\d{7,8}-[\dkK]$/

const normalizeDecimal = (v: unknown) => (typeof v === "string" ? v.replace(",", ".") : v)

const coerceNumber = (v: unknown) => (v === "" ? 0 : Number(v))

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

const textField = (msg: string) => v.pipe(v.string(), v.minLength(1, msg), v.maxLength(255, msg))

export const updateAcademicSchema = v.object({
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
	sex: v.optional(v.picklist(SEX)),
	birthDate: v.optional(v.string()),
	city: v.optional(textField("La ciudad debe tener entre 1 y 255 caracteres")),
	nationalityCode: v.optional(
		v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
	),
	jce: jceSchema,
	annualDiscountHours: annualDiscountHoursSchema,
})

export type UpdateAcademicDto = v.InferInput<typeof updateAcademicSchema>

const requiredNumber = v.pipe(
	v.unknown(),
	v.transform(normalizeDecimal),
	v.transform(coerceNumber),
	v.number(),
)

export const createAcademicSchema = v.object({
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
	sex: v.picklist(SEX, "Seleccione una opción"),
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

export type CreateAcademicDto = v.InferInput<typeof createAcademicSchema>

export interface ImportResult {
	imported: number
	errors: ImportRowError[]
}

export interface ImportRowError {
	row: number
	reasons: string[]
}
