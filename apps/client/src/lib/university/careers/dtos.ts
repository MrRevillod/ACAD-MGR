import * as v from "valibot"

export interface CareerDTO {
	id: string
	name: string
	departmentId: string
}

export interface GetCareersParams {
	department_id?: string
	name?: string
}

export const createCareerSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
	departmentId: v.pipe(v.string(), v.nonEmpty("Seleccione un departamento")),
})

export type CreateCareerDto = v.InferInput<typeof createCareerSchema>

export const createCareerDTOInitialInput = {
	name: "",
	departmentId: "",
}
