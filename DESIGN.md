---
version: alpha
name: UCT Academic Platform
description: Sistema de gestión académica UCT

colors:
  primary: "#0075B4"
  corp-blue: "#0075B4"
  corp-white: "#FFFFFF"
  corp-gray: "#878787"
  corp-yellow: "#EDC500"

typography:
  fontFamily: "Vista Sans, system-ui, sans-serif"
  h1:
    fontSize: 1.125rem
    fontWeight: 600
    lineHeight: 1.3
  section-title:
    fontSize: 0.75rem
    fontWeight: 600
    letterSpacing: 0.05em
    textTransform: uppercase
    color: "{colors.corp-blue}"
  label:
    fontSize: 0.75rem
    fontWeight: 500
    letterSpacing: 0.025em
    textTransform: uppercase
    color: "{colors.corp-gray}"
  value:
    fontSize: 0.9375rem
    fontWeight: 500
    color: "#1A1A1A"
  body:
    fontSize: 0.875rem
    fontWeight: 400
  meta:
    fontSize: 0.75rem
    fontWeight: 400
  badge-label:
    fontSize: 0.75rem
    fontWeight: 600
    letterSpacing: 0.025em

rounded:
  md: 8px
  lg: 12px
  full: 9999px

spacing:
  container: max-w-7xl
  page-x: 1rem
  page-y: 2rem
  section-padding: 1.5rem
  section-gap: 1.5rem
  grid-col-gap: 2rem
  grid-row-gap: 1rem
  label-value-gap: 0.25rem
  avatar-size: 6rem

components:
  sidebar:
    backgroundColor: "{colors.corp-blue}"
    textColor: "{colors.corp-white}"
    rounded: "{rounded.lg}"
  card:
    backgroundColor: "{colors.corp-white}"
    rounded: "{rounded.lg}"
    padding: "{spacing.section-padding}"
  section-title:
    textColor: "{colors.corp-blue}"
  badge:
    rounded: "{rounded.full}"
  badge-base:
    backgroundColor: "{colors.corp-blue}10"
    textColor: "{colors.corp-blue}"
  badge-advanced:
    backgroundColor: "{colors.corp-yellow}15"
    textColor: "#7A6400"
  timeline-dot-base:
    backgroundColor: "{colors.corp-blue}"
  timeline-dot-advanced:
    backgroundColor: "{colors.corp-yellow}"
  timeline-line:
    backgroundColor: "{colors.corp-gray}20"
  back-link:
    textColor: "{colors.corp-gray}"
  back-link-hover:
    textColor: "{colors.corp-blue}"
  avatar:
    size: 6rem
    backgroundColor: "{colors.corp-white}10"

notes:
  color-inline-values:
    text-primary: "#1A1A1A"
    badge-advanced-text: "#7A6400"
  unpublished:
    - "iconColor: corp-blue (no oficial en spec, se usa iconSize: 16px en section-title)"
    - "ring: 2px solid white/15 en avatar (no oficial en spec)"
---

## Overview

Perfil académico institucional de la Facultad de Ingeniería, Universidad Católica de Temuco. Diseño plano, informacional, sin sombras ni dark mode. La interfaz prioriza la legibilidad de datos sobre la decoración.

## Colors

La paleta son 4 colores definidos en `layout.css` vía `@theme` de Tailwind v4. Los nombres `corp-*` reflejan su uso directo en las clases utilitarias.

- **primary / corp-blue (#0075B4):** Azul corporativo. Sidebar, títulos de sección, badges de grado base, hover de links, borde de botón en landing. Domina la interfaz.
- **corp-gray (#878787):** Gris secundario. Labels de campo (`text-xs uppercase tracking-wider`), metadata, bordes de card, separadores.
- **corp-yellow (#EDC500):** Amarillo acento. Iconos en sidebar, badge de grado avanzado.
- **neutral / corp-white (#FFFFFF):** Fondo principal. Cards, sidebar text.

Dos valores inline (no tokenizados en `@theme`):
- `#1A1A1A` — color de los valores de datos en cards.
- `#7A6400` — texto del badge de grado avanzado.

## Typography

**Vista Sans** como fuente corporativa. Humanista, legible en tamaños pequeños, con una presencia institucional que no compite con los datos. El contraste semántico clave está entre labels y values:

- **label:** `text-xs font-medium tracking-wide uppercase` en gris. Discreto, técnico.
- **value:** `text-[15px] font-medium` en `#1A1A1A`. Ligeramente mayor que el label, buen contraste semántico.
- **section-title:** `text-xs font-semibold tracking-widest uppercase` en azul. Encabeza cada bloque.
- **h1:** Nombre del académico en sidebar, `text-lg font-semibold`.

## Layout

Dos columnas asimétricas en perfil académico:

- **Sidebar** (320px fijos): Fondo azul, texto blanco. Avatar circular, datos personales (RUT, ORCID, nacionalidad, fecha nacimiento, sexo, fecha ingreso).
- **Main** (resto): Cards blancas con borde `1px corp-gray/20`, `rounded-xl`. Tres secciones apiladas con `space-y-6`: Información Laboral (grid 2 cols), Categorización Académica (grid 3 cols), Grados Académicos (timeline vertical).

## Components

Los componentes se modelan según los bloques observados en `academics/[id]/+page.svelte`. Cada badge y dot de timeline tiene variante base/advanced separada para permitir colores distintos. Hover states se expresan como entradas separadas (e.g. `back-link-hover`).
