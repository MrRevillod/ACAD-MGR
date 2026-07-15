export class CLf64Value {
	private constructor(private readonly value: number | null) {}

	public get number(): number {
		return this.value ?? 0
	}

	static from(value?: number | null): CLf64Value {
		if (typeof value !== "number" || !Number.isFinite(value)) {
			console.warn("Invalid CLf64Value:", value)
			return new CLf64Value(null)
		}

		return new CLf64Value(value)
	}

	static format(value?: number | null, opts?: { min?: number; max?: number }): string {
		const v = CLf64Value.from(value)
		return v.toDisplay(opts)
	}

	public toDisplay(opts?: { min?: number; max?: number }): string {
		if (this.value == null) return "--"
		return this.value.toLocaleString("es-CL", {
			minimumFractionDigits: opts?.min ?? 1,
			maximumFractionDigits: opts?.max ?? 2,
		})
	}
}
