# Alef

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte**, base on **TSX**.

- Born in Typescript
- With Standard JSX Syntax
- AOT Compile in Rust
- No Virtual DOM
- Zero Runtime
- Reactive
- Support SSR

![Alef Component](./assets/alef_component.png)

## Draft

- stage 1
  - **nodes rendering**: render nodes using native DOM
  - **events**: handle events to update view
  - **memo**: reactive memo
  - **effect**: reactive effect
- stage 2
  - **styling**: transform inline CSS with scope
  - **import alef component** - `import Logo from "Logo.alef"`
  - **slots** - `<Logo><img ... /></Logo>`
  - **reuse pattern** - reuse common logics
  - **stores** - share state in global
- stage 3
  - **SSR** - server side rendering
  - **CSS preprocess** - support **lesss**, **sass**...
  - **tooling** - IDE support, documentation, ...


  ## Status

  **Drafting the draft**
  