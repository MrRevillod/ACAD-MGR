export type AuthorshipPosition = "first" | "middle" | "last"

export const POSITION_LABELS: Record<AuthorshipPosition, string> = {
	first: "Primer autor",
	middle: "Co-autor",
	last: "Último autor",
}

export interface Work {
	id: string
	openalexId: string
	title: string
	abstract: string | null
	doi: string | null
	publicationDate: string | null
	publicationYear: number | null
	type: string
	lang: string
	isAccepted: boolean
	isPublished: boolean
	primarySourceId: string | null
	journalKind: string | null
}

export type JournalKind = "wos" | "scopus"

export const JOURNAL_KIND_LABELS: Record<JournalKind, string> = {
	wos: "WoS",
	scopus: "Scopus",
}

export const WORK_TYPE_LABELS: Record<string, string> = {
	article: "Artículo",
	book: "Libro",
	"book-chapter": "Capítulo de libro",
	"book-review": "Reseña de libro",
	"conference-abstract": "Abstract de conferencia",
	"conference-paper": "Paper de conferencia",
	"data-paper": "Paper de datos",
	dissertation: "Tesis",
	editorial: "Editorial",
	erratum: "Errata",
	letter: "Carta",
	libguide: "Guía",
	other: "Otro",
	paratext: "Paratexto",
	"peer-review": "Revisión por pares",
	preprint: "Preprint",
	"reference-entry": "Entrada de referencia",
	report: "Reporte",
	retraction: "Retracción",
	review: "Revisión",
	software: "Software",
	"software-paper": "Paper de software",
	standard: "Estándar",
	"supplementary-materials": "Materiales suplementarios",
}

export interface Source {
	id: string
	openalexId: string
	displayName: string
	ty: string
	issnL: string | null
	issn: string[] | null
	kind: JournalKind | null
}

export interface Authorship {
	orcid: string
	name: string
	isExternal: boolean
	isCorresponding: boolean
	affiliations: string[]
	position: AuthorshipPosition
}

export interface WorkTopic {
	topicId: string
	name: string
	score: number
	subfieldId: string
	subfieldName: string
	fieldId: string
	fieldName: string
	domainId: string
	domainName: string
}

export interface WorkKeyword {
	keywordId: string
	name: string
	score: number
}

export interface WorkDetail extends Work {
	source: Source | null
	authorships: Authorship[]
	topics: WorkTopic[]
	keywords: WorkKeyword[]
}

export interface GetWorksParams {
	academicId?: string
	search?: string
	yearFrom?: number
	yearTo?: number
	isAccepted?: boolean
	isPublished?: boolean
	departmentId?: string
	careerId?: string
	size?: number
	journalKind?: JournalKind
}

export interface SyncResult {
	academicId: string
	academicOrcid: string
	worksFetched: number
	worksCreated: number
	worksSkipped: number
	authorshipsInserted: number
	topicsLinked: number
	keywordsLinked: number
	errors: string[]
}

export interface ResearchDomain {
	id: string
	openalexId: string
	name: string
}

export interface ResearchField {
	id: string
	openalexId: string
	name: string
	domainId: string
}

export interface ResearchSubfield {
	id: string
	openalexId: string
	name: string
	fieldId: string
}

export interface ResearchTopic {
	id: string
	openalexId: string
	name: string
	subfieldId: string
}

export interface ResearchKeyword {
	id: string
	openalexId: string
	name: string
}
