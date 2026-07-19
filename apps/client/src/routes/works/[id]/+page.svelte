<script lang="ts">
	import {
		CircleAlert,
		ArrowLeft,
		Building2,
		ExternalLink,
		Loader,
		Mail,
		Network,
		Tag,
	} from "@lucide/svelte"

	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { toast } from "svelte-sonner"

	import Badge from "$lib/shared/components/ui/badge.svelte"
	import HtmlRenderer from "$shared/components/ui/html-renderer.svelte"

	import { DateValue } from "$shared/value-objects/date.value"
	import { FullName } from "$shared/value-objects/full-name.value"
	import { WORK_TYPE_LABELS } from "$works/dtos"
	import { useWorkDetailQuery } from "$works/queries"
	import { AuthorshipPositionValue } from "$works/value-objects/position.value"
	import { authStore } from "$lib/auth/store.svelte"
	import { resolve } from "$app/paths"

	const id = $derived(page.params.id ?? "")

	const query = useWorkDetailQuery(() => id)

	function copyOrcid(orcid: string) {
		void navigator.clipboard.writeText(orcid)
		toast.success("ORCID copiado al portapapeles")
	}
</script>

<div class="h-full overflow-y-auto">
	<div class="mx-auto max-w-4xl px-4 py-8 sm:px-6 lg:px-8">
		<button
			type="button"
			onclick={() => void goto("/works")}
			class="inline-flex items-center gap-1 text-sm text-corp-blue transition-colors hover:text-corp-blue/80"
		>
			<ArrowLeft class="size-3.5" />
			Volver al catálogo
		</button>

		{#if query.isPending}
			<div class="flex justify-center py-12">
				<Loader class="size-6 animate-spin text-corp-gray" />
			</div>
		{:else if query.isError || !query.data}
			<div class="flex flex-col items-center py-12 text-center">
				<CircleAlert class="size-8 text-red-500" />
				<p class="mt-3 text-sm text-corp-gray">Error al cargar la obra.</p>
			</div>
		{:else}
			{@const work = query.data}
			<article class="mt-6 space-y-8">
				<header>
					<div class="flex flex-wrap items-center gap-2">
						<Badge variant="base">{WORK_TYPE_LABELS[work.type] ?? work.type}</Badge>
						{#if work.publicationYear}
							<span class="text-sm text-corp-gray">
								<span class="tabular-nums">{work.publicationYear}</span>
								{#if work.publicationDate}· {DateValue.formatDate(
										work.publicationDate,
									)}{/if}
							</span>
						{/if}
						{#if work.isAccepted}
							<span
								class="inline-flex items-center rounded-full bg-emerald-50 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-emerald-700 uppercase"
							>
								Aceptado
							</span>
						{/if}
						{#if work.isPublished}
							<span
								class="inline-flex items-center rounded-full bg-corp-blue/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-blue uppercase"
							>
								Publicado
							</span>
						{/if}
					</div>
					<h1 class="mt-3 text-2xl font-semibold text-balance text-[#1A1A1A]">
						<HtmlRenderer html={work.title} />
					</h1>
					<div
						class="mt-3 flex flex-wrap items-center gap-x-3 gap-y-1 text-xs text-corp-gray"
					>
						{#if work.doi}
							<a
								href={work.doi}
								target="_blank"
								rel="noopener"
								class="inline-flex items-center gap-1 text-corp-blue hover:underline"
							>
								<span>Ver DOI</span>
								<ExternalLink class="size-3" />
							</a>
						{/if}
						<a
							href={work.openalexId}
							target="_blank"
							rel="noopener"
							class="inline-flex items-center gap-1 text-corp-blue hover:underline"
						>
							<span>Ver en OpenAlex</span>
							<ExternalLink class="size-3" />
						</a>
						<span>Idioma: {work.lang}</span>
					</div>
				</header>

				{#if work.abstract}
					<section>
						<h2
							class="mb-3 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							Abstract
						</h2>
						<HtmlRenderer
							as="p"
							html={work.abstract}
							class="text-pretty text-sm leading-relaxed text-[#1A1A1A]"
						/>
					</section>
				{/if}

				{#if work.source}
					<section>
						<h2
							class="mb-3 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							Publicado en
						</h2>
						<p class="text-sm font-medium text-[#1A1A1A]">
							{work.source.displayName}
							<span class="text-corp-gray">· {work.source.ty}</span>
							<span class="mx-1.5 text-corp-gray/40">|</span>
							Indexación:
							{#if work.source.kind.code}
								<Badge
									variant={work.source.kind.code === "scopus"
										? "advanced"
										: "base"}
								>
									{work.source.kind.toDisplay()}
								</Badge>
							{:else}
								<span
									class="inline-flex items-center rounded-full bg-red-100 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-red-700 uppercase ml-1"
								>
									Desconocida
								</span>
							{/if}
						</p>
					</section>
				{/if}

				{#if work.authorships.length > 0}
					<section>
						<h2
							class="mb-3 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							Autores
							<span
								class="rounded-full bg-corp-gray/10 px-1.5 py-0.5 text-[10px] font-semibold text-corp-gray"
							>
								{work.authorships.length}
							</span>
						</h2>
						<div class="space-y-2">
							{#each work.authorships as auth (auth.orcid)}
								<div class="rounded-lg border border-corp-gray/10 bg-white p-3">
									<div class="flex items-center justify-between gap-2">
										<div class="flex items-center gap-2">
											<p class="text-sm font-medium text-[#1A1A1A]">
												{FullName.fromFullString(auth.name)}
											</p>
											{#if auth.isCorresponding}
												<Badge variant="default">
													<Mail class="mr-1 size-3" />
													Corresponding
												</Badge>
											{/if}
											{#if auth.isExternal}
												<span
													class="inline-flex items-center rounded-full bg-corp-gray/10 px-2 py-0.5 text-[11px] font-semibold tracking-wide text-corp-gray uppercase"
												>
													Externo
												</span>
											{/if}
										</div>
										{#if !auth.isExternal && auth.academicId}
											<a
												href={authStore.isAuthenticated
													? resolve(`/academics/${auth.academicId}`)
													: resolve(
															`/public/academics/${auth.academicId}`,
														)}
												class="shrink-0 text-xs font-medium text-corp-blue hover:underline"
											>
												Ir al perfil académico →
											</a>
										{/if}
									</div>
									<button
										type="button"
										onclick={() => copyOrcid(auth.orcid)}
										class="mt-0.5 inline-flex items-center gap-1 text-xs text-corp-gray transition-colors hover:text-corp-blue"
										title="Copiar ORCID"
									>
										<span>ORCID: {auth.orcid}</span>
									</button>
									<p class="mt-0.5 text-xs text-corp-gray">
										Posición: <span class="font-medium text-[#1A1A1A]"
											>{AuthorshipPositionValue.LABELS[auth.position]}</span
										>
									</p>
									{#if auth.affiliations.length > 0}
										<ul class="mt-2 space-y-1">
											{#each auth.affiliations as aff, i (i)}
												<li
													class="flex items-start gap-1.5 text-xs text-corp-gray"
												>
													<Building2 class="mt-0.5 size-3 shrink-0" />
													<span>{aff}</span>
												</li>
											{/each}
										</ul>
									{/if}
								</div>
							{/each}
						</div>
					</section>
				{/if}

				{#if work.topics.length > 0}
					<section>
						<h2
							class="mb-3 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							<Network class="size-3" />
							Topics
						</h2>
						<div class="space-y-2">
							{#each work.topics as t (t.topicId)}
								<div
									class="flex items-center justify-between gap-3 rounded-lg border border-corp-gray/10 bg-white p-3"
								>
									<div class="min-w-0">
										<p class="text-sm font-medium text-[#1A1A1A]">{t.name}</p>
										<p class="mt-0.5 truncate text-xs text-corp-gray">
											{t.domainName}
											<span class="text-corp-gray/40">→</span>
											{t.fieldName}
											<span class="text-corp-gray/40">→</span>
											{t.subfieldName}
										</p>
									</div>
									<span
										class="shrink-0 rounded-full bg-corp-blue/10 px-2 py-0.5 text-xs font-semibold text-corp-blue tabular-nums"
									>
										{(t.score * 100).toFixed(1)}%
									</span>
								</div>
							{/each}
						</div>
					</section>
				{/if}

				{#if work.keywords.length > 0}
					<section>
						<h2
							class="mb-3 flex items-center gap-1.5 text-xs font-semibold tracking-widest uppercase text-corp-blue"
						>
							<Tag class="size-3" />
							Keywords
						</h2>
						<div class="flex flex-wrap gap-1.5">
							{#each work.keywords as k (k.keywordId)}
								<span
									class="inline-flex items-center gap-1 rounded-full bg-corp-gray/10 px-2.5 py-1 text-xs text-corp-gray"
								>
									{k.name}
									<span class="tabular-nums text-[10px] text-corp-gray/70">
										{(k.score * 100).toFixed(0)}%
									</span>
								</span>
							{/each}
						</div>
					</section>
				{/if}
			</article>
		{/if}
	</div>
</div>
