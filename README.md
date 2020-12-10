![Alef Component Concept](./assets/banner.svg)

# Alef Component Concept

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte** and based on **TSX**. Core features include:

- Born in **Typescript**
- With Standard **JSX** Syntax
- **AOT** Compile in Rust
- Zero Runtime
- No Virtual DOM
- Reactive
- Builtin Styling
- **SSR**

## Stages
This draft is parted in three stages, currently accept any new feature and improvement. After the draft is locked, the **AOT** comilper will be implemented to make it works in nodejs, Deno and browsers.

- Stage I ([RFCs](https://github.com/alephjs/alef-component-draft/issues/3))
  - **Nodes Rendering** - render nodes using native DOM
  - **Conditional Rendering** - render if-else expression in JSX
  - **Loop Rendering** - render list
  - **Events** - handle events to update view
  - **Memo** - use computed state
  - **Side Effect** - react for state changes
- Stage II ([RFCs](https://github.com/alephjs/alef-component-draft/issues/4))
  - **Import Alef Component** - `import Logo from "./Logo.alef"`
  - **Slots** - `<Logo><img ... /></Logo>`
  - **Reuse Pattern** - reuse common logics
  - **Context** - share state in child component tree
  - **Styling** - inline CSS with scope
  - **Transition** - transition animation for view changes
  - **Mount** - mount Alef Component to DOM
- Stage III ([RFCs](https://github.com/alephjs/alef-component-draft/issues/5))
  - **Asynchronous Component** - wait for data fetching
  - **Error Boundary** - catch errors in child component tree
  - **SSR** - server side rendering
  - **Precompile** - transfom Alef Component code before AOT compilation

## Run Draft

```bash
git clone https://github.com/alephjs/alef-component-rfcs
cd alef-component-rfcs

npx serve examples
```

## Status
Core concept is done, request for comments.
