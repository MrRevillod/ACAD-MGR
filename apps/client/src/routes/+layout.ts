import { authService } from "$lib/auth/auth.service"

export const ssr = false

export const load = async () => {
	await authService.bootstrapSession()
}
