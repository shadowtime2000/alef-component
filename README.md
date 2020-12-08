![Alef Component Draft](./assets/banner.svg)

<br>

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
  - **styling**: inline CSS with scope
  - **import alef component** - `import Logo from "./Logo.alef"`
  - **slots** - `<Logo><img ... /></Logo>`
  - **reuse pattern** - reuse common logics
  - **stores** - share state between components
- Stage III ([RFC](https://github.com/alephjs/alef-component-draft/issues/5))
  - **SSR** - server side rendering 
  - **suspense** - suspense for data fetching

## Run Draft

```bash
git clone https://github.com/alephjs/alef
cd alef

npx serve
```
