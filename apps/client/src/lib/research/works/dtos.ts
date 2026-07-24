import type { JournalKind } from "./value-objects/journal-kind.value"
import type { AuthorshipPosition } from "./value-objects/position.value"

export interface WorkDTO {
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
	researchLineId?: string
	researchLineName?: string
	overriddenFields?: string[]
}

export const WORK_TYPE_LABELS: Record<string, string> = {
	"article": "Artículo",
	"book": "Libro",
	"book-chapter": "Capítulo de libro",
	"book-review": "Reseña de libro",
	"conference-abstract": "Abstract de conferencia",
	"conference-paper": "Paper de conferencia",
	"data-paper": "Paper de datos",
	"dissertation": "Tesis",
	"editorial": "Editorial",
	"erratum": "Errata",
	"letter": "Carta",
	"libguide": "Guía",
	"other": "Otro",
	"paratext": "Paratexto",
	"peer-review": "Revisión por pares",
	"preprint": "Preprint",
	"reference-entry": "Entrada de referencia",
	"report": "Reporte",
	"retraction": "Retracción",
	"review": "Revisión",
	"software": "Software",
	"software-paper": "Paper de software",
	"standard": "Estándar",
	"supplementary-materials": "Materiales suplementarios",
}

export interface SourceDTO {
	id: string
	openalexId: string
	displayName: string
	ty: string
	issnL: string | null
	issn: string[] | null
	kind: JournalKind | null
}

export interface AuthorshipDTO {
	orcid: string
	name: string
	isExternal: boolean
	isCorresponding: boolean
	affiliations: string[]
	position: AuthorshipPosition
	academicId?: string
}

export interface WorkTopicDTO {
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

export interface WorkKeywordDTO {
	keywordId: string
	name: string
	score: number
}

export interface WorkDetailDTO extends WorkDTO {
	source: SourceDTO | null
	authorships: AuthorshipDTO[]
	topics: WorkTopicDTO[]
	keywords: WorkKeywordDTO[]
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
	researchLineId?: string
}

export interface WorkOverridesInput {
	title?: string | null
	abstract?: string | null
	doi?: string | null
	publicationYear?: number | null
	isAccepted?: boolean | null
	isPublished?: boolean | null
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
