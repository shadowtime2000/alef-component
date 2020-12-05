let numbers = [1];

$: sum = numbers.reduce((t, n) => t + n, 0);

function addNumber() {
	numbers = [...numbers, (numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1]]
}

$t: <p>0 + {numbers.join(' + ')} = {sum}</p>
$t: <button onClick={addNumber}>Add a number</button>
