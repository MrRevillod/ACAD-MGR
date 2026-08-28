<script lang="ts">
	import Select from "$shared/components/ui/select.svelte"

	import { degreeItems, indexationItems, monthItems } from "../productivity-labels"

	let {
		degree = $bindable("all"),
		month = $bindable("1"),
		indexation = $bindable("all"),
	} = $props()
</script>

<div class="space-y-4">
	<div class="space-y-1">
		<label
			for="productivity-degree"
			title="Publicaciones contadas según el grado académico de sus autores"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Grado académico
		</label>
		<Select
			id="productivity-degree"
			items={degreeItems}
			bind:value={degree}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-month"
			title="Mes cuya Σ JCE (Doctor) se usa como divisor del indicador"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Mes base (Σ JCE)
		</label>
		<Select
			id="productivity-month"
			items={monthItems}
			bind:value={month}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
	<div class="space-y-1">
		<label
			for="productivity-indexation"
			title="Publicaciones mostradas según su indexación en WoS o Scopus"
			class="block text-[11px] font-medium uppercase tracking-wider text-corp-gray"
		>
			Indexación
		</label>
		<Select
			id="productivity-indexation"
			items={indexationItems}
			bind:value={indexation}
			placeholder="Seleccionar…"
			class="w-full"
		/>
	</div>
</div>
