<script lang="ts">
	import { useQuery } from "$shared/http/tanstack"
	import { GraduationCap, Users, Tags, ListOrdered, Briefcase } from "@lucide/svelte"

	import { usersService } from "$users/service"
	import { optionService } from "$options/service"
	import { academicService } from "$academics/service"
	import { categoryService } from "$categories/service"
	import { positionService } from "$work-positions/service"

	const academicsQuery = useQuery(() => ({
		queryKey: ["admin", "academics"],
		queryFn: () => academicService.list(),
	}))

	const usersQuery = useQuery(() => ({
		queryKey: ["admin", "users"],
		queryFn: () => usersService.list(),
	}))

	const categoriesQuery = useQuery(() => ({
		queryKey: ["admin", "categories"],
		queryFn: () => categoryService.list(),
	}))

	const optionsQuery = useQuery(() => ({
		queryKey: ["admin", "options"],
		queryFn: () => optionService.list(),
	}))

	const positionsQuery = useQuery(() => ({
		queryKey: ["admin", "positions"],
		queryFn: () => positionService.list(),
	}))
</script>

<div>
	<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-5">
		<a
			href="/admin/categories"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Tags class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{categoriesQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Categorías</p>
		</a>

		<a
			href="/admin/options"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<ListOrdered class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{optionsQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Opciones</p>
		</a>

		<a
			href="/admin/positions"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Briefcase class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{positionsQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Cargos</p>
		</a>

		<a
			href="/academics"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<GraduationCap class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{academicsQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Académicos</p>
		</a>

		<a
			href="/admin/users"
			class="rounded-xl border border-corp-gray/20 bg-white p-5 transition-colors hover:border-corp-blue/30"
		>
			<div
				class="flex size-10 items-center justify-center rounded-lg bg-corp-blue/10 text-corp-blue"
			>
				<Users class="size-5" />
			</div>
			<p class="mt-3 text-lg font-semibold text-[#1A1A1A]">
				{usersQuery.data?.length ?? "—"}
			</p>
			<p class="text-xs font-medium tracking-wide uppercase text-corp-gray">Usuarios</p>
		</a>
	</div>
</div>
