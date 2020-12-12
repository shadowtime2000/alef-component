$: () => {
  console.log('component B mounted')
  return () => console.log('component B unmounted')
}

$t: <p>A</p>

$style: `
  p {
    padding: 12px;
    border: 1px dashed green;
    color: green;
  }
`