<script lang="ts">
	import { authStore } from "$auth/store.svelte"
	import { Users, Settings, ArrowRight } from "@lucide/svelte"
</script>

<div class="mx-auto flex h-full max-w-[1600px] items-center justify-center px-4 sm:px-6 lg:px-8">
	<div class="max-w-xl text-center">
		<div
			class="mb-6 inline-flex items-center gap-2 rounded-full border border-corp-blue/10 bg-corp-blue/4 px-4 py-1.5"
		>
			<span class="text-xs font-semibold tracking-widest uppercase text-corp-blue"
				>Facultad de Ingeniería · UCT</span
			>
		</div>

		<h1 class="text-3xl font-bold tracking-tight text-[#1A1A1A] sm:text-4xl">
			Gestión Académica
		</h1>
		<p class="mt-4 text-base leading-relaxed text-corp-gray">
			Sistema de administración y consulta de la información académica de los docentes de la
			Facultad de Ingeniería. Visualiza fichas, categorías, grados y más.
		</p>

		<div class="mt-10 flex items-center justify-center gap-4">
			<a
				href="/academics"
				class="inline-flex items-center gap-2 rounded-lg bg-corp-blue px-5 py-2.5 text-sm font-medium text-white transition-colors hover:bg-corp-blue/90"
			>
				<Users class="size-4" />
				Ver Académicos
				<ArrowRight class="size-4" />
			</a>

			{#if authStore.isAuthenticated()}
				<a
					href="/admin"
					class="inline-flex items-center gap-2 rounded-lg border border-corp-gray/20 px-5 py-2.5 text-sm font-medium text-corp-gray transition-colors hover:border-corp-gray/30 hover:text-[#1A1A1A]"
				>
					<Settings class="size-4" />
					Administrar
				</a>
			{/if}
		</div>
	</div>
</div>
