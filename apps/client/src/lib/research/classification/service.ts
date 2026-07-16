import { http } from "$lib/shared/http/client"
import type {
	ResearchDomain,
	ResearchField,
	ResearchKeyword,
	ResearchSubfield,
	ResearchTopic,
} from "./dtos"

class ClassificationService {
	public async domains(): Promise<ResearchDomain[]> {
		return http.request<ResearchDomain[]>({
			method: "GET",
			url: "/works-classification/domains",
		})
	}

	public async fields(domainId?: string): Promise<ResearchField[]> {
		return http.request<ResearchField[]>({
			method: "GET",
			url: "/works-classification/fields",
			params: domainId ? { domain_id: domainId } : undefined,
		})
	}

	public async subfields(fieldId?: string): Promise<ResearchSubfield[]> {
		return http.request<ResearchSubfield[]>({
			method: "GET",
			url: "/works-classification/subfields",
			params: fieldId ? { field_id: fieldId } : undefined,
		})
	}

	public async topics(subfieldId?: string): Promise<ResearchTopic[]> {
		return http.request<ResearchTopic[]>({
			method: "GET",
			url: "/works-classification/topics",
			params: subfieldId ? { subfield_id: subfieldId } : undefined,
		})
	}

	public async keywords(): Promise<ResearchKeyword[]> {
		return http.request<ResearchKeyword[]>({
			method: "GET",
			url: "/works-classification/keywords",
		})
	}
}

export const classificationService = new ClassificationService()
