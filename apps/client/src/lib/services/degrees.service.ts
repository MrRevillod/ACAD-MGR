import { request } from "$lib/shared/http/request"
import type { Degree, DegreeKind } from "$lib/types"

export const degreesService = {
	listByAcademic(academicId: string): Promise<Degree[]> {
		return request<Degree[]>({ method: "GET", url: `/degrees/${academicId}` })
	},

	create(data: {
		academicId: string
		name: string
		university: string
		obtainedAt: string
		kind: DegreeKind
		countryCode: string
	}): Promise<Degree> {
		return request<Degree>({ method: "POST", url: "/degrees", data })
	},
}
