import { api } from "./api"
import { ApiResponse } from "./response"

import type { AxiosError, AxiosRequestConfig, AxiosResponse } from "axios"

export const request = <T>(config: AxiosRequestConfig): Promise<T> =>
	api
		.request<unknown, AxiosResponse<unknown>>(config)
		.then((response: AxiosResponse<unknown>) => {
			const payload = response.data

			if (!ApiResponse.is(payload)) {
				throw ApiResponse.genericError(response.status, "Respuesta del servidor inválida.")
			}

			if (!payload.success) {
				throw payload
			}

			return (payload as ApiResponse<T>).data as T
		})
		.catch((error: unknown) => {
			if (error instanceof ApiResponse) throw error

			const axiosError = error as AxiosError<unknown>

			if (axiosError.response?.data && ApiResponse.is(axiosError.response.data)) {
				throw axiosError.response.data
			}

			if (axiosError.response) {
				throw ApiResponse.genericError(axiosError.response.status, "Error del servidor.")
			}

			if (axiosError.code === "ECONNABORTED") {
				throw ApiResponse.genericError(0, "La solicitud tardó demasiado.")
			}

			throw ApiResponse.genericError(0, "Error de conexión.")
		})
