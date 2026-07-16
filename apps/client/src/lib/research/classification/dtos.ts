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
