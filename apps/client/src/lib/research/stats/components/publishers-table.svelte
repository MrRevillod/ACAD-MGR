<script lang="ts">
	import { goto } from "$app/navigation"
	import { resolve } from "$app/paths"
	import Badge from "$lib/shared/components/ui/badge.svelte"
	import { FullName } from "$shared/value-objects/full-name.value"
	import type { TopPublisher } from "../types"

	interface Props {
		publishers: TopPublisher[]
	}

	let { publishers }: Props = $props()

	const optionLabel: Record<string, string> = {
		teaching: "Docencia",
		research: "Investigación",
	}

	function goToAcademic(id: string) {
		void goto(resolve(`/academics/${id}`))
	}
</script>

<div class="overflow-x-auto">
	<table class="w-full caption-bottom text-sm">
		<thead>
			<tr class="border-b border-corp-gray/20 bg-gray-100 text-left">
				<th class="px-4 py-3 text-xs font-medium tracking-wide uppercase text-corp-gray">Nombre</th>
				<th class="px-4 py-3 text-right text-xs font-medium tracking-wide uppercase text-corp-gray">Total</th>
				<th class="px-3 py-3 text-right text-xs font-medium tracking-wide uppercase text-corp-gray">Scopus</th>
				<th class="px-3 py-3 text-right text-xs font-medium tracking-wide uppercase text-corp-gray">WoS</th>
				<th class="px-4 py-3 text-xs font-medium tracking-wide uppercase text-corp-gray">Opción</th>
			</tr>
		</thead>
		<tbody>
			{#each publishers as p, i (p.academicId)}
				<tr class="border-b border-corp-gray/10 transition-colors {i % 2 === 0 ? 'bg-gray-50' : ''} hover:bg-corp-blue/4">
					<td class="px-4 py-2.5">
						<button
							class="text-left text-[#1A1A1A] font-medium hover:text-corp-blue hover:underline"
							onclick={() => goToAcademic(p.academicId)}
						>
							{FullName.fromFullString(p.name)}
						</button>
					</td>
					<td class="px-4 py-2.5 text-right font-semibold text-[#1A1A1A]">{p.total}</td>
					<td class="px-3 py-2.5 text-right font-medium text-corp-yellow">{p.scopus}</td>
					<td class="px-3 py-2.5 text-right font-medium text-corp-blue">{p.wos}</td>
					<td class="px-4 py-2.5">
						<Badge variant={p.option === "research" ? "advanced" : "base"}>
							{optionLabel[p.option] ?? p.option}
						</Badge>
					</td>
				</tr>
			{:else}
				<tr>
					<td colspan="5" class="px-4 py-8 text-center text-sm text-corp-gray">
						Sin publicadores para los filtros seleccionados.
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
