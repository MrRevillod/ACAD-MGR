import { httpClient } from "$lib/shared/http/request"
import type {
	GetWorksParams,
	ResearchDomain,
	ResearchField,
	ResearchKeyword,
	ResearchSubfield,
	ResearchTopic,
	SyncResult,
	Work,
	WorkDetail,
} from "./types"

function serializeParams(params: GetWorksParams) {
	const out: Record<string, string | number | boolean> = {}
	for (const [k, v] of Object.entries(params)) {
		if (v == null) continue
		if (Array.isArray(v)) {
			if (v.length === 0) continue
			out[k] = v.join(",")
		} else {
			out[k] = v as string | number | boolean
		}
	}
	return out
}

export const worksService = {
	list(params: GetWorksParams = {}): Promise<Work[]> {
		return httpClient.request<Work[]>({
			method: "GET",
			url: "/works",
			params: serializeParams(params),
		})
	},

	get(id: string): Promise<WorkDetail> {
		return httpClient.request<WorkDetail>({
			method: "GET",
			url: `/works/${id}`,
		})
	},

	listByAcademic(
		academicId: string,
		params: Omit<GetWorksParams, "academicId"> = {},
	): Promise<Work[]> {
		return httpClient.request<Work[]>({
			method: "GET",
			url: `/works/academic/${academicId}`,
			params: serializeParams(params),
		})
	},

	sync(academicId: string): Promise<SyncResult> {
		return httpClient.request<SyncResult>({
			method: "POST",
			url: `/works/sync/${academicId}`,
		})
	},
}

export const classificationService = {
	domains(): Promise<ResearchDomain[]> {
		return httpClient.request<ResearchDomain[]>({
			method: "GET",
			url: "/works-classification/domains",
		})
	},

	fields(domainId?: string): Promise<ResearchField[]> {
		return httpClient.request<ResearchField[]>({
			method: "GET",
			url: "/works-classification/fields",
			params: domainId ? { domain_id: domainId } : undefined,
		})
	},

	subfields(fieldId?: string): Promise<ResearchSubfield[]> {
		return httpClient.request<ResearchSubfield[]>({
			method: "GET",
			url: "/works-classification/subfields",
			params: fieldId ? { field_id: fieldId } : undefined,
		})
	},

	topics(subfieldId?: string): Promise<ResearchTopic[]> {
		return httpClient.request<ResearchTopic[]>({
			method: "GET",
			url: "/works-classification/topics",
			params: subfieldId ? { subfield_id: subfieldId } : undefined,
		})
	},

	keywords(): Promise<ResearchKeyword[]> {
		return httpClient.request<ResearchKeyword[]>({
			method: "GET",
			url: "/works-classification/keywords",
		})
	},
}
