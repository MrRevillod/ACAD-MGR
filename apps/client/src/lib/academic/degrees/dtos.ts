import * as v from "valibot"
import { DegreeKindValue } from "./value-objects/kind.value"

export interface DegreeDTO {
	id: string
	academicId: string
	name: string
	university: string
	obtainedAt: string
	kind: string
	countryCode: string
}

export const createDegreeSchema = v.object({
	academicId: v.pipe(v.string(), v.nonEmpty("El académico es obligatorio")),
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
	university: v.pipe(
		v.string(),
		v.minLength(1, "La universidad debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "La universidad debe tener entre 1 y 255 caracteres"),
	),
	obtainedAt: v.pipe(v.string(), v.nonEmpty("La fecha de obtención es obligatoria")),
	kind: v.picklist(DegreeKindValue.KINDS, "Seleccione un tipo de grado"),
	countryCode: v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
})

export type CreateDegreeDto = v.InferInput<typeof createDegreeSchema>

export const updateDegreeSchema = v.object({
	name: v.optional(
		v.pipe(
			v.string(),
			v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
			v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
		),
	),
	university: v.optional(
		v.pipe(
			v.string(),
			v.minLength(1, "La universidad debe tener entre 1 y 255 caracteres"),
			v.maxLength(255, "La universidad debe tener entre 1 y 255 caracteres"),
		),
	),
	obtainedAt: v.optional(v.string()),
	countryCode: v.optional(
		v.pipe(v.string(), v.length(2, "El código de país debe tener 2 caracteres")),
	),
})

export type UpdateDegreeDto = v.InferInput<typeof updateDegreeSchema>

export const createDegreeDTOInitialInput = {
	academicId: "",
	name: "",
	university: "",
	obtainedAt: "",
	kind: "base" as const,
	countryCode: "CL",
}
