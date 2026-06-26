<script lang="ts">
	import type { Snippet } from "svelte"
	import { tv, type VariantProps } from "tailwind-variants"

	const badgeVariants = tv({
		base: "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold tracking-wide",
		variants: {
			variant: {
				base: "bg-corp-blue/10 text-corp-blue",
				advanced: "bg-corp-yellow/15 text-[#7A6400]",
				default: "bg-corp-gray/10 text-corp-gray",
			},
		},
		defaultVariants: {
			variant: "default",
		},
	})

	type Variants = VariantProps<typeof badgeVariants>

	let {
		variant = "default",
		class: className = "",
		children,
		...restProps
	}: { variant?: Variants["variant"]; class?: string; children: Snippet } & Record<
		string,
		unknown
	> = $props()
</script>

<span class={badgeVariants({ variant, className })} {...restProps}>
	{@render children?.()}
</span>
