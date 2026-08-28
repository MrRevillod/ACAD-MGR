<script lang="ts">
	import { Gauge, Percent, Sigma, SlidersHorizontal } from "@lucide/svelte"

	import Dialog from "$shared/components/ui/dialog.svelte"

	interface Props {
		open: boolean
	}

	let { open = $bindable(false) }: Props = $props()
</script>

<Dialog
	bind:open
	title="Indicador: Productividad por jornada completa"
	description="Qué mide este gráfico y cómo leer sus valores y filtros."
	class="max-w-2xl"
>
	<div class="space-y-5">
		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Gauge class="size-4 text-corp-blue/60" />
				Qué mide
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				El indicador es el ratio
				<span class="font-medium text-corp-ink"> publicaciones ÷ Σ JCE (Doctor) </span>
				del alcance seleccionado (facultad, departamento o línea de investigación). Muestra la
				producción anual de publicaciones relativa a las horas de jornada completa de los académicos
				con grado de doctor del alcance.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Percent class="size-4 text-corp-blue/60" />
				La división
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				Cada punto del gráfico equivale a
				<span class="font-medium text-corp-ink">
					publicaciones_del_año ÷ Σ JCE (Doctor)
				</span>
				, es decir, el número de publicaciones de ese año relativo a las horas de jornada completa
				del alcance.
			</p>
			<ul class="mt-3 space-y-0.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-gold"
						aria-hidden="true"
					></span>
					<p>Si Σ JCE es 0, el ratio es 0 (guarda contra división por cero).</p>
				</li>
			</ul>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<Sigma class="size-4 text-corp-blue/60" />
				La suma del denominador
			</h3>
			<p class="mt-2 text-sm leading-relaxed text-corp-gray">
				El denominador <span class="font-medium text-corp-ink">Σ JCE (Doctor)</span> es la
				suma de las horas <span class="font-medium text-corp-ink">JCE</span> de los académicos
				con grado de doctor del alcance:
			</p>
			<ul class="mt-2 space-y-0.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Facultad</span> — todos los doctores.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Departamento</span> — doctores de ese
						departamento.
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Línea</span> — doctores cuya línea dominante
						es esa línea.
					</p>
				</li>
			</ul>
			<p class="mt-3 text-sm leading-relaxed text-corp-gray">
				Refleja el roster actual de académicos y, por diseño,
				<span class="font-medium text-corp-ink">no varía por año ni por mes</span>. Puedes
				ver el total en horas bajo el gráfico.
			</p>
		</section>

		<section>
			<h3 class="flex items-center gap-2 text-sm font-semibold text-[#1A1A1A]">
				<SlidersHorizontal class="size-4 text-corp-blue/60" />
				Los filtros
			</h3>
			<ul class="mt-3 space-y-0.5 text-sm text-corp-gray">
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Grado académico</span> — qué publicaciones
						se cuentan en el numerador (todas, o las de autores con grado de doctor o magíster).
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Mes base (Σ JCE)</span> — mes de referencia
						para el año del período (un período va de un mes base a su equivalente del año
						siguiente).
					</p>
				</li>
				<li class="flex items-start gap-2.5">
					<span
						class="mt-1.5 size-2.5 shrink-0 rounded-full bg-corp-blue"
						aria-hidden="true"
					></span>
					<p>
						<span class="font-medium text-corp-ink">Indexación</span> — línea mostrada:
						<span class="font-medium text-corp-ink">Ambas</span> dibuja solo la
						tendencia combinada de todas las publicaciones;
						<span class="font-medium text-corp-ink">WoS</span>
						o
						<span class="font-medium text-corp-ink">Scopus</span> muestran solo esa indexación.
					</p>
				</li>
			</ul>
		</section>
	</div>
</Dialog>
