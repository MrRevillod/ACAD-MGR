import * as v from "valibot"

import { PlantaValue, type AcademicPlanta } from "$lib/academic/academics/value-objects/planta.value"

export interface AcademicCategory {
	id: string
	name: string
	planta: AcademicPlanta
}

export const createCategorySchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
	planta: v.picklist(PlantaValue.PLANTA),
})

export type CreateCategoryDto = v.InferInput<typeof createCategorySchema>
