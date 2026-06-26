import type { AcademicView, Degree } from "./types"

export const mockAcademic: AcademicView = {
	id: "e5f6a7b8-c9d0-1234-ef56-789012345678",
	names: "Rodrigo Antonio",
	paternalSurname: "Aguilar",
	maternalSurname: "Vera",
	email: "raguilar@uct.cl",
	orcid: "0000-0002-1234-5678",
	sex: "H",
	birthDate: "1976-01-15",
	joinedAt: "2024-03-04",
	workPosition: "Docente",
	department: "Ciencias Matemáticas y Físicas",
	career: "Ingeniería Civil en Informática",
	jce: 0.5,
	category: "Profesor Asistente",
	planta: "permanente",
	option: "teaching",
	acadCategoryHours: 14,
	annualDiscountHours: 14,
	nationality: "Chile",
	city: "Temuco",
}

export const mockDegrees: Degree[] = [
	{
		id: "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
		academicId: mockAcademic.id,
		name: "Ingeniero Constructor",
		university: "Pontificia Universidad Católica de Valparaíso",
		obtainedAt: "2005-04-13",
		kind: "base",
		countryCode: "CL",
	},
	{
		id: "b2c3d4e5-f6a7-8901-bcde-f12345678901",
		academicId: mockAcademic.id,
		name: "Doctor en Geografía",
		university: "Universidad Nacional Autónoma de México",
		obtainedAt: "2021-06-24",
		kind: "advanced",
		countryCode: "MX",
	},
]
