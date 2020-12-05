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

// eq `useEffect(()=>{}, [n])`
$: () => {
  console.log(`current n is ${n}.`)
}

// eq `useEffect(()=>{}, [])`
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
  /* unused h1, tree-shaking to remove it */
  h1 {
    font-size: 200%;
  }
  p {
    color: ${Math.abs(n) >= 10 ? 'red' : 'green'}    
  }
`
```

## Draft

- Stage 1
  - **nodes rendering**: render nodes using native DOM
  - **conditional rendering**：...if...else...
  - **loop rendering**：render list
  - **events**: handle events to update view
  - **memo**: use computed states
  - **side effect**: react(apply side effect) for state changing
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

### Run Draft

```bash
npx serve DRAFT
```

## Status

Drafting the draft.
