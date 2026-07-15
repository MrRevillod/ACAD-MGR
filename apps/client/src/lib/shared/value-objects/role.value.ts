export class RoleValue {
	private constructor(private readonly value: string) {}

	static from(value?: string): RoleValue | null {
		if (typeof value !== "string" || value.trim() === "") return null
		return new RoleValue(value.trim())
	}

	public get code(): string {
		return this.value
	}

	static format(value?: string): string {
		const grade = RoleValue.from(value)
		if (!grade) return "--"
		return grade.toDisplay()
	}

	static default(): RoleValue {
		return new RoleValue("student")
	}

	toDisplay(): string {
		const labels: Record<string, string> = {
			admin: "Administrador",
		}

		return labels[this.value] ?? "--"
	}
}
