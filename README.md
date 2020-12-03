# Alef Component

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte** and based on **TSX**.

- Born in Typescript
- With Standard JSX Syntax
- AOT Compile in Rust
- No Virtual DOM
- Zero Runtime
- Reactive
- Support SSR

![Alef Component](./assets/alef_component.png)

## Draft

- Stage 1
  - **nodes rendering**: render nodes using native DOM
  - **events**: handle events to update view
  - **memo**: reactive state
  - **effect**: reactive function
- Stage 2
  - **styling**: transform inline CSS with scope
  - **import alef component** - `import Logo from "Logo.alef"`
  - **slots** - `<Logo><img ... /></Logo>`
  - **reuse pattern** - reuse common logics
  - **stores** - share state in global
- Stage 3
  - **SSR** - server side rendering
  - **CSS preprocess** - support **less**, **sass**, ...
  - **tooling** - client command, IDE support, documentation, ...

## Status

Drafting the draft.
