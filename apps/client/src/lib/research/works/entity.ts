import type {
	WorkDTO,
	WorkDetailDTO,
	SourceDTO,
	AuthorshipDTO,
	WorkTopicDTO,
	WorkKeywordDTO,
} from "./dtos"

import { JournalKindValue } from "./value-objects/journal-kind.value"

export class Work {
	constructor(
		public id: string,
		public openalexId: string,
		public title: string,
		public abstract: string | null,
		public doi: string | null,
		public publicationDate: string | null,
		public publicationYear: number | null,
		public type: string,
		public lang: string,
		public isAccepted: boolean,
		public isPublished: boolean,
		public primarySourceId: string | null,
		public journalKind: JournalKindValue,
		public researchLineId?: string,
		public researchLineName?: string,
	) {}

	static fromDTO(dto: WorkDTO): Work {
		return new Work(
			dto.id,
			dto.openalexId,
			dto.title,
			dto.abstract,
			dto.doi,
			dto.publicationDate,
			dto.publicationYear,
			dto.type,
			dto.lang,
			dto.isAccepted,
			dto.isPublished,
			dto.primarySourceId,
			JournalKindValue.from(dto.journalKind),
			dto.researchLineId,
			dto.researchLineName,
		)
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
			work.abstract,
			work.doi,
			work.publicationDate,
			work.publicationYear,
			work.type,
			work.lang,
			work.isAccepted,
			work.isPublished,
			work.primarySourceId,
			work.journalKind,
			work.researchLineId,
			work.researchLineName,
		)
	}

	static fromDTO(dto: WorkDetailDTO): WorkDetail {
		return new WorkDetail(
			Work.fromDTO(dto),
			Source.fromDTO(dto.source),
			dto.authorships.map((a) => ({
				...a,
				position: a.position,
			})),
			dto.topics,
			dto.keywords,
		)
	}
}

export class Source {
	constructor(
		public readonly id: string,
		public readonly openalexId: string,
		public readonly displayName: string,
		public readonly ty: string,
		public readonly issnL: string | null,
		public readonly issn: string[] | null,
		public readonly kind: JournalKindValue,
	) {}

	static fromDTO(dto: SourceDTO | null): Source | null {
		if (!dto) return null
		return new Source(
			dto.id,
			dto.openalexId,
			dto.displayName,
			dto.ty,
			dto.issnL,
			dto.issn,
			JournalKindValue.from(dto.kind),
		)
	}
}
