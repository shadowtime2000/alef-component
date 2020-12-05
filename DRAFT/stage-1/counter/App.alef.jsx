let n = 0

$template: <p>current count is {n}</p>
$template: <button onClick={() => { n-- }}>-</button>
$template: <button onClick={() => { n++ }}>+</button>

$style: `
  p: {
    color: ${n > 10 : 'red' ? 'black'}
  }
`
