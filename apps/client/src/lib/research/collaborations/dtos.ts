export interface CollaborationWorkRefDTO {
	id: string
	title: string
	publicationYear: number | null
}

export interface CollaborationNodeDTO {
	id: string
	name: string
	department: string
	totalWorks: number
}

export interface CollaborationEdgeDTO {
	sourceId: string
	targetId: string
	weight: number
	works: CollaborationWorkRefDTO[]
}

export interface CollaborationGraphDTO {
	academicId: string
	nodes: CollaborationNodeDTO[]
	edges: CollaborationEdgeDTO[]
	recommendations: CollaborationRecommendationDTO[]
}

export interface CollaborationRecommendationDTO {
	academicId: string
	name: string
	names: string
	paternalSurname: string
	maternalSurname: string
	department: string
	totalWorks: number
	weight: number
	matches: RecommendationMatchDTO[]
}

export interface RecommendationMatchDTO {
	type: "topic" | "keyword"
	id: string
	name: string
	focusWorks: MatchWorkRefDTO[]
	candidateWorks: MatchWorkRefDTO[]
}

export interface MatchWorkRefDTO {
	workId: string
	title: string
	publicationYear: number | null
	score: number
}
