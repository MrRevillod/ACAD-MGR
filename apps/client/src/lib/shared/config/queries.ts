import { createQuery } from "@tanstack/svelte-query"

import { configService } from "./service"

export function useConfig() {
	return createQuery(() => ({
		queryKey: ["config"],
		queryFn: () => configService.get(),
		staleTime: 5 * 60 * 1000,
	}))
}
