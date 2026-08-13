<script lang="ts">
	import { Building2, ChevronDown, ExternalLink } from "@lucide/svelte"

	import type { AuthorshipDTO } from "$works/dtos"
	import { FullName } from "$shared/value-objects/full-name.value"
	import { authStore } from "$lib/auth/store.svelte"
	import { AuthorshipPositionValue } from "$works/value-objects/position.value"

	interface Props {
		authors: AuthorshipDTO[]
	}

	let { authors }: Props = $props()

	let expandedAffiliationAuthor = $state<string | null>(null)

	function toggleAffiliations(orcid: string) {
		expandedAffiliationAuthor = expandedAffiliationAuthor === orcid ? null : orcid
	}
</script>

<div class="grid items-start gap-2 sm:grid-cols-2">
	{#each authors as auth, index (auth.orcid)}
		<div class="min-w-0 rounded-lg border border-corp-gray/10 bg-white p-3">
			<div class="flex items-start justify-between gap-2">
				<p class="min-w-0 flex-1 truncate text-sm font-medium text-[#1A1A1A]">
					{FullName.fromFullString(auth.name)}
				</p>
				<div class="flex shrink-0 flex-wrap items-center justify-end gap-1">
					{#if auth.isCorresponding}
						<span
							class="shrink-0 rounded-full bg-corp-blue/10 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-corp-blue uppercase"
						>
							Correspondiente
						</span>
					{/if}
					{#if auth.isExternal}
						<span
							class="shrink-0 rounded-full bg-corp-gray/10 px-2 py-0.5 text-[10px] font-semibold tracking-wide text-corp-gray uppercase"
						>
							Externo
						</span>
					{/if}
					{#if !auth.isExternal && auth.academicId}
						<a
							href={authStore.isAuthenticated
								? `/academics/${auth.academicId}`
								: `/public/academics/${auth.academicId}`}
							class="shrink-0 text-xs font-medium text-corp-blue hover:underline"
							title="Ir al perfil académico"
						>
							Perfil →
						</a>
					{/if}
				</div>
			</div>
			<a
				href={auth.orcid}
				target="_blank"
				rel="noopener"
				class="mt-0.5 inline-flex items-center gap-1 text-xs text-corp-blue transition-colors hover:underline"
				title={auth.orcid}
			>
				{auth.orcid}
				<ExternalLink class="size-3" />
			</a>
			<p class="mt-0.5 text-xs text-corp-gray">
				{AuthorshipPositionValue.labelFor(index)}
			</p>

			{#if auth.affiliations.length > 0}
				<button
					type="button"
					class="mt-2 inline-flex items-center gap-1 text-[11px] font-semibold uppercase tracking-wide text-corp-gray transition-colors hover:text-corp-blue"
					onclick={() => toggleAffiliations(auth.orcid)}
					aria-expanded={expandedAffiliationAuthor === auth.orcid}
				>
					Afiliaciones
					<ChevronDown
						class={`size-3.5 transition-transform ${
							expandedAffiliationAuthor === auth.orcid ? "rotate-180" : ""
						}`}
					/>
				</button>

				{#if expandedAffiliationAuthor === auth.orcid}
					<ul class="mt-1.5 space-y-1">
						{#each auth.affiliations as aff, i (i)}
							<li class="flex items-start gap-1.5 text-xs text-corp-gray">
								<Building2 class="mt-0.5 size-3 shrink-0" />
								<span class="min-w-0 flex-1 truncate" title={aff}>{aff}</span>
							</li>
						{/each}
					</ul>
				{/if}
			{/if}
		</div>
	{/each}
</div>
