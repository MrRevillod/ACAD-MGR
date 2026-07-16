<script lang="ts">
	import { goto } from "$app/navigation"

	import LoginForm from "$auth/components/login-form.svelte"

	async function handleSuccess() {
		await goto("/")
	}
</script>

<div class="flex h-full items-center justify-center px-4 py-8">
	<LoginForm onSuccess={handleSuccess} />
</div>
