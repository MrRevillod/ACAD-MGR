<script lang="ts">
	import { Plus, RotateCcw, X } from "@lucide/svelte"
	import { createForm, Field, Form, handleSubmit, reset } from "@formisch/svelte"
	import { onMount } from "svelte"
	import * as v from "valibot"

	import type { WorkDetail } from "$works/entity"

	import Select from "$shared/components/ui/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import { FullName } from "$shared/value-objects/full-name.value"

	import { useResearchLinesQuery } from "$research/classification/queries"
	import {
		useClearOverridesMutation,
		useUpdateAuthorshipAffiliationsMutation,
		useUpdateOverridesMutation,
	} from "$works/queries"

	const currentYear = new Date().getFullYear()

	const yearItems = Array.from({ length: currentYear - 1900 + 1 }, (_, i) => {
		const v = String(1900 + i)
		return { value: v, label: v }
	}).toReversed()

	const editSchema = v.object({
		title: v.pipe(v.string(), v.maxLength(2000)),
		abstractText: v.nullable(v.pipe(v.string())),
		doi: v.nullable(v.pipe(v.string(), v.maxLength(500))),
		publicationYear: v.nullable(v.pipe(v.string())),
		isAccepted: v.boolean(),
		isPublished: v.boolean(),
		researchLineId: v.nullable(v.pipe(v.string())),
	})

	type EditData = v.InferInput<typeof editSchema>

	interface AuthorDraft {
		orcid: string
		name: string
		isExternal: boolean
		affiliations: string[]
		draftAffiliation: string
	}

	interface Props {
		work: WorkDetail
		submit?: (() => Promise<void>) | null
		isSaving?: boolean
		onSaved: () => void
	}

	// eslint-disable-next-line no-useless-assignment -- `$bindable` writes are read by the parent
	let { work, submit = $bindable(null), isSaving = $bindable(false), onSaved }: Props = $props()

	const form = createForm({ schema: editSchema })

	const researchLinesQuery = useResearchLinesQuery()

	const researchLineItems = $derived(
		researchLinesQuery.data?.map((rl) => ({ value: rl.id, label: rl.name })) ?? [],
	)

	const authorships = $derived(work.authorships ?? [])

	let authorDrafts = $state<AuthorDraft[]>([])
	let selectedCorrespondingOrcid = $state<string | null>(null)

	onMount(() => {
		submit = handleSubmit(form, doSave)
		reset(form, {
			initialInput: {
				title: work.title,
				abstractText: work.abstractText,
				doi: work.doi,
				publicationYear: work.publicationYear?.toString() ?? null,
				isAccepted: work.isAccepted,
				isPublished: work.isPublished,
				researchLineId: work.researchLineId ?? null,
			} satisfies EditData,
		})
		authorDrafts = authorships.map((a) => ({
			orcid: a.orcid,
			name: a.name,
			isExternal: a.isExternal,
			affiliations: [...a.affiliations],
			draftAffiliation: "",
		}))
		selectedCorrespondingOrcid = authorships.find((a) => a.isCorresponding)?.orcid ?? null
	})

	const updateMutation = useUpdateOverridesMutation()
	const clearMutation = useClearOverridesMutation()
	const affiliationsMutation = useUpdateAuthorshipAffiliationsMutation()

	function addAffiliation(index: number) {
		const draft = authorDrafts[index]
		const value = draft.draftAffiliation.trim()
		if (!value) return
		draft.affiliations = [...draft.affiliations, value]
		draft.draftAffiliation = ""
	}

	function removeAffiliation(index: number, affIndex: number) {
		const draft = authorDrafts[index]
		draft.affiliations = draft.affiliations.filter((_, i) => i !== affIndex)
	}

	async function doSave(output: EditData) {
		isSaving = true
		try {
			const data: Record<string, unknown> = {}
			if (output.title !== work.title) data.title = output.title
			if (output.abstractText !== work.abstractText) data.abstractText = output.abstractText
			if (output.doi !== work.doi) data.doi = output.doi
			if (output.publicationYear !== (work.publicationYear?.toString() ?? null))
				data.publicationYear = output.publicationYear
					? Number(output.publicationYear)
					: null
			if (output.isAccepted !== work.isAccepted) data.isAccepted = output.isAccepted
			if (output.isPublished !== work.isPublished) data.isPublished = output.isPublished

			const nextLine = output.researchLineId || null
			const prevLine = work.researchLineId ?? null
			if (nextLine !== prevLine) {
				data.researchLineId = nextLine
			}

			const currentCorresponding = authorships.find((a) => a.isCorresponding)?.orcid ?? null
			if (selectedCorrespondingOrcid !== currentCorresponding) {
				data.correspondingOrcid = selectedCorrespondingOrcid
			}

			const affiliationChanges: { orcid: string; affiliations: string[] }[] = []
			for (const draft of authorDrafts) {
				const original =
					authorships.find((a) => a.orcid === draft.orcid)?.affiliations ?? []
				if (JSON.stringify(original) !== JSON.stringify(draft.affiliations)) {
					affiliationChanges.push({
						orcid: draft.orcid,
						affiliations: [...draft.affiliations],
					})
				}
			}

			if (Object.keys(data).length > 0) {
				await updateMutation.mutateAsync({ id: work.id, data })
			}
			for (const change of affiliationChanges) {
				await affiliationsMutation.mutateAsync({
					workId: work.id,
					orcid: change.orcid,
					affiliations: change.affiliations,
				})
			}
			onSaved()
		} finally {
			isSaving = false
		}
	}

	function handleRestoreAll() {
		clearMutation.mutate(work.id, {
			onSuccess: () => onSaved(),
		})
	}
</script>

<Form of={form} onsubmit={doSave}>
	<div class="mt-6 space-y-8">
		<section>
			<Field of={form} path={["title"]}>
				{#snippet children(field)}
					<div class="space-y-1">
						<span class="block text-xs font-medium text-corp-gray">Título</span>
						<textarea
							{...field.props}
							value={field.input}
							rows={2}
							class="w-full rounded-lg border border-corp-gray/20 bg-white px-3 py-2 text-sm text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50"
						></textarea>
						{#if field.errors}
							<p class="text-xs text-red-500">{field.errors[0]}</p>
						{/if}
					</div>
				{/snippet}
			</Field>
		</section>

		<div class="lg:grid lg:grid-cols-3 lg:items-start lg:gap-6">
			<div class="min-w-0 space-y-6 lg:col-span-2">
				<section>
					<Field of={form} path={["abstractText"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Abstract</span
								>
								<div class="grid">
									<span
										class="invisible col-start-1 row-start-1 px-3 py-2 text-sm leading-6 break-words whitespace-pre-wrap"
										aria-hidden="true">{field.input ?? " "}</span
									>
									<textarea
										{...field.props}
										value={field.input ?? ""}
										class="col-start-1 row-start-1 w-full resize-none overflow-hidden rounded-lg border border-corp-gray/20 bg-white px-3 py-2 text-sm leading-6 text-[#1A1A1A] outline-none transition-colors placeholder:text-corp-gray/50 focus:border-corp-blue/50"
									></textarea>
								</div>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>
			</div>

			<aside class="mt-6 space-y-4 lg:mt-0">
				<section>
					<Field of={form} path={["publicationYear"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Año de publicación</span
								>
								<Select
									items={yearItems}
									value={field.input ?? ""}
									onValueChange={(v) => field.onInput(v || null)}
									placeholder="Seleccionar"
									class="w-full"
								/>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>

				<section class="space-y-2.5">
					<Field of={form} path={["isAccepted"]}>
						{#snippet children(field)}
							<label class="flex items-center gap-2 text-sm">
								<input
									type="checkbox"
									{...field.props}
									checked={field.input ?? false}
									class="size-4 rounded border-corp-gray/30 text-corp-blue focus:ring-corp-blue/30"
								/>
								Aceptado
							</label>
						{/snippet}
					</Field>

					<Field of={form} path={["isPublished"]}>
						{#snippet children(field)}
							<label class="flex items-center gap-2 text-sm">
								<input
									type="checkbox"
									{...field.props}
									checked={field.input ?? false}
									class="size-4 rounded border-corp-gray/30 text-corp-blue focus:ring-corp-blue/30"
								/>
								Publicado
							</label>
						{/snippet}
					</Field>
				</section>

				<section>
					<Field of={form} path={["doi"]}>
						{#snippet children(field)}
							<TextInput
								{...field.props}
								input={field.input}
								errors={field.errors}
								type="text"
								label="DOI"
							/>
						{/snippet}
					</Field>
				</section>

				<section>
					<Field of={form} path={["researchLineId"]}>
						{#snippet children(field)}
							<div class="space-y-1">
								<span class="block text-xs font-medium text-corp-gray"
									>Línea de investigación</span
								>
								<Select
									items={researchLineItems}
									value={field.input ?? ""}
									onValueChange={(v) => field.onInput(v || null)}
									placeholder="Seleccionar"
									class="w-full"
								/>
								{#if field.errors}
									<p class="text-xs text-red-500">{field.errors[0]}</p>
								{/if}
							</div>
						{/snippet}
					</Field>
				</section>
			</aside>
		</div>

		{#if authorships.length > 0}
			<section>
				<p class="mb-2 text-xs font-semibold tracking-widest uppercase text-corp-blue">
					Autores
				</p>
				<div class="grid items-start gap-2 sm:grid-cols-2">
					{#each authorDrafts as draft, index (draft.orcid)}
						<div class="min-w-0 rounded-lg border border-corp-gray/10 p-3">
							<label
								class="flex cursor-pointer items-center gap-2"
								title="Marcar como autor correspondiente"
							>
								<input
									type="radio"
									name="corresponding-author"
									value={draft.orcid}
									bind:group={selectedCorrespondingOrcid}
									class="size-4 text-corp-blue focus:ring-corp-blue/30"
								/>
								<span class="min-w-0 flex-1 text-sm font-medium text-[#1A1A1A]">
									{FullName.fromFullString(draft.name)}
								</span>
								{#if draft.isExternal}
									<span
										class="shrink-0 rounded-full bg-corp-gray/10 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-corp-gray uppercase"
									>
										Externo
									</span>
								{/if}
							</label>

							<div class="mt-2.5">
								<p
									class="mb-1 text-[11px] font-semibold uppercase tracking-wide text-corp-gray"
								>
									Afiliaciones
								</p>
								{#if draft.affiliations.length === 0}
									<p class="text-xs text-corp-gray">
										Sin afiliaciones registradas.
									</p>
								{:else}
									<ul class="space-y-1">
										{#each draft.affiliations as aff, affIndex (aff)}
											<li
												class="flex items-start gap-2 text-xs text-corp-gray"
											>
												<span class="min-w-0 flex-1">{aff}</span>
												<button
													type="button"
													title="Quitar afiliación"
													class="flex size-8 shrink-0 items-center justify-center rounded-md text-corp-gray transition-colors hover:bg-red-50 hover:text-red-600"
													onclick={() =>
														removeAffiliation(index, affIndex)}
												>
													<X class="size-3.5" />
												</button>
											</li>
										{/each}
									</ul>
								{/if}
								<div class="mt-1.5 flex items-center gap-2">
									<input
										type="text"
										bind:value={draft.draftAffiliation}
										placeholder="Nueva afiliación"
										class="min-w-0 flex-1 rounded-lg border border-corp-gray/25 bg-white px-2.5 py-1.5 text-sm text-[#1A1A1A] placeholder:text-corp-gray/70 focus:border-corp-blue focus:outline-none"
										onkeydown={(e) => {
											if (e.key === "Enter") {
												e.preventDefault()
												addAffiliation(index)
											}
										}}
									/>
									<button
										type="button"
										title="Añadir afiliación"
										class="flex size-8 shrink-0 items-center justify-center rounded-lg border border-corp-gray/20 text-corp-blue transition-colors hover:bg-corp-blue/5"
										onclick={() => addAffiliation(index)}
									>
										<Plus class="size-3.5" />
									</button>
								</div>
							</div>
						</div>
					{/each}
				</div>
				<p class="mt-2 text-[11px] text-corp-gray">
					Solo un autor puede ser el correspondiente.
				</p>
			</section>
		{/if}

		<div class="border-t border-corp-gray/10 pt-4">
			<button
				type="button"
				onclick={handleRestoreAll}
				disabled={clearMutation.isPending}
				class="inline-flex items-center gap-1.5 text-xs font-semibold text-corp-gray transition-colors hover:text-corp-blue disabled:opacity-50"
			>
				<RotateCcw class="size-3.5" />
				Restaurar originales
			</button>
		</div>
	</div>
</Form>
