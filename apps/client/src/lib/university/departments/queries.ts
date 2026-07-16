import { createQuery } from "@tanstack/svelte-query"
import { departmentService } from "./service"

export function useDepartmentsQuery() {
	return createQuery(() => ({
		queryKey: ["university", "departments"],
		queryFn: () => departmentService.list(),
		staleTime: 5 * 60 * 1000,
		refetchOnWindowFocus: false,
	}))
}
