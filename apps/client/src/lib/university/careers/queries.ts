import { createQuery } from "@tanstack/svelte-query"
import { careerService } from "./service"

export function useCareersQuery(getDepartmentId: () => string | undefined) {
	return createQuery(() => ({
		queryKey: ["university", "careers", getDepartmentId()],
		queryFn: () =>
			careerService.list(
				getDepartmentId() ? { department_id: getDepartmentId() } : undefined,
			),
		enabled: Boolean(getDepartmentId()),
	}))
}
