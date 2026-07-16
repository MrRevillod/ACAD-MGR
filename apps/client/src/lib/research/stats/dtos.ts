export interface StatsQuery {
	journalKind?: "wos" | "scopus"
	option?: "teaching" | "research"
	departmentId?: string
	yearFrom?: number
	yearTo?: number
}

export interface YearValue {
	year: number
	value: number
}

export interface TimeSeriesStat {
	id?: string | null
	key: string
	values: YearValue[]
}

export interface WorksStatsResponse {
	byJournalKind: TimeSeriesStat[]
	byOption: TimeSeriesStat[]
	byDepartment: TimeSeriesStat[]
}

export interface TopPublisher {
	academicId: string
	name: string
	total: number
	scopus: number
	wos: number
	unindexed: number
	option: string
}

export interface DepartmentDetail {
	department: string
	totalWorks: number
	scopusCount: number
	wosCount: number
	teachingCount: number
	researchCount: number
	topPublishers: TopPublisher[]
}

export interface DepartmentDetailQuery {
	yearFrom?: number
	yearTo?: number
	option?: "teaching" | "research"
	journalKind?: "wos" | "scopus"
}
