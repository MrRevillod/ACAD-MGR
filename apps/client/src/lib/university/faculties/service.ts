import { http } from "$lib/shared/http/client"
import type { Faculty } from "./dtos"

export const facultyService = {
	list(): Promise<Faculty[]> {
		return http.request<Faculty[]>({ method: "GET", url: "/faculties" })
	},
}
