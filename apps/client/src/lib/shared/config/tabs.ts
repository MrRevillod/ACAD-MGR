export const CONFIG_TABS = [
	{ id: "general", label: "General", href: "/admin/config?tab=general" },
	{ id: "categories", label: "Categorías", href: "/admin/config?tab=categories" },
	{ id: "options", label: "Opciones", href: "/admin/config?tab=options" },
	{ id: "positions", label: "Cargos", href: "/admin/config?tab=positions" },
] as const

export type ConfigTabId = (typeof CONFIG_TABS)[number]["id"]
