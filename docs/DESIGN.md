---
version: alpha
name: Bangla Keyboard
description: >
  A themable design system inspired by Raycast's design language.
  Covers the desktop app (macOS/Windows) and marketing website.
  All UI is built with semantic theme tokens — no hardcoded values.

colors:
  # Semantic surface tokens (dark theme defaults shown; light overrides below)
  background: "#1A1B1E"
  background-secondary: "#232529"
  background-tertiary: "#2C2E33"
  surface: "#2C2E33"
  surface-hover: "#35373D"
  surface-active: "#3E4047"
  overlay: "rgba(0, 0, 0, 0.5)"

  # Text
  text-primary: "#EBEBEF"
  text-secondary: "#A0A0AB"
  text-tertiary: "#6E6E7A"
  text-inverse: "#1A1B1E"

  # Brand / Accent
  accent: "#FF6363"
  accent-hover: "#FF7A7A"
  accent-active: "#E55555"
  accent-subtle: "rgba(255, 99, 99, 0.12)"

  # Semantic feedback
  success: "#59D499"
  success-subtle: "rgba(89, 212, 153, 0.12)"
  warning: "#FFC531"
  warning-subtle: "rgba(255, 197, 49, 0.12)"
  error: "#FF6363"
  error-subtle: "rgba(255, 99, 99, 0.12)"
  info: "#56C2FF"
  info-subtle: "rgba(86, 194, 255, 0.12)"

  # Borders & separators
  border: "#35373D"
  border-hover: "#4A4C54"
  border-focus: "#56C2FF"
  separator: "rgba(255, 255, 255, 0.06)"

  # Palette (for accent customization and illustrations)
  red: "#FF6363"
  orange: "#FF9F43"
  yellow: "#FFC531"
  green: "#59D499"
  blue: "#56C2FF"
  purple: "#B388FF"
  magenta: "#FF7EB3"

typography:
  display:
    fontFamily: Inter
    fontSize: 36px
    fontWeight: 700
    lineHeight: 1.1
    letterSpacing: -0.02em
  headline-lg:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: 600
    lineHeight: 1.25
    letterSpacing: -0.015em
  headline-md:
    fontFamily: Inter
    fontSize: 20px
    fontWeight: 600
    lineHeight: 1.3
    letterSpacing: -0.01em
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: 400
    lineHeight: 1.5
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: 400
    lineHeight: 1.5
  body-sm:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: 400
    lineHeight: 1.4
  label-lg:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: 500
    lineHeight: 1.3
  label-md:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: 500
    lineHeight: 1.3
  label-sm:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: 500
    lineHeight: 1.2
    letterSpacing: 0.02em
  mono-md:
    fontFamily: JetBrains Mono
    fontSize: 13px
    fontWeight: 400
    lineHeight: 1.5
  mono-sm:
    fontFamily: JetBrains Mono
    fontSize: 11px
    fontWeight: 400
    lineHeight: 1.4

spacing:
  xs: 4px
  sm: 8px
  md: 12px
  lg: 16px
  xl: 24px
  2xl: 32px
  3xl: 48px
  4xl: 64px
  section: 80px

rounded:
  xs: 4px
  sm: 6px
  md: 8px
  lg: 12px
  xl: 16px
  full: 9999px

components:
  # ─── Buttons ───
  button-primary:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.text-inverse}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    height: 36px
    padding: 16px
  button-primary-hover:
    backgroundColor: "{colors.accent-hover}"
  button-primary-active:
    backgroundColor: "{colors.accent-active}"
  button-secondary:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text-primary}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    height: 36px
    padding: 16px
  button-secondary-hover:
    backgroundColor: "{colors.surface-hover}"
  button-ghost:
    backgroundColor: transparent
    textColor: "{colors.text-secondary}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    height: 36px
    padding: 16px
  button-ghost-hover:
    backgroundColor: "{colors.surface-hover}"
    textColor: "{colors.text-primary}"

  # ─── Search Bar ───
  search-bar:
    backgroundColor: "{colors.background-secondary}"
    textColor: "{colors.text-primary}"
    typography: "{typography.body-lg}"
    rounded: "{rounded.lg}"
    height: 48px
    padding: 16px
  search-bar-placeholder:
    textColor: "{colors.text-tertiary}"

  # ─── List Items ───
  list-item:
    backgroundColor: transparent
    textColor: "{colors.text-primary}"
    typography: "{typography.body-md}"
    rounded: "{rounded.md}"
    height: 40px
    padding: 12px
  list-item-hover:
    backgroundColor: "{colors.surface-hover}"
  list-item-active:
    backgroundColor: "{colors.accent-subtle}"
    textColor: "{colors.text-primary}"
  list-item-subtitle:
    textColor: "{colors.text-secondary}"
    typography: "{typography.body-sm}"

  # ─── Sidebar / Navigation ───
  sidebar:
    backgroundColor: "{colors.background}"
    width: 240px
    padding: 8px
  sidebar-item:
    backgroundColor: transparent
    textColor: "{colors.text-secondary}"
    typography: "{typography.label-lg}"
    rounded: "{rounded.md}"
    height: 36px
    padding: 12px
  sidebar-item-active:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text-primary}"
  sidebar-item-hover:
    backgroundColor: "{colors.surface-hover}"
    textColor: "{colors.text-primary}"

  # ─── Action Panel / Context Menu ───
  action-panel:
    backgroundColor: "{colors.background-secondary}"
    rounded: "{rounded.lg}"
    padding: 4px
  action-item:
    backgroundColor: transparent
    textColor: "{colors.text-primary}"
    typography: "{typography.body-md}"
    rounded: "{rounded.sm}"
    height: 36px
    padding: 12px
  action-item-hover:
    backgroundColor: "{colors.surface-hover}"
  action-shortcut:
    textColor: "{colors.text-tertiary}"
    typography: "{typography.mono-sm}"

  # ─── Input Fields ───
  input:
    backgroundColor: "{colors.background-secondary}"
    textColor: "{colors.text-primary}"
    typography: "{typography.body-md}"
    rounded: "{rounded.md}"
    height: 36px
    padding: 12px
  input-focus:
    backgroundColor: "{colors.background-tertiary}"
  input-error:
    backgroundColor: "{colors.background-secondary}"
  input-label:
    textColor: "{colors.text-secondary}"
    typography: "{typography.label-md}"

  # ─── Keyboard Key Visual ───
  kbd:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text-primary}"
    typography: "{typography.mono-sm}"
    rounded: "{rounded.xs}"
    height: 24px
    padding: 6px

  # ─── Toast / Notification ───
  toast:
    backgroundColor: "{colors.surface}"
    textColor: "{colors.text-primary}"
    typography: "{typography.label-md}"
    rounded: "{rounded.lg}"
    padding: 12px

  # ─── Tooltip ───
  tooltip:
    backgroundColor: "{colors.text-primary}"
    textColor: "{colors.text-inverse}"
    typography: "{typography.label-sm}"
    rounded: "{rounded.sm}"
    padding: 8px

  # ─── Badge / Chip ───
  badge:
    backgroundColor: "{colors.accent-subtle}"
    textColor: "{colors.accent}"
    typography: "{typography.label-sm}"
    rounded: "{rounded.full}"
    height: 22px
    padding: 8px

  # ─── Card / Panel ───
  card:
    backgroundColor: "{colors.surface}"
    rounded: "{rounded.lg}"
    padding: 16px

  # ─── Toggle / Switch ───
  toggle:
    backgroundColor: "{colors.surface-hover}"
    rounded: "{rounded.full}"
    height: 22px
    width: 40px
  toggle-active:
    backgroundColor: "{colors.accent}"
---

# Bangla Keyboard — Design System

## Overview

Bangla Keyboard follows **Raycast's design language**: minimal, keyboard-first, and high-contrast. The UI is built for speed and clarity — every element must be instantly scannable and respond to keyboard interaction. The visual tone is professional and utilitarian, inspired by developer tools, with warm personality introduced through accent colors and smooth micro-interactions.

**Design principles:**

- **Themable by default** — Every color, font size, spacing, and radius is a semantic token. The system ships with a Dark and Light theme; custom themes can override any token.
- **Component-driven** — All UI is composed from a finite set of reusable components. No one-off styles or duplicated code.
- **Platform-agnostic** — Tokens and components apply equally to the desktop app (macOS/Windows via Tauri) and the marketing website.
- **Icon system** — All icons use [Phosphor Icons](https://phosphoricons.com) (regular weight, 20px default). No custom SVGs, no hardcoded paths.
- **Keyboard-first** — Every interactive element is reachable and operable via keyboard. Focus states are always visible.

**Target platforms:** macOS desktop app, Windows desktop app, marketing/docs website.

## Colors

The color system uses **semantic tokens** rather than raw values. Every surface, text element, and interactive state references a token. The same token names resolve to different values depending on the active theme.

### Dark Theme (Default)

- **Background (#1A1B1E):** The deepest layer. Used for the app window, sidebar backgrounds, and full-bleed sections on the website.
- **Background Secondary (#232529):** Slightly elevated surfaces — search bars, input fields, dropdown panels.
- **Surface (#2C2E33):** Cards, list item hover states, and grouped content containers.
- **Text Primary (#EBEBEF):** All headings, body text, and primary labels. High contrast against dark backgrounds.
- **Text Secondary (#A0A0AB):** Subtitles, metadata, placeholder text, and secondary labels.
- **Text Tertiary (#6E6E7A):** Disabled text, keyboard shortcut hints, and watermarks.
- **Accent (#FF6363):** Primary interactive color — active states, primary buttons, selected sidebar items, focus rings. A warm red that provides strong contrast against dark surfaces.
- **Semantic Colors:** Green (#59D499) for success/active states, Yellow (#FFC531) for warnings, Blue (#56C2FF) for informational highlights and focus borders, Purple (#B388FF) and Magenta (#FF7EB3) for decorative accents.

### Light Theme

| Token                | Light Value             |
| -------------------- | ----------------------- |
| background           | #FFFFFF                 |
| background-secondary | #F5F5F7                 |
| background-tertiary  | #EBEBEF                 |
| surface              | #F0F0F2                 |
| surface-hover        | #E8E8EC                 |
| surface-active       | #DDDDE2                 |
| overlay              | rgba(0, 0, 0, 0.3)      |
| text-primary         | #1A1B1E                 |
| text-secondary       | #5C5C66                 |
| text-tertiary        | #9898A3                 |
| text-inverse         | #FFFFFF                 |
| accent               | #E04848                 |
| accent-hover         | #CC3F3F                 |
| accent-active        | #B83636                 |
| accent-subtle        | rgba(224, 72, 72, 0.08) |
| border               | #E0E0E5                 |
| border-hover         | #CCCCCC                 |
| border-focus         | #3B9EDB                 |
| separator            | rgba(0, 0, 0, 0.06)     |

All semantic feedback colors (success, warning, error, info) are slightly darkened in light mode for contrast compliance.

## Typography

All text uses **Inter** — a geometric sans-serif optimized for UI readability at small sizes. Monospace contexts (keyboard shortcuts, code) use **JetBrains Mono**.

- **Display (36px/700):** Hero headings on the website and onboarding screens.
- **Headline Large (24px/600):** Page titles in settings, section headers on the website.
- **Headline Medium (20px/600):** Panel titles, modal headers.
- **Body Large (16px/400):** Primary reading text on the website, large input fields (search bar).
- **Body Medium (14px/400):** Default body text in the app — list items, descriptions, form labels.
- **Body Small (12px/400):** Captions, helper text, timestamps.
- **Label Large (14px/500):** Button text, sidebar items, active navigation.
- **Label Medium (12px/500):** Badges, tags, small buttons, keyboard shortcut labels.
- **Label Small (11px/500):** Chip text, overlines, status indicators.
- **Mono Medium (13px/400):** Inline code, keyboard shortcut display in action panels.
- **Mono Small (11px/400):** Compact shortcut hints.

Font stack fallback: `Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif`.

## Layout

The layout follows a **sidebar + content panel** pattern for the desktop settings app, and a **single-column floating panel** for the input method candidate window.

**Desktop settings app:**

- Fixed sidebar (240px) on the left with navigation items
- Content panel fills remaining width, max 720px for readability
- 16px base padding inside panels, 8px between list items

**Input method candidate window:**

- Floating panel, width adapts to content (min 280px, max 480px)
- 8px internal padding, 4px gap between candidate items
- Positioned near the text cursor, respects screen edges

**Website:**

- Max-width container: 1120px, centered
- 24px column gutters, 32px section margins on desktop
- Responsive breakpoints: 640px (mobile), 1024px (tablet), 1280px (desktop)

**Spacing scale:** Based on a 4px unit. Use `xs` (4px) for tight gaps (icon-to-text), `sm` (8px) for related items, `md` (12px) for element padding, `lg` (16px) for section padding, `xl` (24px) for group separation, `2xl`+ for page-level margins.

## Elevation & Depth

Depth is conveyed through **layered surfaces** and **subtle borders**, not heavy drop shadows. This follows Raycast's flat-but-layered aesthetic visible in the screenshots.

- **Level 0 (Background):** The base app/page background. No shadow.
- **Level 1 (Surface):** Cards, sidebar, content panels. Distinguished by a 1px border (`border` token) or a slightly lighter fill. No shadow in dark mode; a `0 1px 3px rgba(0,0,0,0.08)` shadow in light mode.
- **Level 2 (Overlay):** Floating panels (candidate window, action panel, dropdown menus, tooltips). Shadow: `0 8px 24px rgba(0,0,0,0.24)` in dark, `0 8px 24px rgba(0,0,0,0.12)` in light. Background uses `background-secondary` with optional backdrop-blur (8px) where platform supports it.
- **Level 3 (Modal):** Full modals and dialogs. Dimmed background overlay (`overlay` token). Shadow: `0 16px 48px rgba(0,0,0,0.32)`.

Blur effects (vibrancy) are optional enhancements on macOS; the design must remain legible without them.

## Shapes

The shape language is **softly rounded** — approachable but not bubbly. Interactive elements use medium radii; containers use slightly larger radii. Pill shapes are reserved for badges and toggles.

- **xs (4px):** Keyboard key visuals (`kbd`), inline code blocks
- **sm (6px):** Action menu items, small interactive targets
- **md (8px):** Buttons, input fields, list items, sidebar items
- **lg (12px):** Cards, panels, search bar, floating windows, toasts
- **xl (16px):** Large hero cards on the website
- **full (9999px):** Badges, toggles, avatar circles

## Components

All UI is assembled from the components defined below. Each component references theme tokens — never hardcoded values. Components are shared across the desktop app and website.

### Search Bar

The top-level search/filter input. Full-width within its container, 48px height, `rounded-lg`. Left-aligned Phosphor `MagnifyingGlass` icon in `text-tertiary`. Placeholder text uses `text-tertiary`.

### Sidebar

Fixed-width vertical navigation (240px). Items are 36px tall, `rounded-md`, with a Phosphor icon (20px) and label. Active item gets `surface` background and `text-primary` color. Hover state uses `surface-hover`.

### List Item

A single row in a scrollable list. 40px height, `rounded-md`. Supports: leading Phosphor icon, primary text, trailing subtitle/metadata, and optional trailing accessory (badge, shortcut hint). Selected state uses `accent-subtle` background.

### Action Panel

A floating context menu triggered by `⌘K` or right-click. `rounded-lg`, elevated (Level 2). Contains `action-item` rows with icon + label + keyboard shortcut aligned right. Grouped by separators.

### Button

Three variants: **Primary** (accent fill, inverse text), **Secondary** (surface fill, primary text), **Ghost** (transparent, secondary text). All 36px height, `rounded-md`, `label-lg` typography. Icon-only buttons are 36×36px square.

### Input Field

Text inputs and textareas. `background-secondary` fill, `rounded-md`, 36px height. Focus state adds a 2px `border-focus` ring. Error state shows `error` border and helper text below in `error` color.

### Keyboard Key (`kbd`)

Visual representation of a keyboard key or shortcut. `surface` background, `rounded-xs`, `mono-sm` typography. Used in action panels and documentation.

### Toast / Notification

Transient feedback at the bottom of the viewport. `surface` background, `rounded-lg`, Level 2 elevation. Contains icon + message. Auto-dismisses after 3s.

### Tooltip

Appears on hover/focus after 500ms delay. Inverted colors (`text-primary` background, `text-inverse` text), `rounded-sm`, `label-sm` typography.

### Badge / Chip

Compact status indicator. Pill-shaped (`rounded-full`), `accent-subtle` background, `accent` text. Variants use semantic color tokens (success, warning, info).

### Card

A content container for grouped information. `surface` background, `rounded-lg`, 16px padding. Used in settings panels and website feature sections.

### Toggle / Switch

A boolean control. Pill-shaped track (40×22px), circular thumb. Inactive: `surface-hover` track. Active: `accent` track with white thumb.

### Icon Usage

All icons are sourced from **Phosphor Icons** (https://phosphoricons.com). Default weight: **Regular**. Default size: **20px** (matches `label-lg` line height). Use **Bold** weight (20px) for active/emphasized states only. Never use custom SVGs or inline SVG markup — always reference Phosphor by name.

## Do's and Don'ts

- Do reference theme tokens for every visual property — never write raw hex, px, or font values in component code
- Do use Phosphor Icons exclusively — no custom SVGs, no emoji as icons
- Do support both Dark and Light themes from day one — test both before shipping
- Do keep the sidebar + panel layout consistent across all settings screens
- Do maintain WCAG AA contrast (4.5:1 for body text, 3:1 for large text and icons)
- Do use `rounded-md` (8px) as the default radius for interactive elements
- Don't hardcode any color, font-size, spacing, or border-radius in component code
- Don't create one-off styled elements — extract a reusable component
- Don't mix rounded and sharp corners in the same view hierarchy
- Don't use more than one accent color per screen (semantic colors excluded)
- Don't use shadows in dark mode except for Level 2+ floating elements
- Don't render SVG paths directly — always use the Phosphor icon component
