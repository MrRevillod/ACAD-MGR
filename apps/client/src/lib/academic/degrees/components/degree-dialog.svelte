<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { degreeService } from "$lib/academic/degrees/service"
	import { countryItems } from "$lib/shared/countries"
	import { DEGREE_KIND_LABELS } from "$lib/academic/degrees/enums"
	import { createDegreeSchema } from "../dtos"
	import type { Degree } from "$lib/academic/degrees/dtos"
	import type { DegreeKind } from "$lib/academic/degrees/enums"

	interface Props {
		academicId: string
		degree?: Degree | null
		createKind?: DegreeKind
		open: boolean
		onClose: () => void
	}

	let {
		academicId,
		degree = null,
		createKind = "base",
		open = $bindable(),
		onClose,
	}: Props = $props()

	const form = createForm({ schema: createDegreeSchema })

	$effect(() => {
		if (!open) return
		if (degree) {
			reset(form, {
				initialInput: {
					academicId,
					name: degree.name,
					university: degree.university,
					obtainedAt: degree.obtainedAt,
					kind: degree.kind,
					countryCode: degree.countryCode,
				},
			})
		} else {
			reset(form, {
				initialInput: {
					academicId,
					name: "",
					university: "",
					obtainedAt: "",
					kind: createKind,
					countryCode: "CL",
				},
			})
		}
	})

	const queryClient = useQueryClient()

	const createDeg = createMutation(() => ({
		mutationFn: (output: {
			academicId: string
			name: string
			university: string
			obtainedAt: string
			kind: DegreeKind
			countryCode: string
		}) => degreeService.create(output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", academicId] })
			open = false
		},
	}))

	const updateDeg = createMutation(() => ({
		mutationFn: ({
			degId,
			data,
		}: {
			degId: string
			data: { name?: string; university?: string; obtainedAt?: string; countryCode?: string }
		}) => degreeService.update(degId, data),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["degrees", academicId] })
			open = false
		},
	}))

	function handleSubmit(output: {
		academicId: string
		name: string
		university: string
		obtainedAt: string
		kind: DegreeKind
		countryCode: string
	}) {
		if (degree) {
			updateDeg.mutate({
				degId: degree.id,
				data: {
					name: output.name,
					university: output.university,
					obtainedAt: output.obtainedAt,
					countryCode: output.countryCode,
				},
			})
		} else {
			createDeg.mutate(output)
		}
	}

	const pending = $derived(createDeg.isPending || updateDeg.isPending)
</script>

<Dialog bind:open title={degree ? "Editar grado" : "Nuevo grado"} class="max-w-xl">
	<Form of={form} onsubmit={(output) => handleSubmit(output)}>
		<div class="grid gap-4">
			<Field of={form} path={["name"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Nombre</span
						>
						<input
							{...field.props}
							value={field.input}
							placeholder="Ej: Magíster en Ciencias"
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<Field of={form} path={["university"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>Universidad</span
						>
						<input
							{...field.props}
							value={field.input}
							placeholder="Ej: Universidad Católica de Temuco"
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						/>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["obtainedAt"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Fecha</span
							>
							<input
								{...field.props}
								value={field.input}
								type="date"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["kind"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Tipo</span
							>
							<select
								{...field.props}
								value={field.input}
								disabled
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-gray-50 px-3 text-sm text-corp-gray outline-none"
							>
								<option value="base">{DEGREE_KIND_LABELS.base}</option>
								<option value="advanced">{DEGREE_KIND_LABELS.advanced}</option>
							</select>
						</label>
					{/snippet}
				</Field>
			</div>
			<Field of={form} path={["countryCode"]}>
				{#snippet children(field)}
					<label class="grid gap-1.5">
						<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
							>País</span
						>
						<select
							{...field.props}
							value={field.input}
							class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
						>
							<option value="">Seleccionar país...</option>
							{#each countryItems as c (c.value)}
								<option value={c.value}>{c.label}</option>
							{/each}
						</select>
						{#if field.errors}
							<p class="text-xs text-red-600">{field.errors[0]}</p>
						{/if}
					</label>
				{/snippet}
			</Field>
			<div class="mt-2 flex justify-end gap-2">
				<Button variant="secondary" type="button" onclick={onClose}>Cancelar</Button>
				<Button type="submit" disabled={pending}>
					{pending ? "Guardando..." : "Guardar"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
