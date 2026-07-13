import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query"

import { classificationService, worksService } from "./service"
import type { GetWorksParams, SyncResult } from "./types"
import { departmentService } from "$lib/university/departments/service"
import { careerService } from "$lib/university/careers/service"

export function useWorksQuery(getParams: () => GetWorksParams) {
	return createQuery(() => ({
		queryKey: ["works", "list", getParams()],
		queryFn: () => worksService.list(getParams()),
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

export function useDomainsQuery() {
	return createQuery(() => ({
		queryKey: ["classification", "domains"],
		queryFn: () => classificationService.domains(),
		staleTime: 5 * 60 * 1000,
	}))
}

export function useFieldsQuery(getDomainId: () => string | undefined) {
	return createQuery(() => ({
		queryKey: ["classification", "fields", getDomainId()],
		queryFn: () => classificationService.fields(getDomainId()),
		enabled: Boolean(getDomainId()),
	}))
}

export function useSubfieldsQuery(getFieldId: () => string | undefined) {
	return createQuery(() => ({
		queryKey: ["classification", "subfields", getFieldId()],
		queryFn: () => classificationService.subfields(getFieldId()),
		enabled: Boolean(getFieldId()),
	}))
}

export function useTopicsQuery(getSubfieldId: () => string | undefined) {
	return createQuery(() => ({
		queryKey: ["classification", "topics", getSubfieldId()],
		queryFn: () => classificationService.topics(getSubfieldId()),
		enabled: Boolean(getSubfieldId()),
	}))
}

export function useKeywordsQuery() {
	return createQuery(() => ({
		queryKey: ["classification", "keywords"],
		queryFn: () => classificationService.keywords(),
		staleTime: 5 * 60 * 1000,
	}))
}

export function useDepartmentsQuery() {
	return createQuery(() => ({
		queryKey: ["university", "departments"],
		queryFn: () => departmentService.list(),
		staleTime: 5 * 60 * 1000,
	}))
}

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
