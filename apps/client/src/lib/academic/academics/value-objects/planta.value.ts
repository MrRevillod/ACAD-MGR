export class PlantaValue {
	private constructor(private readonly value: string) {}

	public static readonly PLANTA = ["adjunta", "permanente"] as const

	public static readonly LABELS: Record<string, string> = {
		adjunta: "Adjunta",
		permanente: "Permanente",
	}

	static from(value?: string): PlantaValue {
		if (typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid PlantaValue:", value)
			return new PlantaValue("")
		}

		return new PlantaValue(value)
	}

	public get code(): string {
		return this.value
	}

	public toDisplay(): string {
		return PlantaValue.LABELS[this.value] || "--"
	}
}

export type AcademicPlanta = (typeof PlantaValue.PLANTA)[number]
