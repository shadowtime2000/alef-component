# Alef Component

Alef Component for Modern Web Apps, it's inspired by **React** and **Svelte** and based on **TSX**.

- Born in Typescript
- With Standard JSX Syntax
- AOT Compile in Rust
- Zero Runtime
- No Virtual DOM
- Reactive
- Builtin Styling
- Support SSR

```jsx
import Logo from './Logo.alef'

const name: string = 'World' // prop

let n: number = 0 // state

// eq `useMemo`
$: double = 2 * n
$: message = `Hello ${name}!`

// eq `useEffect(callback, [n])`
$: () => {
  console.log(`current count is ${n}`)
}

// eq `useEffect(callback, [])`
$: () => {
  console.log('mounted')
  return () => {
    console.log('unmounted')
  }
}

// event handler
function onClick() {
  n++ // eq `setN(n => n+1)`
}

// nodes(template)
$t: <Logo />
$t: <div>{message}</div>
$t: <p onClick={onClick}>Counter: {double}</p>

// styling
$style: `
  /* unused h1 (tree-shaking) */
  h1 {
    font-size: 200%;
  }
  p {
    color: ${Math.abs(n) >= 10 ? 'red' : 'green'}    
  }
`
```

## Draft

This *DRAFT* is parted in three stages, currently accept any new features and improvements about it. After the draft is locked, the **AOT** comilper in Rust will be implemented to make it works in nodejs and Deno.

- Stage 1
  - **nodes rendering**: render nodes using native DOM
  - **conditional rendering**：...if...else...
  - **loop rendering**：render list
  - **events**: handle events to update view
  - **memo**: use computed states
  - **side effect**: react for state changing
- Stage 2
  - **styling**: transform inline CSS with scope
  - **import alef component** - `import Logo from "Logo.alef"`
  - **slots** - `<Logo><img ... /></Logo>`
  - **reuse pattern** - reuse common logics
  - **stores** - share state in global
- Stage 3
  - **SSR** - server side rendering 
  - **suspense** - suspense for data fetching
  - **tooling** - documentation, webpack/rullup loader, IDE support, ...

### Run Draft

```bash
git clone https://github.com/alephjs/alef
cd alef

npx serve DRAFT
```

## Status

Drafting the draft.
