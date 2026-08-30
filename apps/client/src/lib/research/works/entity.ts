import type {
	WorkDTO,
	WorkDetailDTO,
	SourceDTO,
	AuthorshipDTO,
	WorkTopicDTO,
	WorkKeywordDTO,
	WorkOverridesDTO,
} from "./dtos"

import { JournalKindValue } from "./value-objects/journal-kind.value"

export class Work {
	constructor(
		public id: string,
		public openalexId: string,
		public title: string,
		public abstractText: string | null,
		public doi: string | null,
		public publicationDate: string | null,
		public publicationYear: number | null,
		public ty: string,
		public lang: string,
		public isAccepted: boolean,
		public isPublished: boolean,
		public sourceId: string | null,
		public journalKind: JournalKindValue,
		public overrides: WorkOverridesDTO = {},
		public researchLineId: string | null = null,
		public researchLineName: string | null = null,
	) {}

	get overriddenFields(): string[] {
		const o = this.overrides
		const fields: string[] = []
		if (o.title != null) fields.push("title")
		if (o.abstractText != null) fields.push("abstractText")
		if (o.doi != null) fields.push("doi")
		if (o.publicationYear != null) fields.push("publicationYear")
		if (o.isAccepted != null) fields.push("isAccepted")
		if (o.isPublished != null) fields.push("isPublished")
		if (o.researchLineId != null) fields.push("researchLineId")
		if (o.journalKind != null) fields.push("journalKind")
		return fields
	}

	static fromDTO(dto: WorkDTO): Work {
		return new Work(
			dto.id,
			dto.openalexId,
			dto.title,
			dto.abstractText,
			dto.doi,
			dto.publicationDate,
			dto.publicationYear,
			dto.ty,
			dto.lang,
			dto.isAccepted,
			dto.isPublished,
			dto.sourceId,
			JournalKindValue.from(dto.journalKind),
			dto.overrides ?? {},
			dto.researchLineId ?? null,
			dto.researchLineName ?? null,
		)
	}

	isFieldOverridden(field: string): boolean {
		return this.overriddenFields.includes(field)
	}
}

export class WorkDetail extends Work {
	constructor(
		work: Work,
		public source: Source | null,
		public authorships: AuthorshipDTO[],
		public topics: WorkTopicDTO[],
		public keywords: WorkKeywordDTO[],
	) {
		super(
			work.id,
			work.openalexId,
			work.title,
			work.abstractText,
			work.doi,
			work.publicationDate,
			work.publicationYear,
			work.ty,
			work.lang,
			work.isAccepted,
			work.isPublished,
			work.sourceId,
			work.journalKind,
			work.overrides,
			work.researchLineId,
			work.researchLineName,
		)
	}

	static fromDTO(dto: WorkDetailDTO): WorkDetail {
		return new WorkDetail(
			Work.fromDTO(dto),
			Source.fromDTO(dto.source ?? null),
			dto.authorships ?? [],
			dto.topics ?? [],
			dto.keywords ?? [],
		)
	}
}

export class Source {
	constructor(
		public readonly id: string,
		public readonly openalexId: string,
		public readonly name: string,
		public readonly ty: string,
		public readonly issn: string | null,
		public readonly kind: JournalKindValue,
	) {}

	static fromDTO(dto: SourceDTO | null): Source | null {
		if (!dto) return null
		return new Source(
			dto.id,
			dto.openalexId,
			dto.name,
			dto.ty,
			dto.issn,
			JournalKindValue.from(dto.kind),
		)
	}
}
