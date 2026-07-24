import type { GetWorksParams, SyncResult, WorkOverridesInput } from "./dtos"

import { worksService } from "./service"
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"
import { toast } from "svelte-sonner"

export function useWorksQuery(getParams: () => GetWorksParams) {
	return createQuery(() => ({
		queryKey: ["works", "list", getParams()],
		queryFn: () => worksService.list(getParams()),
		staleTime: 30_000,
		refetchOnWindowFocus: false,
	}))
}

export function useWorkDetailQuery(getId: () => string) {
	return createQuery(() => ({
		queryKey: ["works", "detail", getId()],
		queryFn: () => worksService.get(getId()),
		enabled: Boolean(getId()),
	}))
}

export function useWorksByAcademicQuery(
	getAcademicId: () => string,
	getParams: () => Omit<GetWorksParams, "academicId"> = () => ({}),
) {
	return createQuery(() => ({
		queryKey: ["works", "by-academic", getAcademicId(), getParams()],
		queryFn: () => worksService.listByAcademic(getAcademicId(), getParams()),
		enabled: Boolean(getAcademicId()),
	}))
}

export function useSyncWorksMutation() {
	const qc = useQueryClient()
	return createMutation<SyncResult, Error, string>(() => ({
		mutationFn: (academicId: string) => worksService.sync(academicId),
		onSuccess: () => {
			void qc.invalidateQueries({ queryKey: ["works"] })
		},
	}))
}

export function useSyncAllMutation() {
	return createMutation<void, Error, void>(() => ({
		mutationFn: () => worksService.syncAll(),
		onSuccess: () => {
			toast.success("Sincronización iniciada. Recibirás un correo cuando finalice.")
		},
		onError: () => {
			toast.error("Error al solicitar la sincronización")
		},
	}))
}

export function useUpdateOverridesMutation() {
	const qc = useQueryClient()
	return createMutation<void, Error, { id: string; data: WorkOverridesInput }>(() => ({
		mutationFn: ({ id, data }) => worksService.updateOverrides(id, data),
		onSuccess: () => {
			void qc.invalidateQueries({ queryKey: ["works"] })
			toast.success("Cambios guardados")
		},
		onError: () => {
			toast.error("Error al guardar cambios")
		},
	}))
}

export function useClearOverridesMutation() {
	const qc = useQueryClient()
	return createMutation<void, Error, string>(() => ({
		mutationFn: (id) => worksService.clearOverrides(id),
		onSuccess: () => {
			void qc.invalidateQueries({ queryKey: ["works"] })
			toast.success("Valores originales restaurados")
		},
		onError: () => {
			toast.error("Error al restaurar valores originales")
		},
	}))
}
