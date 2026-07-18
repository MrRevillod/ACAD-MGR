import { http } from "$lib/shared/http/client"
import { Academic } from "./entity"

import type {
	AcademicDTO,
	PublicAcademicDTO,
	CreateAcademicDTO,
	GetAcademicsParams,
	ImportResult,
	UpdateAcademicDTO,
} from "./dtos"

class AcademicsService {
	public list(params?: GetAcademicsParams): Promise<Academic[]> {
		const academics = http.request<AcademicDTO[]>({
			method: "GET",
			url: "/academics",
			params,
		})

		return academics.then((data) => data.map((dto) => Academic.fromDTO(dto)))
	}

	public get(id: string): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "GET",
			url: `/academics/${id}`,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public getPublic(id: string): Promise<Academic> {
		const academic = http.request<PublicAcademicDTO>({
			method: "GET",
			url: `/academics/public/${id}`,
		})

		return academic.then((dto) => Academic.fromPublicDTO(dto))
	}

	public create(data: CreateAcademicDTO): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "POST",
			url: "/academics",
			data,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public update(id: string, data: UpdateAcademicDTO): Promise<Academic> {
		const academic = http.request<AcademicDTO>({
			method: "PATCH",
			url: `/academics/${id}`,
			data,
		})

		return academic.then((dto) => Academic.fromDTO(dto))
	}

	public import(file: File): Promise<ImportResult> {
		const formData = new FormData()
		formData.append("file", file)

		return http.request<ImportResult>({
			method: "POST",
			url: "/academics/import",
			data: formData,
			headers: {
				"Content-Type": "multipart/form-data",
			},
		})
	}
}

export const academicService = new AcademicsService()
