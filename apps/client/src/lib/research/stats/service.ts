import { http } from "$lib/shared/http/client"
import type {
	DepartmentDetail,
	DepartmentDetailQuery,
	StatsQuery,
	WorksStatsResponse,
} from "./types"

class StatsService {
	getWorksStats(params?: StatsQuery): Promise<WorksStatsResponse> {
		return http.request<WorksStatsResponse>({
			method: "GET",
			url: "/stats/works",
			params,
		})
	}

	getDepartmentDetail(
		id: string,
		params?: DepartmentDetailQuery,
	): Promise<DepartmentDetail> {
		return http.request<DepartmentDetail>({
			method: "GET",
			url: `/stats/department/${id}`,
			params,
		})
	}
}

export const statsService = new StatsService()
