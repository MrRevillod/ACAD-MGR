export class AuthorshipPositionValue {
	private constructor(private readonly value: string) {}

	public static readonly POSITIONS = ["first", "middle", "last"] as const

	public static readonly LABELS: Record<string, string> = {
		first: "Primer autor",
		middle: "Co-autor",
		last: "Último autor",
	}

	static from(value?: string): AuthorshipPositionValue {
		if (!value || typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid AuthorshipPositionValue:", value)
			return new AuthorshipPositionValue("")
		}
		return new AuthorshipPositionValue(value)
	}

	public get code(): string {
		return this.value
	}

	public toDisplay(): string {
		return AuthorshipPositionValue.LABELS[this.value] || "--"
	}
}

export type AuthorshipPosition = (typeof AuthorshipPositionValue.POSITIONS)[number]
