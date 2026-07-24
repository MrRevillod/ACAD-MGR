<script lang="ts">
	import { page } from "$app/state"
	import { goto } from "$app/navigation"
	import { Construction } from "@lucide/svelte"

	import Button from "$shared/components/ui/button.svelte"

	/* -- disabled temporarily --
	import type { SelfUpdateDTO } from "$academics/dtos"
	import { toast } from "svelte-sonner"
	import { Form, Field } from "@formisch/svelte"
	import { createForm, reset } from "@formisch/svelte"
	import { useMutation, useQuery } from "$shared/http/tanstack"
	import { Loader, CircleAlert, CircleCheck, RefreshCw, Send } from "@lucide/svelte"
	import { SexValue } from "$shared/value-objects/sex.value"
	import { countryItems } from "$shared/countries"
	import { academicService } from "$academics/service"
	import { selfUpdateAcademicDTOSchema } from "$academics/dtos"
	import Select from "$shared/components/ui/form/select.svelte"
	import TextInput from "$shared/components/ui/form/text-input.svelte"
	import DatePicker from "$shared/components/ui/form/date-picker.svelte"
	*/

	const _token = $derived(page.url.searchParams.get("token") ?? "")
</script>

<div
	class="mx-auto flex min-h-dvh max-w-2xl flex-col items-center justify-center px-4 py-12 text-center"
>
	<Construction class="size-12 text-amber-500" />
	<h1 class="mt-4 text-lg font-semibold text-[#1A1A1A]">Edición de perfil no disponible</h1>
	<p class="mt-2 max-w-sm text-sm text-corp-gray">
		Esta funcionalidad estará disponible próximamente. Disculpa las molestias.
	</p>
	<Button variant="primary" class="mt-6" onclick={() => goto("/")}>Volver al inicio</Button>
</div>
