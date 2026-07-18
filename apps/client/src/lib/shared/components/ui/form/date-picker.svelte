<script lang="ts">
	import type { FieldElementProps } from "@formisch/svelte"
	import type { DateValue } from "@internationalized/date"

	import { DatePicker } from "bits-ui"
	import { CalendarDate, parseDate } from "@internationalized/date"
	import { Calendar as CalendarIcon } from "@lucide/svelte"

	import InputLabel from "./label.svelte"
	import InputErrors from "./errors.svelte"

	interface Props extends Partial<FieldElementProps> {
		class?: string
		label?: string
		hint?: string
		placeholder?: string
		required?: boolean
		input: unknown
		errors: [string, ...string[]] | null
	}

	let {
		class: className,
		label,
		hint,
		placeholder = "Seleccionar fecha",
		name,
		required,
		input,
		errors,
		...fieldProps
	}: Props = $props()

	let hiddenRef: HTMLInputElement | undefined = $state()
	let selected: CalendarDate | undefined = $state()
	let open = $state(false)

	$effect(() => {
		if (typeof input === "string" && /^\d{4}-\d{2}-\d{2}$/.test(input)) {
			try {
				selected = parseDate(input)
				return
			} catch {
				/* ignore */
			}
		}
		selected = undefined
	})

	function handleDateChange(d: DateValue | undefined) {
		const cd = d instanceof CalendarDate ? d : undefined
		selected = cd
		const val = cd ? cd.toString() : ""
		if (hiddenRef) {
			hiddenRef.value = val
			hiddenRef.dispatchEvent(new Event("input", { bubbles: true }))
			hiddenRef.dispatchEvent(new Event("change", { bubbles: true }))
		}
	}
</script>

<div class={["grid gap-1.5", className]}>
	<InputLabel {name} {label} {hint} {required} />

	<input type="hidden" bind:this={hiddenRef} {...fieldProps} {name} {required} />

	<DatePicker.Root
		value={selected}
		onValueChange={handleDateChange}
		bind:open
		locale="es-CL"
		closeOnDateSelect={true}
		weekdayFormat="short"
		fixedWeeks={true}
	>
		<DatePicker.Input
			class="flex h-10 w-full cursor-text select-none items-center rounded-lg border bg-white px-3 text-sm text-[#1A1A1A] outline-none transition-colors focus-within:border-corp-blue/50 focus-within:ring-2 focus-within:ring-corp-blue/10 {errors
				? 'border-red-500'
				: 'border-corp-gray/20'}"
		>
			{#snippet children({ segments })}
				{#each segments as { part, value }, i (part + i)}
					<div class="inline-block select-none">
						<DatePicker.Segment
							{part}
							class="rounded px-0.5 outline-none focus:bg-corp-blue/10 focus:text-corp-blue aria-[valuetext=Empty]:text-corp-gray/50"
						>
							{value}
						</DatePicker.Segment>
					</div>
				{/each}
				<DatePicker.Trigger
					class="ml-auto flex size-7 shrink-0 items-center justify-center rounded-md text-corp-gray/50 transition-colors hover:bg-corp-gray/5 hover:text-corp-gray/80"
				>
					<CalendarIcon class="size-4" />
				</DatePicker.Trigger>
			{/snippet}
		</DatePicker.Input>

		<DatePicker.Portal>
			<DatePicker.Content
				sideOffset={6}
				class="z-50 rounded-xl border border-corp-gray/20 bg-white p-3 shadow-lg"
			>
				<DatePicker.Calendar>
					{#snippet children({ months, weekdays })}
						<DatePicker.Header class="flex items-center justify-between gap-2">
							<DatePicker.MonthSelect
								class="flex-1 rounded-lg border border-corp-gray/20 px-2 py-1.5 text-sm text-[#1A1A1A] outline-none transition-colors focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
							<DatePicker.YearSelect
								class="w-20 rounded-lg border border-corp-gray/20 px-2 py-1.5 text-sm text-[#1A1A1A] outline-none transition-colors focus:border-corp-blue/50 focus:ring-2 focus:ring-corp-blue/10"
							/>
						</DatePicker.Header>

						{#each months as month (month.value)}
							<DatePicker.Grid class="w-full border-collapse">
								<DatePicker.GridHead>
									<DatePicker.GridRow class="mb-1 flex">
										{#each weekdays as day (day)}
											<DatePicker.HeadCell
												class="flex size-8 items-center justify-center text-xs font-normal text-corp-gray/60"
											>
												{day.slice(0, 2)}
											</DatePicker.HeadCell>
										{/each}
									</DatePicker.GridRow>
								</DatePicker.GridHead>
								<DatePicker.GridBody>
									{#each month.weeks as weekDates (weekDates)}
										<DatePicker.GridRow class="flex">
											{#each weekDates as date (date)}
												<DatePicker.Cell
													{date}
													month={month.value}
													class="relative flex size-8 items-center justify-center p-0 text-sm"
												>
													<DatePicker.Day
														class="inline-flex size-7 items-center justify-center rounded-md text-sm text-[#1A1A1A] transition-colors hover:bg-corp-blue/5 data-disabled:text-corp-gray/30 data-selected:bg-corp-blue data-selected:text-white data-focused:ring-1 data-focused:ring-corp-blue/30 data-focused:ring-inset focus-visible:outline-none"
													/>
												</DatePicker.Cell>
											{/each}
										</DatePicker.GridRow>
									{/each}
								</DatePicker.GridBody>
							</DatePicker.Grid>
						{/each}
					{/snippet}
				</DatePicker.Calendar>
			</DatePicker.Content>
		</DatePicker.Portal>
	</DatePicker.Root>

	<InputErrors {name} {errors} />
</div>
