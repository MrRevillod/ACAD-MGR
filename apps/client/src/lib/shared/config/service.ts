import { http } from "$lib/shared/http/client"

import type { AppConfigDTO, UpdateAppConfigInput } from "./dtos"

class ConfigService {
	public get(): Promise<AppConfigDTO> {
		return http.request<AppConfigDTO>({
			method: "GET",
			url: "/config",
		})
	}

	public update(data: UpdateAppConfigInput): Promise<AppConfigDTO> {
		return http.request<AppConfigDTO>({
			method: "PATCH",
			url: "/config",
			data,
		})
	}
}

export const configService = new ConfigService()
