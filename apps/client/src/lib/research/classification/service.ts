import { http } from "$lib/shared/http/client"
import type {
	ResearchDomain,
	ResearchField,
	ResearchKeyword,
	ResearchLineDTO,
	ResearchLinesDetailResponseDTO,
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

	public async researchLines(): Promise<ResearchLineDTO[]> {
		return http.request<ResearchLineDTO[]>({
			method: "GET",
			url: "/works-classification/research-lines",
		})
	}

	public async researchLineDetails(): Promise<ResearchLinesDetailResponseDTO> {
		return http.request<ResearchLinesDetailResponseDTO>({
			method: "GET",
			url: "/works-classification/research-lines/detail",
		})
	}

	public async updateMapping(subfieldOpenalexId: string, researchLineId: string): Promise<void> {
		await http.request<void>({
			method: "PUT",
			url: "/works-classification/research-line-mappings",
			data: { subfieldOpenalexId, researchLineId },
		})
	}

	public async deleteMapping(subfieldOpenalexId: string): Promise<void> {
		await http.request<void>({
			method: "DELETE",
			url: `/works-classification/research-line-mappings/${encodeURIComponent(subfieldOpenalexId)}`,
		})
	}
}

export const classificationService = new ClassificationService()
