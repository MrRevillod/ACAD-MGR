<script lang="ts">
	import type { TimeSeriesStat } from "../types"

	interface Props {
		journalKind: TimeSeriesStat[]
		option: TimeSeriesStat[]
	}

	let { journalKind, option }: Props = $props()

	const scopus = $derived(
		journalKind.find((s) => s.key === "scopus")?.values.reduce((a, v) => a + v.value, 0) ?? 0,
	)
	const wos = $derived(
		journalKind.find((s) => s.key === "wos")?.values.reduce((a, v) => a + v.value, 0) ?? 0,
	)
	const teaching = $derived(
		option.find((s) => s.key === "teaching")?.values.reduce((a, v) => a + v.value, 0) ?? 0,
	)
	const research = $derived(
		option.find((s) => s.key === "research")?.values.reduce((a, v) => a + v.value, 0) ?? 0,
	)
	const total = $derived(scopus + wos)
</script>

<div class="grid grid-cols-2 gap-4">
	<div class="rounded-lg border border-corp-yellow/30 bg-corp-yellow/5 p-4">
		<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Scopus</p>
		<p class="mt-1 text-3xl font-bold text-corp-yellow">{scopus}</p>
		<p class="mt-1 text-xs text-corp-gray">de {total} publicaciones indexadas</p>
	</div>
	<div class="rounded-lg border border-corp-blue/30 bg-corp-blue/5 p-4">
		<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">WoS</p>
		<p class="mt-1 text-3xl font-bold text-corp-blue">{wos}</p>
		<p class="mt-1 text-xs text-corp-gray">de {total} publicaciones indexadas</p>
	</div>
	<div class="rounded-lg border border-corp-blue/20 bg-corp-blue/5 p-4">
		<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Docencia</p>
		<p class="mt-1 text-3xl font-bold text-corp-blue">{teaching}</p>
		<p class="mt-1 text-xs text-corp-gray">publicaciones</p>
	</div>
	<div class="rounded-lg border border-corp-yellow/20 bg-corp-yellow/5 p-4">
		<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Investigación</p>
		<p class="mt-1 text-3xl font-bold text-corp-yellow">{research}</p>
		<p class="mt-1 text-xs text-corp-gray">publicaciones</p>
	</div>
</div>
