let ok = false;

function toggle() {
	ok = !ok
}

!ok && <button onClick={toggle}>OFF</button>;
ok && <button onClick={toggle}>ON</button>;
