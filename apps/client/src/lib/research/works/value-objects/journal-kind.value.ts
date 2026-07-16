export class JournalKindValue {
	private constructor(private readonly value: string) {}

	public static readonly KINDS = ["wos", "scopus"] as const

	public static readonly LABELS: Record<string, string> = {
		wos: "WoS",
		scopus: "Scopus",
	}

	static from(value?: string | null): JournalKindValue {
		if (!value || typeof value !== "string" || value.trim() === "") {
			console.warn("Invalid JournalKindValue:", value)
			return new JournalKindValue("")
		}
		return new JournalKindValue(value)
	}

	public get code(): string {
		return this.value
	}

	public toDisplay(): string {
		return JournalKindValue.LABELS[this.value] || "--"
	}
}

export type JournalKind = (typeof JournalKindValue.KINDS)[number]
