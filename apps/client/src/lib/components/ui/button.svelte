<script lang="ts">
	import { Button as ButtonPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"
	import { tv, type VariantProps } from "tailwind-variants"

	const buttonVariants = tv({
		base: "inline-flex items-center justify-center gap-1.5 rounded-lg text-sm font-medium outline-none transition-colors active:scale-[0.96] disabled:pointer-events-none disabled:opacity-50",
		variants: {
			variant: {
				primary:
					"bg-corp-blue text-white hover:bg-corp-blue/90 focus-visible:ring-2 focus-visible:ring-corp-blue/30",
				secondary:
					"border border-corp-gray/20 bg-white text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A] focus-visible:ring-2 focus-visible:ring-corp-gray/20",
				ghost:
					"text-corp-gray hover:bg-corp-gray/5 hover:text-[#1A1A1A] focus-visible:ring-2 focus-visible:ring-corp-gray/20",
				danger:
					"bg-red-600 text-white hover:bg-red-700 focus-visible:ring-2 focus-visible:ring-red-600/30",
			},
			size: {
				default: "h-10 px-4 py-2",
				sm: "h-8 rounded-md px-3 text-xs",
				lg: "h-12 rounded-lg px-6 text-base",
				icon: "size-10",
			},
		},
		defaultVariants: {
			variant: "primary",
			size: "default",
		},
	})

	type Variants = VariantProps<typeof buttonVariants>

	let {
		ref = $bindable(null),
		variant = "primary",
		size = "default",
		class: className = "",
		children,
		...restProps
	}: ButtonPrimitive.RootProps & Variants & { class?: string; children: Snippet } = $props()
</script>

<ButtonPrimitive.Root bind:ref class={buttonVariants({ variant, size, className })} {...restProps}>
	{@render children?.()}
</ButtonPrimitive.Root>
