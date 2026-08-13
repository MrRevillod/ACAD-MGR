export class AuthorshipPositionValue {
	public static readonly POSITIONS = ["first", "middle", "last"] as const

	public static labelFor(displayIndex: number): string {
		return displayIndex === 0 ? "Autor" : "Coautor"
	}
}

export type AuthorshipPosition = (typeof AuthorshipPositionValue.POSITIONS)[number]
