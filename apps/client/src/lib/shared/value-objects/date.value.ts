export class DateValue {
	private constructor(private readonly value?: string) {}

	public get iso(): string | null {
		return this.value ?? null
	}

	private parsed(): Date | null {
		if (!this.value) return null
		const d = new Date(this.value)
		return Number.isNaN(d.getTime()) ? null : d
	}

	static from(value?: string | null): DateValue {
		if (typeof value !== "string") {
			console.warn("Invalid DateValue:", value)
			return new DateValue(undefined)
		}

		return new DateValue(value)
	}

	static format(
		value?: string | null,
		timeStyle: Intl.DateTimeFormatOptions["timeStyle"] = "short",
	): string {
		const date = DateValue.from(value)
		if (!date) return "--"
		return date.toDisplay(timeStyle)
	}

	static formatDate(value?: string | null): string {
		const date = DateValue.from(value)
		if (!date) return "--"
		return date.toDisplayDate()
	}

	toDisplay(timeStyle: Intl.DateTimeFormatOptions["timeStyle"] = "short"): string {
		const d = this.parsed()
		if (!d) return "--"

		return new Intl.DateTimeFormat("es-CL", {
			dateStyle: "medium",
			timeStyle,
		}).format(d)
	}

	toDisplayDate(): string {
		const d = this.parsed()
		if (!d) return "--"

		const day = String(d.getDate()).padStart(2, "0")
		const month = String(d.getMonth() + 1).padStart(2, "0")
		return `${day}-${month}-${d.getFullYear()}`
	}
}
