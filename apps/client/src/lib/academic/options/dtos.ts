import * as v from "valibot"
import { AcademicOptionValue, type AcademicOption } from "$lib/academic/academics/value-objects/option.value"

export interface AcademicCategoryOption {
	id: string
	categoryId: string
	option: AcademicOption
	hours: number | null
}

const normalizeDecimal = (v: unknown) => (typeof v === "string" ? v.replace(",", ".") : v)

const hoursSchema = v.optional(
	v.pipe(
		v.unknown(),
		v.transform(normalizeDecimal),
		v.transform((v) => (v === "" || v === undefined || v === null ? null : Number(v))),
		v.nullable(v.pipe(v.number(), v.minValue(0, "No puede ser negativo"))),
	),
)

export const createOptionSchema = v.object({
	categoryId: v.pipe(v.string(), v.nonEmpty("Seleccione una categoría")),
	option: v.picklist(AcademicOptionValue.OPTIONS, "Seleccione una opción"),
	hours: hoursSchema,
})

export type CreateOptionDto = v.InferInput<typeof createOptionSchema>
