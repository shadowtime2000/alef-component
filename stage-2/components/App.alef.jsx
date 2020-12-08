import Hello from './Hello.alef'

let name = 'World'

function onChange(e) {
  name = e.target.value
}

function reset() {
  name = 'World'
}

$t: <Hello name={name} />
$t: <input value={name} onChange={onChange} />
$t: <button onClick={reset}>Reset</button>
