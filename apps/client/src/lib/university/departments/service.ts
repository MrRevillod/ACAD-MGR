import { http } from "$lib/shared/http/client"
import type { Department } from "./dtos"

export const departmentService = {
	list(): Promise<Department[]> {
		return http.request<Department[]>({ method: "GET", url: "/departments" })
	},
}
