<script lang="ts">
	import type { CollaborationRecommendationDTO, MatchWorkRefDTO } from "$collaborations/dtos"

	import { ChevronDown, Flame, Hash } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"
	import HtmlRenderer from "$shared/components/ui/html-renderer.svelte"
	import { authStore } from "$lib/auth/store.svelte"
	import { FullName } from "$shared/value-objects/full-name.value"
	import WorkDetailDialog from "$works/components/work-detail-dialog.svelte"

	interface Props {
		open: boolean
		recommendation: CollaborationRecommendationDTO | null
		focusName: string
	}

	let { open = $bindable(false), recommendation, focusName }: Props = $props()

	let expandedMatchId = $state<string | null>(null)
	let workDetailOpen = $state(false)
	let selectedWorkId = $state<string | null>(null)

	const fullName = $derived(
		recommendation
			? FullName.of(
					recommendation.names,
					recommendation.paternalSurname,
					recommendation.maternalSurname,
				).format()
			: "",
	)

	const profileHref = $derived(
		recommendation
			? `${authStore.isAuthenticated ? "/academics" : "/public/academics"}/${recommendation.academicId}`
			: "#",
	)

	function toggleMatch(id: string) {
		expandedMatchId = expandedMatchId === id ? null : id
	}

	function openWork(workId: string) {
		selectedWorkId = workId
		workDetailOpen = true
	}

	function scorePillClass(score: number): string {
		if (score >= 0.8) return "bg-green-600/10 text-green-700"
		if (score >= 0.6) return "bg-corp-blue/10 text-corp-blue"
		return "bg-corp-gray/10 text-corp-gray"
	}

	function workScoreWidth(score: number): number {
		return Math.round(score * 100)
	}
</script>

<Dialog bind:open title="Recomendación de posible colaboración" class="max-w-2xl">
	{#if !recommendation}
		<p class="py-8 text-center text-sm text-corp-gray">No hay información para mostrar.</p>
	{:else}
		<div class="flex max-h-[70vh] flex-col">
			<div
				class="mb-4 grid shrink-0 grid-cols-2 gap-2 rounded-lg border border-corp-gray/15 bg-corp-gray/[0.03] px-3 py-2"
			>
				<span
					class="flex min-w-0 items-center justify-center gap-1.5 text-sm font-medium text-[#1A1A1A]"
				>
					<span class="size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span class="truncate">{focusName}</span>
				</span>
				<span
					class="flex min-w-0 items-center justify-center gap-1.5 text-sm font-medium text-[#1A1A1A]"
				>
					<span class="size-2.5 shrink-0 rounded-full bg-green-500"></span>
					<span class="truncate">{fullName}</span>
				</span>
			</div>

			<div class="min-h-0 flex-1 space-y-4 overflow-y-auto pr-1">
				<section>
					<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
						<Hash class="size-4 text-green-600/80" />
						Coincidencias ({recommendation.weight})
					</h3>

					<div class="mt-2 space-y-2">
						{#each recommendation.matches as match (match.id)}
							<div
								class="overflow-hidden rounded-lg border border-corp-gray/15 bg-white transition-colors"
							>
								<button
									type="button"
									class="flex w-full items-center gap-3 px-3 py-2.5 text-left transition-colors hover:bg-corp-gray/5"
									onclick={() => toggleMatch(match.id)}
								>
									<span
										class={`shrink-0 rounded-full px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide ${
											match.type === "topic"
												? "bg-green-600/10 text-green-600"
												: "bg-corp-gray/10 text-corp-gray"
										}`}
									>
										{match.type}
									</span>
									<span
										class="min-w-0 flex-1 truncate text-sm font-medium text-[#1A1A1A]"
									>
										{match.name}
									</span>
									<ChevronDown
										class={`size-4 shrink-0 text-corp-gray transition-transform ${
											expandedMatchId === match.id ? "rotate-180" : ""
										}`}
									/>
								</button>

								{#if expandedMatchId === match.id}
									<div
										class="grid max-h-72 grid-cols-1 gap-3 overflow-y-auto border-t border-corp-gray/10 p-3 sm:grid-cols-2"
									>
										<div>
											<p
												class="mb-1.5 px-1 text-[11px] font-semibold uppercase tracking-wide text-corp-blue"
											>
												Publicaciones
											</p>
											{#if match.focusWorks.length === 0}
												<p class="px-1 text-xs text-corp-gray">
													Sin coincidencias en tus works.
												</p>
											{:else}
												<ul class="space-y-1.5">
													{#each match.focusWorks as work (work.workId)}
														{@render WorkRow(work)}
													{/each}
												</ul>
											{/if}
										</div>
										<div>
											<p
												class="mb-1.5 px-1 text-[11px] font-semibold uppercase tracking-wide text-green-700"
											>
												Publicaciones
											</p>
											{#if match.candidateWorks.length === 0}
												<p class="px-1 text-xs text-corp-gray">
													Sin coincidencias en sus works.
												</p>
											{:else}
												<ul class="space-y-1.5">
													{#each match.candidateWorks as work (work.workId)}
														{@render WorkRow(work)}
													{/each}
												</ul>
											{/if}
										</div>
									</div>
								{/if}
							</div>
						{/each}
					</div>
				</section>
			</div>

			<div
				class="mt-4 flex shrink-0 items-center justify-between gap-2 border-t border-corp-gray/10 pt-3"
			>
				<div class="flex items-center gap-2">
					<Flame class="size-4 shrink-0 text-corp-gray" />
					<p class="text-xs text-corp-gray">
						{fullName} tiene {recommendation.totalWorks} publicaciones registradas.
					</p>
				</div>
				<a
					href={profileHref}
					class="shrink-0 text-xs font-semibold text-corp-blue hover:underline"
				>
					Ir al perfil académico →
				</a>
			</div>
		</div>
	{/if}
</Dialog>

{#snippet WorkRow(work: MatchWorkRefDTO)}
	<li class="rounded-lg border border-corp-gray/10 bg-corp-gray/[0.03]">
		<button
			type="button"
			class="group flex w-full items-start gap-2.5 px-2.5 py-2 text-left"
			onclick={() => openWork(work.workId)}
		>
			<span class="min-w-0 flex-1">
				<span class="block" title={work.title}>
					<HtmlRenderer
						html={work.title}
						class="block truncate text-[13px] font-medium leading-snug text-[#1A1A1A] group-hover:text-corp-blue"
					/>
				</span>
				<span class="mt-1 flex items-center gap-2">
					<span
						class={`shrink-0 rounded-full px-2 py-0.5 text-[10px] font-semibold tabular-nums ${scorePillClass(work.score)}`}
					>
						{work.score.toFixed(2)}
					</span>
					<span class="h-1 min-w-0 flex-1 overflow-hidden rounded-full bg-corp-gray/10">
						<span
							class="block h-full rounded-full bg-corp-blue"
							style:width={`${workScoreWidth(work.score)}%`}
						></span>
					</span>
				</span>
			</span>
		</button>
	</li>
{/snippet}

<WorkDetailDialog bind:open={workDetailOpen} bind:workId={selectedWorkId} />
