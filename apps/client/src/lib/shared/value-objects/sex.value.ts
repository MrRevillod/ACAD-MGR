export class SexValue {
	private constructor(private readonly value: string) {}

	public static readonly LABELS: Record<"H" | "M" | "O", string> = {
		H: "Masculino",
		M: "Femenino",
		O: "Otro",
	}

	static from(value?: string): SexValue {
		if (typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid SexValue:", value)
			return new SexValue("")
		}

		return new SexValue(value.trim())
	}

	static format(value?: string): string {
		const sex = SexValue.from(value)
		if (!sex) return "--"
		return sex.toDisplay()
	}

	public get code(): "H" | "M" | "O" {
		return this.value as "H" | "M" | "O"
	}

	toDisplay(): string {
		return SexValue.LABELS[this.value as "H" | "M" | "O"] || "--"
	}
}
