import { http } from "$lib/shared/http/client"
import type { Career } from "./dtos"

export const careerService = {
	list(params?: { department_id?: string }): Promise<Career[]> {
		return http.request<Career[]>({ method: "GET", url: "/careers", params })
	},
}
