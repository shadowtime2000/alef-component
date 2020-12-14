![Alef Component](./assets/banner.svg)

# Alef Component

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte** and based on **TSX**. Core features include:

- Born in **Typescript**
- With Standard **JSX** Syntax
- **AOT** Compile in Rust
- Zero Runtime
- No Virtual DOM
- Reactive
- Builtin Styling
- **SSR**

## Specification

The main concept of Alef Component is parted in three stages:

- Stage 1 ([Docs](https://github.com/alephjs/alef-component-draft/issues/3))
  - **Nodes Rendering** - render nodes using native DOM
  - **Conditional Rendering** - render if-else expression in JSX
  - **Loop Rendering** - render list
  - **Memo** - use computed state
  - **Side Effect** - react for state changes
  - **Events** - handle events to update view
- Stage 2 ([Docs](https://github.com/alephjs/alef-component-draft/issues/4))
  - **Import Alef Component** - `import Logo from "./Logo.alef"`
  - **Slots** - `<Logo><img ... /></Logo>`
  - **Reuse Pattern** - reuse common logics
  - **Context** - share state in child component tree
  - **Styling** - inline CSS with scope
  - **Transition** - transition animation for view changes
  - **Asynchronous Component** - wait for data fetching
  - **Error Boundary** - catch errors in child component tree
- Stage 3 ([Docs](https://github.com/alephjs/alef-component-draft/issues/5))
  - **Mount** - mount Alef Component to DOM
  - **SSR** - server side rendering
  - **Precompile** - transfom Alef Component code before AOT compilation
  - **Hot Refresh** - refresh component without data losing

## Status

Core concept is done, currently writing the MVP compiler.
