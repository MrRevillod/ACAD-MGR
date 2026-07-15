import * as v from "valibot"

export const loginDTOSchema = v.object({
	email: v.pipe(
		v.string(),
		v.trim(),
		v.minLength(1, "El email es obligatorio."),
		v.maxLength(255, "El email no puede tener más de 255 caracteres."),
	),
	password: v.pipe(
		v.string(),
		v.minLength(1, "La contraseña es obligatoria."),
		v.maxLength(255, "La contraseña no puede tener más de 255 caracteres."),
	),
})

export type LoginDTOSchema = typeof loginDTOSchema
export type LoginDTO = v.InferInput<typeof loginDTOSchema>
