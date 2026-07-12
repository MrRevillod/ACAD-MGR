<script lang="ts">
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { createMutation, useQueryClient } from "@tanstack/svelte-query"
	import Dialog from "$lib/shared/components/ui/dialog.svelte"
	import Button from "$lib/shared/components/ui/button.svelte"
	import { academicService } from "$lib/academic/academics/service"
	import { countryItems } from "$lib/shared/countries"
	import { SEX_LABELS } from "$lib/academic/academics/enums"
	import { updateAcademicSchema } from "../dtos"
	import type { Academic } from "$lib/academic/academics/dtos"
	import type { UpdateAcademicDto } from "$lib/academic/academics/dtos"

	interface Props {
		academic: Academic
		open: boolean
		onClose: () => void
	}

	let { academic, open = $bindable(), onClose }: Props = $props()

	const form = createForm({ schema: updateAcademicSchema })

	$effect(() => {
		if (!open) return
		const matched = countryItems.find((i) => i.label.includes(academic.nationality))
		reset(form, {
			initialInput: {
				names: academic.names,
				paternalSurname: academic.paternalSurname,
				maternalSurname: academic.maternalSurname,
				email: academic.email,
				orcid: academic.orcid ?? null,
				sex: academic.sex,
				birthDate: academic.birthDate,
				city: academic.city,
				nationalityCode: matched?.value ?? "CL",
				jce: academic.jce,
				annualDiscountHours: academic.annualDiscountHours,
			},
		})
	})

	const queryClient = useQueryClient()

	const updateAcademic = createMutation(() => ({
		mutationFn: (output: UpdateAcademicDto) => academicService.update(academic.id, output),
		onSuccess: () => {
			void queryClient.invalidateQueries({ queryKey: ["academic", academic.id] })
			open = false
		},
	}))
</script>

<Dialog bind:open title="Editar académico" class="max-w-2xl">
	<Form of={form} onsubmit={(output) => updateAcademic.mutate(output)}>
		<div class="grid gap-4">
			<div class="grid grid-cols-2 gap-4">
				<Field of={form} path={["names"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Nombres</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["paternalSurname"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Apellido paterno</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["maternalSurname"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Apellido materno</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["email"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Email</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="email"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["orcid"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>ORCID</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								placeholder="0000-0000-0000-0000"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["sex"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Sexo</span
							>
							<select
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							>
								{#each Object.entries(SEX_LABELS) as [value, label] (value)}
									<option {value}>{label}</option>
								{/each}
							</select>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["birthDate"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Fecha de nacimiento</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="date"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["city"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Ciudad</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["nationalityCode"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Nacionalidad</span
							>
							<select
								{...field.props}
								value={field.input ?? ""}
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
				<Field of={form} path={["jce"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>JCE</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="number"
								step="any"
								min="0"
								max="1"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							{#if field.errors}
								<p class="text-xs text-red-600">{field.errors[0]}</p>
							{/if}
						</label>
					{/snippet}
				</Field>
				<Field of={form} path={["annualDiscountHours"]}>
					{#snippet children(field)}
						<label class="grid gap-1.5">
							<span class="text-xs font-medium tracking-wide uppercase text-corp-gray"
								>Horas descuento anual</span
							>
							<input
								{...field.props}
								value={field.input ?? ""}
								type="number"
								step="any"
								min="0"
								class="h-10 w-full rounded-lg border border-corp-gray/20 bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</label>
					{/snippet}
				</Field>
			</div>
			<div class="mt-2 flex justify-end gap-2">
				<Button variant="secondary" type="button" onclick={onClose}>Cancelar</Button>
				<Button type="submit" disabled={updateAcademic.isPending}>
					{updateAcademic.isPending ? "Guardando..." : "Guardar"}
				</Button>
			</div>
		</div>
	</Form>
</Dialog>
