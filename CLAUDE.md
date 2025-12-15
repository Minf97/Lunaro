# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Lunaro is a desktop application built with Tauri 2, Svelte 5, and shadcn-svelte components. It combines a Rust backend (Tauri) with a modern Svelte 5 frontend using Runes API, styled with Tailwind CSS v4.

## Development Commands

### Start development server

```bash
npm run tauri dev
```

This starts both the Vite dev server (port 1420) and the Tauri application.

### Build for production

```bash
npm run tauri build
```

### Type checking

```bash
npm run check
```

Runs svelte-check for Svelte files and TypeScript compiler for Node files.

### Linting and formatting

```bash
npm run lint      # Check for linting and formatting issues
npm run format    # Auto-format all files with Prettier
```

### Add shadcn-svelte components

```bash
npx shadcn-svelte@next add <component>
```

Available components: button, card, dialog, input, command, sidebar, etc.
Full list: https://next.shadcn-svelte.com/docs/components

## Architecture

### Frontend (Svelte 5 with Runes)

**Key locations:**

- `src/App.svelte` - Main application entry point
- `src/main.ts` - Svelte app mount point
- `src/lib/` - Shared library code

**Component structure:**

- `src/lib/components/` - Custom application components
  - `AppSideBar.svelte` - Main sidebar layout
  - `Search.svelte` - Search component
  - `HelloWorld.svelte` - Example component
- `src/lib/components/ui/` - shadcn-svelte UI components (auto-generated, avoid manual edits)

**State management:**

- Uses Svelte 5 runes (`$state`, `$derived`, `$effect`)
- `src/lib/commands.svelte.ts` - Contains `GlobalState` class for shared application state
- State is reactive using Svelte 5's new runes API

**Utilities:**

- `src/lib/utils.ts` - Contains `cn()` helper for className merging (clsx + tailwind-merge)
- `src/lib/hooks/` - Custom Svelte hooks (e.g., `is-mobile.svelte.ts`)

**Path aliases:**

- `$lib` maps to `./src/lib` - always use this alias for imports

### Backend (Tauri/Rust)

**Key locations:**

- `src-tauri/src/lib.rs` - Main Tauri application setup
- `src-tauri/src/commands/` - Tauri command modules
- `src-tauri/tauri.conf.json` - Tauri configuration

**Command structure:**

- Commands are defined in `src-tauri/src/commands/default.rs`
- Available commands: `read`, `write` (file operations)
- Commands are invoked from frontend using `@tauri-apps/api/core`
- Register new commands in `lib.rs` using `tauri::generate_handler![]`

**Adding new commands:**

1. Create function in `src-tauri/src/commands/default.rs` with `#[tauri::command]` attribute
2. Add to handler in `src-tauri/src/lib.rs`
3. Import and use in frontend via `invoke()` from `@tauri-apps/api/core`

### Styling (Tailwind CSS v4)

- Uses Tailwind CSS v4 (configured via `@tailwindcss/vite` plugin)
- Design system defined in `src/app.css` using CSS variables
- Supports dark mode via `.dark` class
- Custom color scheme using OKLCH color space
- No separate `tailwind.config.js` needed (Tailwind v4 uses CSS-based configuration)

### Git Hooks

Husky is configured with pre-commit hooks:

- Runs `lint-staged` on commit
- Auto-formats staged files with Prettier
- Auto-fixes ESLint issues on staged files

## Important Patterns

### Svelte 5 Runes

This project uses Svelte 5 with runes enabled. Use:

- `$state()` for reactive state
- `$derived()` for computed values
- `$effect()` for side effects
- `$props()` for component props

Do NOT use legacy Svelte reactivity ($: syntax).

### Tauri Commands

Frontend-to-backend communication pattern:

```typescript
import { invoke } from '@tauri-apps/api/core';
const result = await invoke<string>('command_name', { arg: value });
```

### Component Imports

shadcn-svelte components use barrel exports:

```typescript
import * as Sidebar from '$lib/components/ui/sidebar';
```

### TypeScript Configuration

Project uses TypeScript with strict mode. Two tsconfig files:

- `tsconfig.app.json` - For application code
- `tsconfig.node.json` - For Node/build tooling

## Testing

Currently no test framework is configured. When adding tests, common choices for this stack:

- Vitest for unit tests
- Playwright for E2E tests
