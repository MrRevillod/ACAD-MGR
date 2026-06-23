<script lang="ts">
	import { ArrowRight } from "@lucide/svelte"
</script>

<div class="flex min-h-screen items-center justify-center bg-white">
	<div class="text-center">
		<h1 class="text-3xl font-bold tracking-tight text-[#1a1a1a]">ORCID Academic Manager</h1>
		<p class="mt-2 text-sm text-corp-gray">Sistema de gestión académica</p>

		<div class="mt-8">
			<a
				href="/academics/e5f6a7b8-c9d0-1234-ef56-789012345678"
				class="inline-flex items-center gap-2 rounded-lg border border-corp-blue/30 bg-white px-5 py-2.5 text-sm font-medium text-corp-blue transition-colors hover:bg-corp-blue/[0.04]"
			>
				Ver ficha académica
				<ArrowRight class="size-4" />
			</a>
		</div>
	</div>
</div>
