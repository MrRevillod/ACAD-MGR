export class AcademicOptionValue {
	private constructor(private readonly value: string) {}

	public static readonly OPTIONS = ["teaching", "research"] as const

	public static readonly LABELS: Record<string, string> = {
		teaching: "Docencia",
		research: "Investigación",
	}

	static from(value?: string): AcademicOptionValue {
		if (typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid AcademicOptionValue:", value)
			return new AcademicOptionValue("")
		}

		return new AcademicOptionValue(value)
	}

	public get code(): string {
		return this.value
	}

	public toDisplay(): string {
		return AcademicOptionValue.LABELS[this.value] || "--"
	}
}

export type AcademicOption = (typeof AcademicOptionValue.OPTIONS)[number]
