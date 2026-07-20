<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { academicService } from "$academics/service"
	import { selfUpdateAcademicDTOSchema, type SelfUpdateDTO } from "$academics/dtos"
	import { createForm, reset } from "@formisch/svelte"
	import { Form, Field } from "@formisch/svelte"
	import { Loader, CircleAlert, CheckCircle, RefreshCw, Send } from "@lucide/svelte"
	import { toast } from "svelte-sonner"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import Select from "$shared/components/ui/form/select.svelte"
	import DatePicker from "$shared/components/ui/form/date-picker.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import { countryItems } from "$shared/countries"
	import { SexValue } from "$shared/value-objects/sex.value"

	const token = $derived(page.url.searchParams.get("token") ?? "")

	type ViewState = "loading" | "invalid" | "valid" | "success"

	let viewState: ViewState = $state("loading")
	let errorMsg = $state("")
	let academicData = $state<{
		id: string
		email: string
		names: string
		paternalSurname: string
		maternalSurname: string
		orcid: string | null
		sex: string
		birthDate: string | null
		city: string
		nationalityCode: string | null
	} | null>(null)
	let isSubmitting = $state(false)
	let isSyncing = $state(false)

	const form = createForm({ schema: selfUpdateAcademicDTOSchema })

	async function load() {
		if (!token) {
			viewState = "invalid"
			errorMsg = "Enlace inválido: no se encontró el token."
			return
		}

		try {
			const academic = await academicService.validateOneTimeToken(token)
			academicData = {
				id: academic.id,
				email: academic.email,
				names: academic.names,
				paternalSurname: academic.paternalSurname,
				maternalSurname: academic.maternalSurname,
				orcid: academic.orcid ?? null,
				sex: academic.sex.code,
				birthDate: academic.birthDate.iso,
				city: academic.city,
				nationalityCode: academic.nationality.code,
			}
			reset(form, {
				initialInput: {
					names: academic.names,
					paternalSurname: academic.paternalSurname,
					maternalSurname: academic.maternalSurname,
					orcid: academic.orcid ?? null,
					sex: academic.sex.code,
					birthDate: academic.birthDate.iso ?? "",
					city: academic.city,
					nationalityCode: academic.nationality.code ?? "CL",
				},
			})
			viewState = "valid"
		} catch {
			viewState = "invalid"
			errorMsg =
				"El enlace de actualización ha expirado o es inválido. Solicita un nuevo enlace desde tu perfil académico."
		}
	}

	$effect(() => {
		void load()
	})

	const countryOptions = $derived(countryItems)
	const sexOptions = $derived(
		Object.entries(SexValue.LABELS).map(([value, label]) => ({ label, value })),
	)

	async function handleSync() {
		if (!token) return
		isSyncing = true
		try {
			const result = await academicService.syncWorksByToken(token)
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
		} finally {
			isSyncing = false
		}
	}

	async function handleFormSubmit(data: SelfUpdateDTO) {
		if (!token) return
		isSubmitting = true
		try {
			await academicService.updateByToken(token, data)
			viewState = "success"
		} catch (e) {
			const msg = e instanceof Error ? e.message : "Error al actualizar el perfil"
			toast.error(msg)
		} finally {
			isSubmitting = false
		}
	}
</script>

<div class="mx-auto flex min-h-dvh max-w-2xl flex-col px-4 py-12">
	{#if viewState === "loading"}
		<div class="flex flex-1 items-center justify-center">
			<Loader class="size-6 animate-spin text-corp-gray" />
		</div>
	{:else if viewState === "invalid"}
		<div class="flex flex-1 flex-col items-center justify-center text-center">
			<CircleAlert class="size-10 text-red-500" />
			<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Enlace inválido o expirado</h1>
			<p class="mt-2 text-sm text-corp-gray">{errorMsg}</p>
		</div>
	{:else if viewState === "success"}
		<div class="flex flex-1 flex-col items-center justify-center text-center">
			<CheckCircle class="size-10 text-green-500" />
			<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Perfil actualizado</h1>
			<p class="mt-2 text-sm text-corp-gray">Tus datos se han actualizado correctamente.</p>
			<Button
				variant="primary"
				class="mt-6"
				onclick={() => goto(`/public/academics/${academicData?.id ?? ""}`)}
			>
				Ver mi perfil
			</Button>
		</div>
	{:else if viewState === "valid" && academicData}
		{@const acad = academicData}
		<div class="mb-6 rounded-xl border border-corp-gray/20 bg-white p-6">
			<div class="flex items-start justify-between gap-4">
				<div>
					<h1 class="text-lg font-semibold text-[#1A1A1A]">
						Actualizar perfil académico
					</h1>
				</div>
				<Button variant="secondary" size="sm" disabled={isSyncing} onclick={handleSync}>
					{#if isSyncing}
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
			<Form of={form} onsubmit={(output) => handleFormSubmit(output as SelfUpdateDTO)}>
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
					<Button variant="primary" type="submit" disabled={isSubmitting}>
						{#if isSubmitting}
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
