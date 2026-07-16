import * as v from "valibot"

import { PlantaValue, type AcademicPlanta } from "$academics/value-objects/planta.value"

export interface AcademicCategoryDTO {
	id: string
	name: string
	planta: AcademicPlanta
}

export const createCategoryDTOSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
	planta: v.picklist(PlantaValue.PLANTA),
})

export type CreateCategoryDTO = v.InferInput<typeof createCategoryDTOSchema>
