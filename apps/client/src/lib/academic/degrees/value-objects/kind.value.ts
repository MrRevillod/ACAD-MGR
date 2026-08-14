export class DegreeKindValue {
	private constructor(private readonly value: string) {}

	public static readonly KINDS = ["professional", "magister", "doctor"] as const

	public static readonly LABELS: Record<string, string> = {
		professional: "Profesional",
		magister: "Magíster",
		doctor: "Doctor",
	}

	static from(value?: string): DegreeKindValue {
		if (typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid DegreeKindValue:", value)
			return new DegreeKindValue("")
		}
		return new DegreeKindValue(value)
	}

	public get code(): string {
		return this.value
	}

	public toDisplay(): string {
		return DegreeKindValue.LABELS[this.value] || "--"
	}
}

export type DegreeKind = (typeof DegreeKindValue.KINDS)[number]
