import * as v from "valibot"

export interface AppConfigDTO {
	jceMax: number
}

const coerceNumber = (v: unknown) => (v === "" ? 0 : Number(v))

export const updateAppConfigSchema = v.object({
	jceMax: v.pipe(
		v.unknown(),
		v.transform(coerceNumber),
		v.number(),
		v.minValue(1, "El valor máximo de JCE debe ser mayor que 0"),
	),
})

export type UpdateAppConfigInput = v.InferInput<typeof updateAppConfigSchema>
