let text = 'El Aleph'
let show = false
let ok = false

function toggle() {
  ok = !ok
}

$t: if (ok) {
  <p>
    {show && <code>{text}</code>}
    {!show && <code>{'*'.repeat(text.length)}</code>}
    <span>&nbsp;</span>
    {show && <button onClick={() => { show = false }}>Hide</button>}
    {!show && <button onClick={() => { show = true }}>Show</button>}
  </p>
}

$t: if (ok) {
  <button onClick={toggle}>OFF</button>
} else {
  <button onClick={toggle}>ON</button>
}
