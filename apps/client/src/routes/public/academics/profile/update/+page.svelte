<script lang="ts">
	import type { SelfUpdateDTO } from "$academics/dtos"

	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"
	import { Form, Field } from "@formisch/svelte"
	import { createForm, reset } from "@formisch/svelte"
	import { useMutation, useQuery } from "$shared/http/tanstack"
	import { Loader, CircleAlert, CircleCheck, RefreshCw, Send } from "@lucide/svelte"

	import { SexValue } from "$shared/value-objects/sex.value"
	import { countryItems } from "$shared/countries"
	import { academicService } from "$academics/service"
	import { selfUpdateAcademicDTOSchema } from "$academics/dtos"

	import Button from "$shared/components/ui/button.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import DatePicker from "$shared/components/ui/form/date-picker.svelte"

	const token = $derived(page.url.searchParams.get("token") ?? "")

	const form = createForm({ schema: selfUpdateAcademicDTOSchema })

	const tokenQuery = useQuery(() => ({
		queryKey: ["update", "academic", token],
		queryFn: () => academicService.validateOneTimeToken(token),
		enabled: Boolean(token),
	}))

	$effect(() => {
		const d = tokenQuery.data
		if (!d) return

		reset(form, {
			initialInput: {
				names: d.names,
				paternalSurname: d.paternalSurname,
				maternalSurname: d.maternalSurname,
				orcid: d.orcid ?? null,
				sex: d.sex,
				birthDate: d.birthDate ?? "",
				city: d.city,
				nationalityCode: d.nationality ?? "CL",
			},
		})
	})

	const countryOptions = $derived(countryItems)
	const sexOptions = $derived(
		Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	)

	const updateMutation = useMutation(() => ({
		mutationFn: (data: SelfUpdateDTO) => academicService.updateByToken(token, data),
		onError: (error) => toast.error(error.message ?? "Error al actualizar el perfil"),
	}))

	const syncMutation = useMutation(() => ({
		mutationFn: () => academicService.syncWorksByToken(token),
		onSuccess: (result) => {
			if (result.errors.length === 0) {
				toast.success(
					`Sincronización completa: ${result.worksCreated} creadas, ${result.worksSkipped} ya existían`,
				)
			} else {
				toast.warning(
					`Sincronización parcial: ${result.worksCreated} creadas, ${result.errors.length} errores`,
				)
			}
		},
		onError: () => toast.error("Error al sincronizar publicaciones"),
	}))
</script>

<div class="mx-auto flex min-h-dvh max-w-2xl flex-col px-4 py-12">
	{#if !token}
		<div class="flex flex-1 flex-col items-center justify-center text-center">
			<CircleAlert class="size-10 text-red-500" />
			<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Enlace inválido o expirado</h1>
			<p class="mt-2 text-sm text-corp-gray">Enlace inválido: no se encontró el token.</p>
		</div>
	{:else if tokenQuery.isPending}
		<div class="flex flex-1 items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if tokenQuery.isError}
		<div class="flex flex-1 flex-col items-center justify-center text-center">
			<CircleAlert class="size-10 text-red-500" />
			<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Enlace inválido o expirado</h1>
			<p class="mt-2 text-sm text-corp-gray">
				El enlace de actualización ha expirado o es inválido. Solicita un nuevo enlace desde
				tu perfil académico.
			</p>
		</div>
	{:else if updateMutation.isSuccess}
		<div class="flex flex-1 flex-col items-center justify-center text-center">
			<CircleCheck class="size-10 text-green-500" />
			<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Perfil actualizado</h1>
			<p class="mt-2 text-sm text-corp-gray">Tus datos se han actualizado correctamente.</p>
			<Button
				variant="primary"
				class="mt-6"
				onclick={() => goto(`/public/academics/${tokenQuery.data?.id ?? ""}`)}
			>
				Ver mi perfil
			</Button>
		</div>
	{:else}
		{@const acad = tokenQuery.data!}
		<div class="mb-6 rounded-xl border border-corp-gray/20 bg-white p-6">
			<div class="flex items-start justify-between gap-4">
				<div>
					<h1 class="text-lg font-semibold text-[#1A1A1A]">
						Actualizar perfil académico
					</h1>
				</div>
				<Button
					variant="secondary"
					size="sm"
					disabled={syncMutation.isPending}
					onclick={() => syncMutation.mutate()}
				>
					{#if syncMutation.isPending}
						<Loader class="size-3.5 animate-spin" />
						Sincronizando...
					{:else}
						<RefreshCw class="size-3.5" />
						Sincronizar publicaciones
					{/if}
				</Button>
			</div>
		</div>

		<div class="rounded-xl border border-corp-gray/20 bg-white p-6">
			<Form of={form} onsubmit={(output) => updateMutation.mutate(output as SelfUpdateDTO)}>
				<div class="grid grid-cols-2 gap-x-6 gap-y-5">
					<Field of={form} path={["names"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="Nombres"
							/>
						{/snippet}
					</Field>

					<Field of={form} path={["paternalSurname"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="Apellido paterno"
							/>
						{/snippet}
					</Field>

					<Field of={form} path={["maternalSurname"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="Apellido materno"
							/>
						{/snippet}
					</Field>

					<div>
						<p
							class="mb-1.5 text-xs font-medium tracking-wide uppercase text-corp-gray"
						>
							Correo electrónico
						</p>
						<p
							class="h-10 rounded-lg border border-corp-gray/20 bg-corp-gray/5 px-3 text-sm leading-10 text-corp-gray/60"
						>
							{acad.email}
						</p>
					</div>

					<Field of={form} path={["orcid"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input ?? ""}
								errors={field.errors}
								type="text"
								label="ORCID"
								placeholder="https://orcid.org/0000-0000-0000-0000"
							/>
						{/snippet}
					</Field>

					<Field of={form} path={["sex"]}>
						{#snippet children(field)}
							<Select
								{...field.props}
								input={field.input}
								errors={field.errors}
								label="Sexo"
								options={sexOptions}
							/>
						{/snippet}
					</Field>

					<Field of={form} path={["birthDate"]}>
						{#snippet children(field)}
							<DatePicker
								{...field.props}
								input={field.input}
								errors={field.errors}
								label="Fecha de nacimiento"
							/>
						{/snippet}
					</Field>

					<Field of={form} path={["nationalityCode"]}>
						{#snippet children(field)}
							<Select
								{...field.props}
								input={field.input}
								errors={field.errors}
								label="Nacionalidad"
								options={countryOptions}
							/>
						{/snippet}
					</Field>

					<div class="col-span-2">
						<Field of={form} path={["city"]}>
							{#snippet children(field)}
								<TextInput
									{...field.props}
									input={field.input}
									errors={field.errors}
									type="text"
									label="Ciudad"
								/>
							{/snippet}
						</Field>
					</div>
				</div>

				<div class="mt-8 flex justify-end gap-3 border-t border-corp-gray/20 pt-5">
					<Button
						variant="secondary"
						onclick={() => goto(`/public/academics/${acad.id}`)}
					>
						Cancelar
					</Button>
					<Button variant="primary" type="submit" disabled={updateMutation.isPending}>
						{#if updateMutation.isPending}
							<Loader class="size-4 animate-spin" />
							Guardando...
						{:else}
							<Send class="size-4" />
							Guardar cambios
						{/if}
					</Button>
				</div>
			</Form>
		</div>
	{/if}
</div>
