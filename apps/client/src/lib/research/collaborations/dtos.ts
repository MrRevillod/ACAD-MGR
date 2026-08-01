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
}
