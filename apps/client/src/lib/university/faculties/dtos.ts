import * as v from "valibot"

export interface FacultyDTO {
	id: string
	name: string
}

export interface GetFacultiesParams {
	name?: string
}

export const createFacultySchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
})

export type CreateFacultyDto = v.InferInput<typeof createFacultySchema>

export const createFacultyDTOInitialInput = {
	name: "",
}
