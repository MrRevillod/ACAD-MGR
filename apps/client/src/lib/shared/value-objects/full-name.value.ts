export class FullName {
	private constructor(
		public readonly names: string,
		public readonly paternalSurname: string,
		public readonly maternalSurname: string,
	) {}

	static of(names: string, paternalSurname: string, maternalSurname: string): FullName {
		return new FullName(names, paternalSurname, maternalSurname)
	}

	static fromFullString(fullName: string): string {
		return fullName
			.toLowerCase()
			.split(" ")
			.map((w) => w.charAt(0).toUpperCase() + w.slice(1))
			.join(" ")
	}

	private toPascalCase(str: string): string {
		return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase()
	}

	format(): string {
		const n = this.toPascalCase(this.names.split(" ")[0])
		const p = this.toPascalCase(this.paternalSurname)
		const m = this.toPascalCase(this.maternalSurname)
		return `${n} ${p} ${m}`
	}

	initials(): string {
		const n = this.names.charAt(0)
		const p = this.paternalSurname.charAt(0)
		return (n + p).toUpperCase()
	}
}
