import { request } from "$lib/shared/http/request"
import type { AcademicWorkPosition } from "$lib/types"

export const positionsService = {
	list(params?: { name?: string }): Promise<AcademicWorkPosition[]> {
		return request<AcademicWorkPosition[]>({ method: "GET", url: "/work-positions", params })
	},

	create(data: { name: string }): Promise<AcademicWorkPosition> {
		return request<AcademicWorkPosition>({ method: "POST", url: "/work-positions", data })
	},
}
