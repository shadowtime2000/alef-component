let ok = false

function toggle() {
  ok = !ok
}

$t: if (ok) {
  <button onClick={toggle}>OFF</button>
} else {
  <button onClick={toggle}>ON</button>
}
