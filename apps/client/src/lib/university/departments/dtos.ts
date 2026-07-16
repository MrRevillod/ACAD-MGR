import * as v from "valibot"

export interface DepartmentDTO {
	id: string
	name: string
	facultyId: string
}

export interface GetDepartmentsParams {
	name?: string
	faculty_id?: string
}

export const createDepartmentSchema = v.object({
	name: v.pipe(
		v.string(),
		v.minLength(1, "El nombre debe tener entre 1 y 255 caracteres"),
		v.maxLength(255, "El nombre debe tener entre 1 y 255 caracteres"),
	),
	facultyId: v.pipe(v.string(), v.nonEmpty("Seleccione una facultad")),
})

export type CreateDepartmentDto = v.InferInput<typeof createDepartmentSchema>

export const createDepartmentDTOInitialInput = {
	name: "",
	facultyId: "",
}
