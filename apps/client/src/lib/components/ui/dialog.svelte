<script lang="ts">
	import { Dialog as DialogPrimitive, type WithoutChild } from "bits-ui"
	import { X } from "@lucide/svelte"
	import type { Snippet } from "svelte"

	type Props = DialogPrimitive.RootProps & {
		title: string
		description?: string
		class?: string
		contentProps?: WithoutChild<DialogPrimitive.ContentProps>
		children: Snippet
	}

	let {
		open = $bindable(false),
		title,
		description,
		class: className = "",
		contentProps = {},
		children,
		...restProps
	}: Props = $props()
</script>

<DialogPrimitive.Root bind:open {...restProps}>
	<DialogPrimitive.Portal>
		<DialogPrimitive.Overlay
			class="fixed inset-0 z-40 bg-black/20 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0"
		/>
		<DialogPrimitive.Content
			{...contentProps}
			class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed left-1/2 top-1/2 z-50 w-full max-w-lg -translate-x-1/2 -translate-y-1/2 rounded-xl border border-corp-gray/20 bg-white p-6 shadow-lg {className}"
		>
			<div class="mb-4 flex items-start justify-between">
				<div>
					<DialogPrimitive.Title class="text-lg font-semibold text-[#1A1A1A]">
						{title}
					</DialogPrimitive.Title>
					{#if description}
						<DialogPrimitive.Description class="mt-1 text-sm text-corp-gray">
							{description}
						</DialogPrimitive.Description>
					{/if}
				</div>
				<DialogPrimitive.Close
					class="flex size-8 items-center justify-center rounded-lg text-corp-gray transition-colors hover:bg-corp-gray/5 hover:text-[#1A1A1A]"
				>
					<X class="size-4" />
				</DialogPrimitive.Close>
			</div>
			{@render children?.()}
		</DialogPrimitive.Content>
	</DialogPrimitive.Portal>
</DialogPrimitive.Root>
