<script lang="ts">
	interface Props {
		total: number
		wos: number
		scopus: number
		unindexed: number
	}

	let { total, wos, scopus, unindexed }: Props = $props()
</script>

<div class="grid grid-cols-2 gap-3 lg:grid-cols-4">
	<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
		<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">
			Total publicaciones
		</p>
		<p class="mt-2 text-[28px] font-semibold leading-none text-corp-ink tabular-nums">
			{total}
		</p>
	</div>
	<div class="rounded-xl border border-corp-blue/30 bg-corp-blue/5 p-5">
		<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">WoS</p>
		<p class="mt-2 text-[28px] font-semibold leading-none text-corp-blue tabular-nums">{wos}</p>
	</div>
	<div class="rounded-xl border border-corp-yellow/30 bg-corp-yellow/5 p-5">
		<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Scopus</p>
		<p class="mt-2 text-[28px] font-semibold leading-none text-corp-gold tabular-nums">
			{scopus}
		</p>
	</div>
	<div class="rounded-xl border border-corp-gray/20 bg-white p-5">
		<p class="text-[11px] font-medium uppercase tracking-wider text-corp-gray">Sin indexar</p>
		<p class="mt-2 text-[28px] font-semibold leading-none text-corp-gray tabular-nums">
			{unindexed}
		</p>
	</div>
</div>
