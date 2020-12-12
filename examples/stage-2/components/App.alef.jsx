import Hello from './Hello.alef'
import A from './A.alef'
import B from './B.alef'

let name = 'World'
let component = 'Hello'

$t: if (component === 'Hello') {
  <Hello name={name} />
} else if (component === 'A') {
  <A />
} else if (component === 'B') {
  <B />
}

$t:
<p>
  Show Component:
  {['Hello', 'A', 'B'].map(name => (
    <button onClick={() => component = name} key={name} disabled={name === component}>{name}</button>
  ))}
</p>

$t: <input value={e => name = e.target.value} onChange={onChange} />
$t: <button onClick={() => name = 'World'}>Reset</button>
