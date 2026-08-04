<script lang="ts">
	import { CircleDot, Hash, Layers, Scale, Sparkles, Tag } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"

	interface Props {
		open: boolean
	}

	let { open = $bindable(false) }: Props = $props()
</script>

<Dialog
	bind:open
	title="Cómo leer esta red"
	description="Significado de los filtros, colores y pesos del grafo."
	class="max-w-3xl"
>
	<div class="space-y-5">
		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<CircleDot class="size-4 text-corp-blue/60" />
				Nodos
			</h3>
			<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2">
					<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span>
						<span class="font-medium text-[#1A1A1A]">Este académico</span> — el foco de la
						vista.
					</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-[#D8E6EF]"></span>
					<span>
						<span class="font-medium text-[#1A1A1A]">Coautor</span> — alguien con quien comparte
						publicaciones. El tamaño del nodo refleja su cantidad de publicaciones.
					</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-green-500"></span>
					<span>
						<span class="font-medium text-[#1A1A1A]">Posible colaboración</span> — un académico
						sin publicaciones en común que comparte intereses de investigación.
					</span>
				</li>
			</ul>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Hash class="size-4 text-corp-blue/60" />
				Pesos y aristas
			</h3>
			<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2">
					<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"></span>
					<span>
						<span class="font-medium text-[#1A1A1A]">Coautoría</span> — publicaciones compartidas
						entre ambos académicos.
					</span>
				</li>
				<li class="flex items-start gap-2">
					<span class="mt-1.5 size-2.5 shrink-0 rounded-full bg-green-500"></span>
					<span>
						<span class="font-medium text-[#1A1A1A]">Sugerencia</span> — número de publicaciones
						que cumplen el mínimo de coincidencias por publicación.
					</span>
				</li>
				<li class="flex items-start gap-2">
					<Scale class="mt-0.5 size-4 shrink-0 text-corp-gray" />
					<span>
						Con
						<span class="font-medium text-[#1A1A1A]">Mostrar peso en aristas</span> se muestra
						el número sobre cada conexión.
					</span>
				</li>
			</ul>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Tag class="size-4 text-corp-blue/60" />
				Términos
			</h3>
			<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2">
					<span
						class="mt-1 rounded-full bg-green-600/10 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-green-600"
					>
						topic
					</span>
					<span>
						Área temática (taxonomía OpenAlex) asignada a una publicación, agrupada en
						subfield y línea de investigación. Si dos académicos comparten un topic con
						porcentaje suficiente, cuenta como coincidencia.
					</span>
				</li>
				<li class="flex items-start gap-2">
					<span
						class="mt-1 rounded-full bg-corp-gray/10 px-2 py-0.5 text-[10px] font-semibold uppercase tracking-wide text-corp-gray"
					>
						keyword
					</span>
					<span>
						Palabra clave extraída de la publicación. Cuenta como coincidencia con el
						mismo umbral de porcentaje que los topics.
					</span>
				</li>
			</ul>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Layers class="size-4 text-corp-blue/60" />
				Filtros
			</h3>
			<ul class="mt-2 space-y-1.5 text-sm text-corp-gray">
				<li>
					<span class="font-medium text-[#1A1A1A]">Mostrar colaboraciones</span> — oculta o
					muestra las coautorías.
				</li>
				<li>
					<span class="font-medium text-[#1A1A1A]">Mostrar sugerencias</span> — oculta o muestra
					los posibles colaboradores.
				</li>
				<li>
					<span class="font-medium text-[#1A1A1A]">Separación</span> — distancia entre nodos.
				</li>
				<li>
					<span class="font-medium text-[#1A1A1A]">Zoom</span> — acercamiento de la vista.
				</li>
				<li>
					<span class="font-medium text-[#1A1A1A]">Porcentaje de coincidencia ≥</span> — porcentaje
					mínimo para que un tópico o keyword cuente como coincidencia. Aplica a ambos tipos.
				</li>
				<li>
					<span class="font-medium text-[#1A1A1A]">Coincidencias por publicación ≥</span> —
					cuántos tópicos/keywords compartidos debe tener una publicación para contar como coincidencia.
					El peso de una sugerencia es el número de publicaciones que cumplen esa condición.
				</li>
			</ul>
		</section>

		<div class="flex items-start gap-2 rounded-lg bg-corp-blue/5 px-3 py-2.5">
			<Sparkles class="mt-0.5 size-4 shrink-0 text-corp-blue" />
			<p class="text-xs text-corp-gray">
				Haz clic en una arista de coautoría para ver las publicaciones compartidas, o en una
				sugerencia para ver los tópicos y keywords en común con su porcentaje.
			</p>
		</div>
	</div>
</Dialog>
