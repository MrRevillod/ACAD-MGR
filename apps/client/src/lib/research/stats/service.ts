import { http } from "$lib/shared/http/client"
import type {
	AcademicStatsQuery,
	AcademicStatsResponse,
	DepartmentDetail,
	DepartmentDetailQuery,
	ProductivityQuery,
	ProductivityResponse,
	ResearchLineStatsQuery,
	ResearchLineStatsResponse,
	StatsQuery,
	WorksStatsResponse,
} from "$stats/dtos"

class StatsService {
	public getWorksStats(params?: StatsQuery): Promise<WorksStatsResponse> {
		return http.request<WorksStatsResponse>({
			method: "GET",
			url: "/stats/works",
			params,
		})
	}

	public getDepartmentDetail(
		id: string,
		params?: DepartmentDetailQuery,
	): Promise<DepartmentDetail> {
		return http.request<DepartmentDetail>({
			method: "GET",
			url: `/stats/department/${id}`,
			params,
		})
	}

	public getResearchLineStats(
		id: string,
		params?: ResearchLineStatsQuery,
	): Promise<ResearchLineStatsResponse> {
		return http.request<ResearchLineStatsResponse>({
			method: "GET",
			url: `/stats/research-line/${id}`,
			params,
		})
	}

	public getAcademicStats(
		id: string,
		params?: AcademicStatsQuery,
	): Promise<AcademicStatsResponse> {
		return http.request<AcademicStatsResponse>({
			method: "GET",
			url: `/stats/academic/${id}`,
			params,
		})
	}

	public getProductivity(params?: ProductivityQuery): Promise<ProductivityResponse> {
		return http.request<ProductivityResponse>({
			method: "GET",
			url: "/stats/productivity",
			params,
		})
	}
}

export const statsService = new StatsService()
