export interface StatsQuery {
	journal_kind?: "wos" | "scopus"
	option?: "teaching" | "research"
	department_id?: string
	year_from?: number
	year_to?: number
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
	year_from?: number
	year_to?: number
	option?: "teaching" | "research"
	journal_kind?: "wos" | "scopus"
}
