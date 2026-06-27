# AGENTS.md — ORCID Academic Manager

## Quick commands

```bash
make lint              # cargo clippy + pnpm lint + prettier check
make fmt               # cargo fmt + pnpm format
make run               # docker compose up
make detach            # docker compose up -d
make clean             # docker compose down -v
make clean-db          # drops schema + re-runs sqlx migrations
make db                # psql into postgres container
make migration name=x  # sqlx migrate add --source config/migrations x
make machete           # cargo machete (unused deps)
```

## Architecture

| Layer | Tech | Details |
|-------|------|---------|
| Server | Rust edition 2024, Sword web framework | DI via `#[injectable]`, `#[controller]`, `Module` trait, `sword::prelude::*` |
| Client | SvelteKit + Svelte 5 runes + Tailwind v4 | `adapter-static` with SPA fallback, pnpm@11 |
| DB | PostgreSQL 16 (Docker) | sqlx migrations, no ORM |

## Server conventions

- **Module registration**: 4 modules in `main.rs` order — `SharedModule` → `UniversityModule` → `AcademicModule` → `AuthModule`
- **DI pattern**: `#[injectable]` on services/repos, fields `Arc<T>`, constructors auto-generated
- **Controllers**: `#[controller(kind = ControllerKind::Web, path = "...")]`, use `#[get]` / `#[post]`, return `WebResult<T>`
- **Validation**: `validator` crate with `#[derive(Validate)]`, `#[validate(custom(function = "..."))]` for complex checks
- **Auth**: `#[interceptor(SessionCheck)]` on routes requiring session; JWT via `JsonWebTokenService`
- **Value objects**: `CLf64` (Chilean-format float), `Country`, custom `Id<T>` types
- **Entity builder**: `bon` crate — `Academic::builder().field(value).build()`
- **No tests exist** anywhere in the Rust codebase (`#[cfg(test)]` absent)
- **OpenAPI specs**: 3 YAML files in `apps/server/docs/*.yaml`, served via Swagger UI (registered in `config/config.toml` web.openapi)

## Client conventions

- **pnpm strict**: `engine-strict=true`, `ignore-scripts=true`, lockfile required
- **Svelte 5 runes forced** in `vite.config.ts`
- **Tailwind v4** with `@theme` variables in `layout.css`
- **Design tokens** in `DESIGN.md`: primary `#0075B4`, font "Vista Sans" at `static/fonts/`
- **Formatter**: Prettier with tabs, no semicolons, single quotes false, trailing commas
- **TanStack Query v6** (`@tanstack/svelte-query`): `createQuery` uses Svelte 5 runes internally (`$state.raw` Proxy), **not** Svelte stores — never use `$query` prefix. Access properties directly: `query.data`, `query.isPending`, `query.error`.
- **TanStack Table v9 beta** (`@tanstack/svelte-table`): `ColumnDef` requires 2-3 type args in v9. Use `ColumnDef<any, TData, unknown>[]` in generic components. `createTable` with `tableFeatures()` — use `getAllCells()` instead of `getVisibleCells()` unless column visibility feature is registered.
- **TanStack Table v9 state**: Do NOT use external `$state` for pagination/sorting with `state` getters + `onPaginationChange`. Instead, pass a selector to `createTable(options, (state) => ({ pagination: state.pagination, sorting: state.sorting }))` and read from `table.state.pagination`. Navigation methods (`nextPage()`, `setPageIndex()`) update internal atoms directly.
- **Forms (create/update)**: Use `valibot` + `@formisch/svelte`. Todo form que crea o actualiza datos debe:
  1. Definir un schema valibot — `import * as v from "valibot"` con `v.object({ ... })`, usar `v.pipe()` para cadenas de validación.
  2. Extraer el tipo TS del schema — `type FormData = v.InferInput<typeof schema>`.
  3. Crear el store del form con `createForm({ schema, initialInput? })`.
  4. Envolver en `<Form of={form}>` + `<Field of={form} path={['campo']}>` usando snippets de Svelte 5:
     ```svelte
     <Field of={form} path={['email']}>
       {#snippet children(field)}
         <input {...field.props} value={field.input} />
         {#if field.errors}<p>{field.errors[0]}</p>{/if}
       {/snippet}
     </Field>
     ```
  5. Manejar submit asíncrono en `onsubmit={(output) => mutation.mutate(output)}`.
  6. Para submit programático (ej. botón fuera del `<Form>`), usar `handleSubmit(form, callback)`.
  Search bars, filtros e inputs inline-read-only están exentos.
- **Value objects**: Encapsular valores de dominio con semántica propia en tipos dedicados en lugar de usar `string`, `number` o funciones inline tipo `formatSex(sex)`. Esto даёт type safety, validación centralizada y self-documenting code. Ejemplos actuales: `Country` (сódigo ISO + nombre), `CLf64` (formato chileno de float), `Id<T>` (ID tipado sin `T` bounds). Buscar refactorizar: enums de texto (`Sex`, `AcademicCategory`, estados), strings formateados (rut, phone), monedas, fechas con timezone, identificadores compuestos.
- **Thin pages**: Route files are skeletons — only TanStack Query calls, `$derived` state, and rendering of domain components. No inline forms, no large template blocks, no business logic in `+page.svelte`. Everything is delegated to its domain component.

## Database

- **Migrations**: `apps/server/config/migrations/` — timestamp-prefixed SQL files
- **Schema**: 7 tables (countries, faculties, departments, careers, academic_work_positions, academic_categories, academic_category_options) + users, sessions, academics, degrees
- **Env vars**: from `.env` — `POSTGRES_DATABASE_URL` (Docker) / `LOCAL_POSTGRES_DATABASE_URL` (host)
- **config.toml**: `postgres-db.migrations_path = "config/migrations"`

## Pitfalls

- `regex` crate does NOT support lookahead/lookbehind; use `#[validate(custom)]` for password complexity
- `CLf64` derefs to `f64` via `*value` when binding to sqlx queries
- `sqlx::query_as::<_, Academic>("SELECT * FROM academics WHERE ...")` auto-maps column names to struct fields
- Docker Compose has 4 services; healthcheck dependencies: server ← postgres, nginx ← server + client
- CI triggers only on path-filtered pushes (main/PR) — check `.github/workflows/` for exact paths
