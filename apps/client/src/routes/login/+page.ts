import { redirect } from "@sveltejs/kit"
import { authStore } from "$lib/auth/store.svelte"

export const load = async () => {
	if (authStore.isAuthenticated()) {
		throw redirect(302, "/")
	}
}
