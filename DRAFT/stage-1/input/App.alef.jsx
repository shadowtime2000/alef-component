let name = 'World';

function onChange(e) {
	name = e.target.value
}

function reset() {
	name = 'World'
}

<p>Hello {name}!</p>;
<input value={name} onChange={onChange} />;
<button onClick={reset}>Reset</button>;
