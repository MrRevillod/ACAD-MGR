import { countryCodeToName, codeToEmoji } from "$lib/shared/countries"

export class CountryValue {
	private readonly name?: string
	private readonly flag?: string

	private constructor(public readonly code: string | null) {
		if (!code) {
			this.name = "--"
			this.flag = "--"
			return
		}

		this.name = countryCodeToName[code]
		try {
			this.flag = codeToEmoji(code)
		} catch {
			this.flag = ""
		}
	}

	static from(value: string | null): CountryValue {
		return new CountryValue(value)
	}

	static format(value: string | null): string {
		const cv = CountryValue.from(value)
		if (!cv) return "--"

		return cv.toDisplay()
	}

	toDisplay(): string {
		if (!this.name || !this.flag) return "--"
		return `${this.flag} ${this.name}`
	}
}
