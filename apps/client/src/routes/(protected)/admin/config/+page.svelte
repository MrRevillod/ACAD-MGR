<script lang="ts">
	import type { UpdateAppConfigInput } from "$shared/config/dtos"

	import { toast } from "svelte-sonner"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { useMutation, queryClient } from "$shared/http/tanstack"
	import { updateAppConfigSchema } from "$shared/config/dtos"
	import { configService } from "$shared/config/service"
	import { useConfig } from "$shared/config/queries"

	import { Loader, Settings, Save } from "@lucide/svelte"
	import Button from "$shared/components/ui/button.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"

	const configQuery = useConfig()
	const jceMax = $derived(configQuery.data?.jceMax ?? 42.5)
	const form = $derived.by(() => createForm({ schema: updateAppConfigSchema }))

	const updateConfig = useMutation(() => ({
		mutationFn: (output: UpdateAppConfigInput) => configService.update(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["config"] })
			toast.success("Configuración actualizada")
		},
		onError: () => toast.error("Error al actualizar la configuración"),
	}))

	let initialised = $state(false)

	$effect(() => {
		if (configQuery.isPending || initialised) return
		initialised = true
		reset(form, { initialInput: { jceMax } })
	})
</script>

<svelte:head>
	<title>Configuración | Administración</title>
</svelte:head>

<div>
	<div class="mb-6">
		<h1 class="text-lg font-semibold text-[#1A1A1A]">Configuración del sistema</h1>
		<p class="mt-1 text-sm text-corp-gray">
			Parámetros globales que afectan a toda la aplicación.
		</p>
	</div>

	{#if configQuery.isPending}
		<div class="flex items-center justify-center py-16">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else}
		<div class="max-w-xl rounded-xl border border-corp-gray/20 bg-white p-6">
			<div
				class="mb-5 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
			>
				<Settings class="size-4 text-corp-blue" />
				Jornada Completa Equivalente (JCE)
			</div>

			<Form of={form} onsubmit={(output) => updateConfig.mutate(output)}>
				<Field of={form} path={["jceMax"]}>
					{#snippet children(field)}
						<TextInput
							{...field.props}
							input={field.input ?? ""}
							errors={field.errors}
							type="number"
							label="JCE máxima"
							hint="Horas de la jornada completa equivalente (máximo permitido). Cambiar este valor no re-escala los registros existentes."
						/>
					{/snippet}
				</Field>

				<div class="mt-6 flex justify-end">
					<Button type="submit" disabled={updateConfig.isPending}>
						{#if !updateConfig.isPending}<Save class="size-4" />{/if}
						{updateConfig.isPending ? "Guardando..." : "Guardar"}
					</Button>
				</div>
			</Form>
		</div>
	{/if}
</div>
