<script lang="ts">
	import type { Academic } from "$academics/entity"
	import type { CollaborationNodeDTO, CollaborationWorkRefDTO } from "$collaborations/dtos"

	import {
		forceCenter,
		forceCollide,
		forceLink,
		forceManyBody,
		type SimulationLinkDatum,
		type SimulationNodeDatum,
	} from "d3-force"
	import { Chart, Circle, Layer, Link, Tooltip } from "layerchart"
	import { ForceSimulation } from "layerchart/force"
	import { CircleAlert, Loader, Network } from "@lucide/svelte"

	import { useCollaborationGraphQuery } from "$collaborations/queries"
	import { FullName } from "$shared/value-objects/full-name.value"

	import EdgeWorksDialog from "./edge-works-dialog.svelte"

	interface Props {
		academic: Academic
	}

	let { academic }: Props = $props()

	const query = useCollaborationGraphQuery(() => academic.id)
	const graph = $derived(query.data)

	type GraphNode = CollaborationNodeDTO & SimulationNodeDatum & { displayName: string }
	type GraphLink = {
		source: string
		target: string
		weight: number
		works: CollaborationWorkRefDTO[]
	} & SimulationLinkDatum<GraphNode>

	const graphNodes = $derived.by<GraphNode[]>(() =>
		(graph?.nodes ?? []).map((n) => ({ ...n, displayName: FullName.fromFullString(n.name) })),
	)
	const graphLinks = $derived.by<GraphLink[]>(() =>
		(graph?.edges ?? []).map((e) => ({
			source: e.sourceId,
			target: e.targetId,
			weight: e.weight,
			works: e.works,
		})),
	)

	const maxWorks = $derived(Math.max(1, ...graphNodes.map((n) => n.totalWorks)))
	const maxWeight = $derived(Math.max(1, ...graphLinks.map((l) => l.weight)))

	function nodeRadius(node: GraphNode): number {
		if (node.id === academic.id) return 12
		return 7 + (node.totalWorks / maxWorks) * 9
	}

	function edgeWidth(link: GraphLink): number {
		return 1 + (link.weight / maxWeight) * 2
	}

	function linkEndpointId(value: string | number | GraphNode): string {
		return typeof value === "object" && value !== null ? value.id : String(value)
	}

	function isFocusEdge(link: GraphLink): boolean {
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)
		return s === academic.id || t === academic.id
	}

	let hoveredId = $state<string | null>(null)

	const neighbors = $derived.by(() => {
		const map: Record<string, string[]> = {}
		for (const e of graph?.edges ?? []) {
			;(map[e.sourceId] ??= []).push(e.targetId)
			;(map[e.targetId] ??= []).push(e.sourceId)
		}
		return map
	})

	function isDimmed(node: GraphNode): boolean {
		if (!hoveredId) return false
		if (node.id === hoveredId) return false
		return !neighbors[hoveredId]?.includes(node.id)
	}

	function edgeOpacity(link: GraphLink): number {
		const base = isFocusEdge(link) ? 0.45 : 0.3
		if (!hoveredId) return base
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)
		if (s === hoveredId || t === hoveredId) return 0.65
		return 0.05
	}

	function nodeOpacity(node: GraphNode): number {
		return isDimmed(node) ? 0.25 : 1
	}

	let selectedEdge = $state<{ works: CollaborationWorkRefDTO[]; coauthor: string } | null>(null)
	let edgeDialogOpen = $state(false)

	function openEdge(link: GraphLink) {
		const s = linkEndpointId(link.source)
		const t = linkEndpointId(link.target)
		const edge = graph?.edges.find(
			(e) => (e.sourceId === s && e.targetId === t) || (e.sourceId === t && e.targetId === s),
		)
		if (!edge) return
		const coauthorId = s === academic.id ? t : s
		const coauthorNode = graphNodes.find((n) => n.id === coauthorId)
		selectedEdge = { works: edge.works, coauthor: coauthorNode?.displayName ?? "Coautor" }
		edgeDialogOpen = true
	}

	function shortLabel(name: string): string {
		return name.length > 24 ? `${name.slice(0, 22)}…` : name
	}
</script>

{#if query.isPending}
	<div class="flex items-center justify-center py-16">
		<Loader class="size-5 animate-spin text-corp-gray" />
	</div>
{:else if query.isError}
	<div class="flex flex-col items-center py-14 text-center">
		<CircleAlert class="size-6 text-red-500" />
		<p class="mt-2 text-sm text-corp-gray">Error al cargar la red de colaboración.</p>
	</div>
{:else if !graph || graph.nodes.length <= 1}
	<div class="flex flex-col items-center py-16 text-center">
		<div class="mb-3 flex size-12 items-center justify-center rounded-full bg-corp-blue/5">
			<Network class="size-5 text-corp-blue/60" />
		</div>
		<p class="text-sm text-[#1A1A1A]">No hay colaboraciones internas registradas.</p>
		<p class="mt-1 max-w-sm text-xs text-corp-gray">
			Este académico no comparte publicaciones con otros académicos de la facultad.
		</p>
	</div>
{:else}
	<Chart height={560}>
		{#snippet children({ context })}
			<Layer>
				<ForceSimulation
					forces={{
						link: forceLink<GraphNode, GraphLink>(graphLinks)
							.id((d) => d.id)
							.distance(110)
							.strength(0.35),
						charge: forceManyBody<GraphNode>().strength(-220).distanceMax(500),
						collide: forceCollide<GraphNode>().radius((d) => nodeRadius(d) + 6),
						center: forceCenter(context.width / 2, context.height / 2),
					}}
					data={{ nodes: graphNodes, links: graphLinks }}
					static
					cloneNodes
				>
					{#snippet children({ nodes, linkPositions })}
						{#each graphLinks as link, i (i)}
							<Link
								data={link}
								{...linkPositions[i]}
								type="straight"
								stroke-width={edgeWidth(link)}
								opacity={edgeOpacity(link)}
								class={isFocusEdge(link) ? "stroke-corp-blue" : "stroke-corp-gray"}
								stroke-linecap="round"
								onclick={() => openEdge(link)}
							/>
						{/each}

						{#each nodes as node (node.id)}
							<Circle
								cx={node.x}
								cy={node.y}
								r={nodeRadius(node)}
								fill={node.id === academic.id ? "#0075B4" : "#D8E6EF"}
								stroke={node.id === academic.id ? "#0075B4" : "#B9CFDF"}
								stroke-width={2}
								opacity={nodeOpacity(node)}
								class="cursor-pointer"
								onpointermove={(e) => {
									hoveredId = node.id
									context.tooltip.show(e, node)
								}}
								onpointerleave={() => {
									hoveredId = null
									context.tooltip.hide()
								}}
							/>
							<text
								x={node.x ?? 0}
								y={(node.y ?? 0) + nodeRadius(node) + 14}
								text-anchor="middle"
								class="fill-corp-gray text-[11px] font-medium"
								opacity={nodeOpacity(node)}
							>
								{node.id === academic.id ? "Este académico" : shortLabel(node.displayName)}
							</text>
						{/each}
					{/snippet}
				</ForceSimulation>
			</Layer>

			<Tooltip.Root variant="none">
				{#if context.tooltip.data}
					<div
						class="pointer-events-none rounded-lg border border-corp-gray/20 bg-white px-3 py-2 shadow-lg"
					>
						<p class="text-[13px] font-semibold text-[#1A1A1A]">
							{context.tooltip.data.displayName}
						</p>
						<p class="mt-0.5 text-xs text-corp-gray">
							{context.tooltip.data.department}
						</p>
						<p class="mt-0.5 text-xs text-corp-gray">
							{context.tooltip.data.totalWorks} publicaciones
						</p>
					</div>
				{/if}
			</Tooltip.Root>
		{/snippet}
	</Chart>
{/if}

<EdgeWorksDialog
	bind:open={edgeDialogOpen}
	works={selectedEdge?.works ?? null}
	coauthor={selectedEdge?.coauthor ?? null}
/>
