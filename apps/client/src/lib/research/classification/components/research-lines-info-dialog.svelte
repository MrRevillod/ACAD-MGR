<script lang="ts">
	import { BookOpen, Columns3, GitFork, Layers } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"

	interface Props {
		open: boolean
	}

	let { open = $bindable(false) }: Props = $props()
</script>

<Dialog
	bind:open
	title="Cómo se clasifica la investigación"
	description="Cómo se organizan las publicaciones en líneas de investigación y qué puedes hacer en este tablero."
	class="max-w-3xl"
>
	<div class="space-y-5">
		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Layers class="size-4 text-corp-blue/60" />
				Una taxonomía en niveles
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				Las publicaciones se clasifican según una taxonomía científica con varios niveles de
				detalle, de lo más general a lo más específico:
			</p>
			<ul class="mt-3 space-y-0.5">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<div>
						<span class="text-sm font-semibold text-corp-ink">Dominio</span>
						<span class="text-sm text-corp-gray">
							— el área más amplia del conocimiento.</span
						>
					</div>
				</li>
				<li class="ml-[7px] border-l border-corp-gray/20 pl-5">
					<span class="text-sm font-semibold text-corp-ink">Campo</span>
					<span class="text-sm text-corp-gray"> — disciplina dentro del dominio.</span>
				</li>
				<li class="ml-[7px] border-l border-corp-gray/20 pl-5">
					<span class="text-sm font-semibold text-corp-ink">Subcampo</span>
					<span class="text-sm text-corp-gray"> — especialidad dentro del campo.</span>
				</li>
				<li class="ml-[7px] border-l border-corp-gray/20 pl-5">
					<span class="text-sm font-semibold text-corp-ink">Tema</span>
					<span class="text-sm text-corp-gray"> — tema concreto de una publicación.</span>
				</li>
			</ul>
			<p class="mt-3 text-sm leading-relaxed text-corp-gray">
				Cada publicación se etiqueta con uno o más temas, y cada tema pertenece a un
				subcampo.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<GitFork class="size-4 text-corp-blue/60" />
				Cómo se relaciona con las líneas
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				Cada <span class="font-medium text-corp-ink">subcampo</span> se asigna a una{" "}
				<span class="font-medium text-corp-ink">línea de investigación</span> de la facultad.
				Como los temas cuelgan de sus subcampos, al asociar un subcampo a una línea, todas las
				publicaciones con temas de ese subcampo quedan agrupadas en esa línea. Así se mide la
				producción de cada línea y se puede ver qué áreas cubre la facultad.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Columns3 class="size-4 text-corp-blue/60" />
				El tablero
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				Cada columna es una línea de investigación y cada tarjeta es un subcampo. Para
				reasignar un subcampo, <span class="font-medium text-corp-ink">arrástralo</span> de
				una columna a otra: sus publicaciones quedarán asociadas a la nueva línea. La
				columna{" "}
				<span class="font-medium text-corp-ink">Sin Asignar</span> reúne los subcampos que aún
				no pertenecen a ninguna línea.
			</p>
		</section>

		<div class="flex items-start gap-2 rounded-lg bg-corp-blue/5 px-3 py-2.5">
			<BookOpen class="mt-0.5 size-4 shrink-0 text-corp-blue" />
			<p class="text-xs text-corp-gray">
				¿Solo quieres cambiar la línea de
				<span class="font-medium text-corp-ink">una publicación</span>? Hazlo en la edición
				de esa publicación: abre la publicación y, en el campo "Línea de investigación" del
				formulario, elige la línea deseada sin afectar al resto del subcampo.
			</p>
		</div>

		<div class="flex items-start gap-2 rounded-lg bg-corp-blue/5 px-3 py-2.5">
			<BookOpen class="mt-0.5 size-4 shrink-0 text-corp-blue" />
			<p class="text-xs text-corp-gray">
				Si una publicación ya tiene una línea asignada manualmente, esa elección tiene
				prioridad sobre la del subcampo.
			</p>
		</div>
	</div>
</Dialog>
