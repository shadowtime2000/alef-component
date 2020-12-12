$: () => {
  console.log('component A mounted')
  return () => console.log('component A unmounted')
}

$t: <p>A</p>

$style: `
  p {
    padding: 12px;
    border: 1px dashed blue;
    color: blue;
  }
`