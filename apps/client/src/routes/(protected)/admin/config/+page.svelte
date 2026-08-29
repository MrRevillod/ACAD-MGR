<script lang="ts">
	import * as v from "valibot"
	import type { PlantaValue } from "$academics/value-objects/planta.value"
	import type { TableFeatures } from "@tanstack/svelte-table"
	import type { AcademicCategory } from "$categories/entity"
	import type { AcademicOptionValue } from "$options/value-objects/option.value"
	import type { AcademicCategoryOption } from "$options/entity"
	import type { AcademicWorkPosition } from "$work-positions/entity"
	import type { UpdateAppConfigInput } from "$shared/config/dtos"

	import { toast } from "svelte-sonner"
	import { useSearchParams } from "runed/kit"
	import { useQuery, useMutation, queryClient } from "$shared/http/tanstack"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import { renderSnippet, createColumnHelper } from "@tanstack/svelte-table"

	import { categoryService } from "$categories/service"
	import { optionService } from "$options/service"
	import { positionService } from "$work-positions/service"
	import { configService } from "$shared/config/service"
	import { academicService } from "$academics/service"
	import { useConfig } from "$shared/config/queries"
	import { updateAppConfigSchema } from "$shared/config/dtos"
	import { CONFIG_TABS } from "$shared/config/tabs"

	import { Plus, Loader, Pencil, Trash2, Settings, Send, Mail } from "@lucide/svelte"
	import Badge from "$shared/components/ui/badge.svelte"
	import Button from "$shared/components/ui/button.svelte"
	import DataTable from "$shared/components/ui/data-table.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import NumberInput from "$shared/components/ui/form/number-input.svelte"
	import CategoryDialog from "$categories/components/category-dialog.svelte"
	import OptionDialog from "$options/components/option-dialog.svelte"
	import PositionDialog from "$work-positions/components/position-dialog.svelte"

	const tabSchema = v.object({
		tab: v.optional(v.fallback(v.picklist(CONFIG_TABS.map((t) => t.id)), "general"), "general"),
	})

	const tabParams = useSearchParams(tabSchema, { pushHistory: true })
	const activeTab = $derived(tabParams.tab)

	// Categorías ------------------------------------------------------------
	const categoriesQuery = useQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
		enabled: activeTab === "categories" || activeTab === "options",
	}))
	let showCreateCategory = $state(false)

	const categoryHelper = createColumnHelper<TableFeatures, AcademicCategory>()
	const categoryColumns = [
		categoryHelper.accessor("name", { header: "Nombre" }),
		categoryHelper.accessor("planta", {
			header: "Planta",
			cell: (info) => renderSnippet(plantaBadge, { value: info.getValue() }),
		}),
		categoryHelper.display({
			id: "actions",
			header: "Acciones",
			cell: () => renderSnippet(actionsCell, {}),
		}),
	]

	// Opciones --------------------------------------------------------------
	const optionsQuery = useQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionService.list(),
		enabled: activeTab === "options",
	}))
	let showCreateOption = $state(false)

	const categories = $derived(categoriesQuery.data ?? [])
	const categoryMap = $derived(Object.fromEntries(categories.map((c) => [c.id, c.name])))

	const optionHelper = createColumnHelper<TableFeatures, AcademicCategoryOption>()
	const optionColumns = [
		optionHelper.accessor("categoryId", {
			header: "Categoría",
			cell: (info) => categoryMap[info.getValue()] ?? info.getValue(),
		}),
		optionHelper.accessor("option", {
			header: "Opción",
			cell: (info) => renderSnippet(optionBadge, { value: info.getValue() }),
		}),
		optionHelper.accessor("hours", {
			header: "Horas",
			cell: (info) => info.getValue()?.toLocaleString("es-CL") ?? "—",
		}),
		optionHelper.display({
			id: "actions",
			header: "Acciones",
			cell: () => renderSnippet(actionsCell, {}),
		}),
	]

	// Cargos ----------------------------------------------------------------
	const positionsQuery = useQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionService.list(),
		enabled: activeTab === "positions",
	}))
	let showCreatePosition = $state(false)

	const positionHelper = createColumnHelper<TableFeatures, AcademicWorkPosition>()
	const positionColumns = [
		positionHelper.accessor("name", { header: "Nombre" }),
		positionHelper.display({
			id: "actions",
			header: "Acciones",
			cell: () => renderSnippet(actionsCell, {}),
		}),
	]

	// Códigos de edición ----------------------------------------------------
	let showSendEditCodesConfirmDialog = $state(false)

	const sendEditCodesMutation = useMutation(() => ({
		mutationFn: () => academicService.sendEditCodesMass(),
		onSuccess: (count) => {
			toast.success(`Códigos enviados a ${count} académicos`)
		},
		onError: () => toast.error("Error al enviar los códigos de edición"),
	}))

	function handleSendEditCodes() {
		showSendEditCodesConfirmDialog = true
	}

	function confirmSendEditCodes() {
		showSendEditCodesConfirmDialog = false
		sendEditCodesMutation.mutate()
	}

	// JCE -------------------------------------------------------------------
	const configQuery = useConfig()
	const jceMax = $derived(configQuery.data?.jceMax ?? 42.5)
	const configForm = $derived.by(() => createForm({ schema: updateAppConfigSchema }))

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
		reset(configForm, { initialInput: { jceMax } })
	})
</script>

<svelte:head>
	<title
		>{CONFIG_TABS.find((t) => t.id === activeTab)?.label ?? "Configuración"} | Administración</title
	>
</svelte:head>

<div>
	{#if activeTab === "categories"}
		<div>
			<div class="mb-6 flex items-center justify-between">
				<div>
					<h2 class="text-base font-semibold text-corp-ink">Categorías Académicas</h2>
					<p class="mt-1 text-sm text-corp-gray">
						Gestiona las categorías académicas por planta.
					</p>
				</div>
				<Button onclick={() => (showCreateCategory = true)}>
					<Plus class="size-4" />
					Nueva
				</Button>
			</div>

			{#if categoriesQuery.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else}
				<DataTable
					data={categoriesQuery.data ?? []}
					columns={categoryColumns}
					pageSize={10}
				/>
			{/if}
		</div>

		<CategoryDialog
			bind:open={showCreateCategory}
			onClose={() => (showCreateCategory = false)}
		/>
	{:else if activeTab === "options"}
		<div>
			<div class="mb-6 flex items-center justify-between">
				<div>
					<h2 class="text-base font-semibold text-corp-ink">Opciones de Categoría</h2>
					<p class="mt-1 text-sm text-corp-gray">
						Gestiona las opciones válidas por categoría.
					</p>
				</div>
				<Button onclick={() => (showCreateOption = true)}>
					<Plus class="size-4" />
					Nueva
				</Button>
			</div>

			{#if optionsQuery.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else}
				<DataTable data={optionsQuery.data ?? []} columns={optionColumns} pageSize={10} />
			{/if}
		</div>

		<OptionDialog bind:open={showCreateOption} onClose={() => (showCreateOption = false)} />
	{:else if activeTab === "positions"}
		<div>
			<div class="mb-6 flex items-center justify-between">
				<div>
					<h2 class="text-base font-semibold text-corp-ink">Cargos Laborales</h2>
					<p class="mt-1 text-sm text-corp-gray">
						Gestiona los cargos laborales de los académicos.
					</p>
				</div>
				<Button onclick={() => (showCreatePosition = true)}>
					<Plus class="size-4" />
					Nuevo
				</Button>
			</div>

			{#if positionsQuery.isPending}
				<div class="flex items-center justify-center py-16">
					<Loader class="size-6 animate-spin text-corp-gray" />
				</div>
			{:else}
				<DataTable
					data={positionsQuery.data ?? []}
					columns={positionColumns}
					pageSize={10}
				/>
			{/if}
		</div>

		<PositionDialog
			bind:open={showCreatePosition}
			onClose={() => (showCreatePosition = false)}
		/>
	{:else}
		<div class="grid grid-cols-1 items-stretch gap-6 lg:grid-cols-2">
			<div class="flex flex-col rounded-xl border border-corp-gray/20 bg-white p-6">
				<div
					class="mb-6 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
				>
					<Settings class="size-4 text-corp-blue" />
					Jornada Completa Equivalente (JCE)
				</div>

				{#if configQuery.isPending}
					<div class="flex flex-1 items-center justify-center py-8">
						<Loader class="size-6 animate-spin text-corp-gray" />
					</div>
				{:else}
					<Form
						of={configForm}
						class="flex flex-1 flex-col"
						onsubmit={(output) => updateConfig.mutate(output)}
					>
						<div class="flex flex-1 flex-col">
							<p class="text-sm leading-relaxed text-corp-gray">
								Horas de la jornada completa equivalente. Define el máximo permitido
								para la carga de cada académico. Cambiar este valor no re-escala los
								registros existentes.
							</p>

							<div class="mt-6 space-y-4">
								<Field of={configForm} path={["jceMax"]}>
									{#snippet children(field)}
										<NumberInput
											{...field.props}
											input={field.input ?? ""}
											errors={field.errors}
											label="JCE máxima"
											step={0.25}
										/>
									{/snippet}
								</Field>
							</div>

							<div class="mt-auto flex justify-end pt-6">
								<Button type="submit" disabled={updateConfig.isPending}>
									{updateConfig.isPending ? "Guardando..." : "Guardar"}
								</Button>
							</div>
						</div>
					</Form>
				{/if}
			</div>

			<div class="flex flex-col rounded-xl border border-corp-gray/20 bg-white p-6">
				<div
					class="mb-6 flex items-center gap-2 text-xs font-semibold tracking-widest uppercase text-corp-blue"
				>
					<Send class="size-4 text-corp-blue" />
					Códigos de edición de perfil
				</div>

				<div class="flex flex-1 flex-col">
					<p class="text-sm leading-relaxed text-corp-gray">
						Los códigos son de un solo uso y permiten a los académicos solicitar un
						enlace de edición de su perfil. El sistema mantiene aproximadamente 10
						códigos vigentes por académico y se envían por correo electrónico.
					</p>
					<p class="mt-3 text-xs text-corp-gray/70">
						Cada envío genera o completa los códigos vigentes del académico.
					</p>

					<div class="mt-auto flex justify-end pt-6">
						<Button
							onclick={handleSendEditCodes}
							disabled={sendEditCodesMutation.isPending}
						>
							{#if sendEditCodesMutation.isPending}
								<Loader class="size-4 animate-spin" />
								Enviando...
							{:else}
								<Send class="size-4" />
								Enviar códigos a todos
							{/if}
						</Button>
					</div>
				</div>
			</div>
		</div>
	{/if}
</div>

<Dialog
	bind:open={showSendEditCodesConfirmDialog}
	title="Enviar códigos de edición masivo"
	description="Usa esta función para permitir que todos los académicos editen su perfil de forma segura."
>
	<div class="space-y-4">
		<div class="rounded-lg bg-corp-blue/5 p-4">
			<p class="text-sm text-corp-ink">
				Se enviará un <strong>código de 8 caracteres</strong> al correo de cada académico. Con este código,
				podrán generar enlaces temporales para editar:
			</p>
			<ul class="mt-3 space-y-1 text-xs text-corp-gray">
				<li class="flex items-start gap-2">
					<span class="mt-1.5 inline-block size-1.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span>Nombres y apellidos</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 inline-block size-1.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span>Correo electrónico</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 inline-block size-1.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span>ORCID y otras ID externas</span>
				</li>
			</ul>
		</div>

		<p class="text-xs text-corp-gray">
			Cada académico recibirá su código personalizado por correo. Esta es una forma segura de delegación sin
			dar acceso directo a la plataforma.
		</p>

		<div class="flex justify-end gap-2">
			<Button variant="secondary" onclick={() => (showSendEditCodesConfirmDialog = false)}>
				Cancelar
			</Button>
			<Button variant="primary" disabled={sendEditCodesMutation.isPending} onclick={confirmSendEditCodes}>
				{#if sendEditCodesMutation.isPending}
					<Loader class="size-4 animate-spin" />
					Enviando...
				{:else}
					<Mail class="size-4" />
					Enviar códigos
				{/if}
			</Button>
		</div>
	</div>
</Dialog>

{#snippet plantaBadge(params: { value: PlantaValue })}
	<Badge variant={params.value.code === "permanente" ? "advanced" : "base"}>
		{params.value.toDisplay()}
	</Badge>
{/snippet}

{#snippet optionBadge(params: { value: AcademicOptionValue })}
	<Badge variant={params.value.code === "research" ? "advanced" : "base"}>
		{params.value.toDisplay()}
	</Badge>
{/snippet}

{#snippet actionsCell(_: Record<string, never>)}
	<div class="flex items-center gap-1">
		<button
			class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-corp-ink"
		>
			<Pencil class="size-4" />
		</button>
		<button
			class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
		>
			<Trash2 class="size-4" />
		</button>
	</div>
{/snippet}
