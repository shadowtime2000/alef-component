let name = 'World'

function onChange(e) {
	name = e.target.value
}

function reset() {
	name = 'World'
}

$t: <p>Hello {name}!</p>
$t: <input value={name} onChange={onChange} />
$t: <button onClick={reset}>Reset</button>
