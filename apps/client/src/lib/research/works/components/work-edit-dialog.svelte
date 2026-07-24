<script lang="ts">
	import { RotateCcw } from "@lucide/svelte"
	import { createForm, Field, Form, reset } from "@formisch/svelte"
	import * as v from "valibot"

	import type { WorkDetail } from "$works/entity"

	import Button from "$shared/components/ui/button.svelte"
	import Dialog from "$shared/components/ui/dialog.svelte"
	import Select from "$shared/components/ui/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import FormFooter from "$shared/components/ui/form/footer.svelte"

	import { useClearOverridesMutation, useUpdateOverridesMutation } from "$works/queries"

	const currentYear = new Date().getFullYear()

	const yearItems = Array.from({ length: currentYear - 1900 + 1 }, (_, i) => {
		const v = String(1900 + i)
		return { value: v, label: v }
	}).toReversed()

	const editSchema = v.object({
		title: v.pipe(v.string(), v.maxLength(2000)),
		abstract: v.nullable(v.pipe(v.string())),
		doi: v.nullable(v.pipe(v.string(), v.maxLength(500))),
		publicationYear: v.nullable(v.pipe(v.string())),
		isAccepted: v.boolean(),
		isPublished: v.boolean(),
	})

	type EditData = v.InferInput<typeof editSchema>

	interface Props {
		work: WorkDetail
		open: boolean
	}

	let { work, open = $bindable(false) }: Props = $props()

	const form = createForm({ schema: editSchema })

	$effect(() => {
		if (open)
			reset(form, {
				initialInput: {
					title: work.title,
					abstract: work.abstract,
					doi: work.doi,
					publicationYear: work.publicationYear?.toString() ?? null,
					isAccepted: work.isAccepted,
					isPublished: work.isPublished,
				} satisfies EditData,
			})
	})

	const updateMutation = useUpdateOverridesMutation()
	const clearMutation = useClearOverridesMutation()

	function handleSubmit(output: EditData) {
		const data: Record<string, unknown> = {}
		if (output.title !== work.title) data.title = output.title
		if (output.abstract !== work.abstract) data.abstract = output.abstract
		if (output.doi !== work.doi) data.doi = output.doi
		if (output.publicationYear !== (work.publicationYear?.toString() ?? null))
			data.publicationYear = output.publicationYear ? Number(output.publicationYear) : null
		if (output.isAccepted !== work.isAccepted) data.isAccepted = output.isAccepted
		if (output.isPublished !== work.isPublished) data.isPublished = output.isPublished

		updateMutation.mutate(
			{ id: work.id, data },
			{
				onSuccess: () => {
					open = false
				},
			},
		)
	}

	function handleRestoreAll() {
		clearMutation.mutate(work.id, {
			onSuccess: () => {
				open = false
			},
		})
	}
</script>

<Dialog bind:open title="Editar publicación" class="max-w-xl">
	<Form of={form} onsubmit={handleSubmit}>
		<div class="grid gap-4">
			<Field of={form} path={["title"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Título"
					/>
				{/snippet}
			</Field>

			<Field of={form} path={["abstract"]}>
				{#snippet children(field)}
					<TextInput
						{...field.props}
						input={field.input}
						errors={field.errors}
						type="text"
						label="Abstract"
					/>
				{/snippet}
			</Field>

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
						/>
						{#if field.errors}
							<p class="text-xs text-red-500">{field.errors[0]}</p>
						{/if}
					</div>
				{/snippet}
			</Field>

			<div class="flex gap-6">
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
			</div>
		</div>

		<FormFooter
			onCancel={() => (open = false)}
			submitLabel="Guardar cambios"
			isPending={updateMutation.isPending}
		>
			<Button
				variant="secondary"
				type="button"
				disabled={clearMutation.isPending}
				onclick={handleRestoreAll}
			>
				<RotateCcw class="size-4" />
				Restaurar originales
			</Button>
		</FormFooter>
	</Form>
</Dialog>
