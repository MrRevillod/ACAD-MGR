import type { ProductivityDegree } from "./dtos"

export const degreeItems = [
	{ value: "all", label: "Total" },
	{ value: "doctor", label: "Doctores" },
	{ value: "magister", label: "Magísteres" },
]

export const monthNames = [
	"Enero",
	"Febrero",
	"Marzo",
	"Abril",
	"Mayo",
	"Junio",
	"Julio",
	"Agosto",
	"Septiembre",
	"Octubre",
	"Noviembre",
	"Diciembre",
]

export const monthItems = monthNames.map((label, i) => ({ value: String(i + 1), label }))

export const degreePhrases: Record<ProductivityDegree, string> = {
	all: "Publicaciones",
	doctor: "Publicaciones de autores con grado de doctor",
	magister: "Publicaciones de autores con grado de magíster",
}

export type ProductivityIndexation = "all" | "wos" | "scopus"

export const indexationItems = [
	{ value: "all", label: "Ambas" },
	{ value: "wos", label: "WoS" },
	{ value: "scopus", label: "Scopus" },
]
