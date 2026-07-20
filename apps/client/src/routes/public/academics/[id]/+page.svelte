<script lang="ts">
	import * as v from "valibot"
	import { page } from "$app/state"
	import { useSearchParams } from "runed/kit"
	import { useQuery } from "$shared/http/tanstack"
	import { academicService } from "$academics/service"
	import { Loader, CircleAlert } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import WorksSection from "$works/components/works-section.svelte"
	import AcademicSidebar from "$academics/components/academic-sidebar.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import Button from "$shared/components/ui/button.svelte"

	const id = $derived(page.params.id ?? "")

	const yearParamsSchema = v.object({
		yearFrom: v.optional(v.fallback(v.string(), ""), ""),
		yearTo: v.optional(v.fallback(v.string(), ""), ""),
	})

	const yearParams = useSearchParams(yearParamsSchema, { debounce: 300, pushHistory: false })

	const academicQuery = useQuery(() => ({
		queryKey: ["public-academic", id],
		queryFn: () => academicService.getPublic(id),
		enabled: Boolean(id),
	}))

	const academic = $derived(academicQuery.data)

	let requestEditDialogOpen = $state(false)
	let isRequesting = $state(false)
	let requestSent = $state(false)

	async function handleRequestEdit() {
		isRequesting = true
		try {
			await academicService.requestProfileUpdate(id)
			requestSent = true
			toast.success("Enlace enviado a tu correo electrónico")
		} catch {
			toast.error("Error al solicitar la edición del perfil")
		} finally {
			isRequesting = false
		}
	}
</script>

<div class="h-full overflow-y-auto">
	{#if academicQuery.isPending}
		<div class="flex h-full items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if academicQuery.isError || !academic}
		<div class="flex h-full flex-col items-center justify-center text-center">
			<CircleAlert class="size-8 text-red-500" />
			<p class="mt-3 text-sm text-corp-gray">Académico no encontrado.</p>
		</div>
	{:else}
		<div class="mx-auto max-w-[1600px] px-4 py-8 sm:px-6 lg:px-8">
			<div class="grid grid-cols-1 gap-6 lg:grid-cols-[320px_1fr]">
				<AcademicSidebar
					{academic}
					readonly
					onRequestEdit={() => (requestEditDialogOpen = true)}
				/>
				<div class="min-h-0">
					<WorksSection
						{academic}
						readonly
						bind:yearFrom={yearParams.yearFrom}
						bind:yearTo={yearParams.yearTo}
					/>
				</div>
			</div>
		</div>
	{/if}

	<Dialog
		bind:open={requestEditDialogOpen}
		title="Solicitar edición de perfil"
		description="La edición del perfil está sujeta a la verificación mediante correo académico. Se enviará un enlace a tu correo con un formulario de edición."
	>
		{#if requestSent}
			<p class="mb-4 text-sm text-green-600">
				Enlace enviado correctamente. Revisá tu correo electrónico para acceder al
				formulario de edición.
			</p>
		{/if}
		<div class="flex justify-end gap-2">
			<Button variant="secondary" onclick={() => (requestEditDialogOpen = false)}>
				Cerrar
			</Button>
			{#if !requestSent}
				<Button variant="primary" disabled={isRequesting} onclick={handleRequestEdit}>
					{#if isRequesting}
						<Loader class="size-4 animate-spin" />
						Enviando...
					{:else}
						Enviar enlace
					{/if}
				</Button>
			{/if}
		</div>
	</Dialog>
</div>
