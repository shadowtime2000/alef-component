let n = 0

$t: <p>current count is {n}</p>
$t: <button onClick={() => { n-- }}>-</button>
$t: <button onClick={() => { n++ }}>+</button>

$style: `
  h1 {
    font-size: 200%;
  }
  p {
    color: ${Math.abs(n) >= 10 ? 'red' : 'green'}    
  }
  button {
    display: inline-block;
    width: 24px;
    height: 24px;
    font-weight: bold;
  }
`
