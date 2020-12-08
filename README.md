![Alef Component Draft](./assets/banner.svg)

# Alef Component Draft

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte** and based on **TSX**. Core features include:

- Born in **Typescript**
- With Standard **TSX** Syntax
- **AOT** Compile in Rust
- Zero Runtime
- No Virtual DOM
- Reactive
- Builtin Styling
- Support SSR

## Stages
This *DRAFT* is parted in three stages, currently accept any new features and improvements about it. After the draft is locked, the **AOT** comilper in Rust will be implemented to make it works in nodejs and Deno.

- Stage I ([RFC](https://github.com/alephjs/alef-component-draft/issues/3))
  - **nodes rendering**: render nodes using native DOM
  - **conditional rendering**：...if...else...
  - **loop rendering**：render list
  - **events**: handle events to update view
  - **memo**: use computed states
  - **side effect**: react for state changing
- Stage II ([RFC](https://github.com/alephjs/alef-component-draft/issues/4))
  - **import alef component** - `import Logo from "./Logo.alef"`
  - **slots** - `<Logo><img ... /></Logo>`
  - **reuse pattern** - reuse common logics
  - **context** - share state between components
  - **styling**: inline CSS with scope
- Stage III ([RFC](https://github.com/alephjs/alef-component-draft/issues/5))
  - **asynchronous component** - asynchronous component waiting for data fetching
  - **error boundary** - catch errors in child component tree
  - **SSR** - server side rendering 

## Run Draft

```bash
git clone https://github.com/alephjs/alef
cd alef

npx serve
```
