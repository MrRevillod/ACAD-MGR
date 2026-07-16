<script lang="ts">
	import { toast } from "svelte-sonner"
	import { Loader, RefreshCw, CircleAlert } from "@lucide/svelte"

	import { authStore } from "$auth/store.svelte"
	import { useSyncWorksMutation } from "$works/queries"

	interface Props {
		academicId: string
		orcid: string | null
	}

	let { academicId, orcid }: Props = $props()

	const mutation = useSyncWorksMutation()

	async function handleSync() {
		if (!orcid) return
		try {
			const result = await mutation.mutateAsync(academicId)
			if (result.errors.length === 0) {
				toast.success(
					`Sincronización completa: ${result.worksCreated} creadas, ${result.worksSkipped} ya existían`,
				)
			} else {
				toast.warning(
					`Sincronización parcial: ${result.worksCreated} creadas, ${result.errors.length} errores`,
				)
			}
		} catch {
			toast.error("Error al sincronizar publicaciones")
		}
	}
</script>

{#if authStore.isAuthenticated()}
	{#if !orcid}
		<button
			type="button"
			disabled
			title="El académico no tiene ORCID asociado"
			class="inline-flex h-8 cursor-not-allowed items-center gap-1.5 rounded-md border border-corp-gray/20 bg-white px-3 text-xs font-medium text-corp-gray/50 opacity-60"
		>
			<CircleAlert class="size-3.5" />
			Sin ORCID
		</button>
	{:else}
		<button
			type="button"
			onclick={handleSync}
			disabled={mutation.isPending}
			class="inline-flex h-8 items-center gap-1.5 rounded-md bg-corp-blue px-3 text-xs font-medium text-white shadow-sm transition-colors hover:bg-corp-blue/90 active:scale-[0.96] disabled:opacity-60"
		>
			{#if mutation.isPending}
				<Loader class="size-3.5 animate-spin" />
				Sincronizando...
			{:else}
				<RefreshCw class="size-3.5" />
				Sincronizar desde OpenAlex
			{/if}
		</button>
	{/if}
{/if}
