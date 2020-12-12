import Hello from './Hello.alef'
import A from './A.alef'
import B from './B.alef'

let name = 'World'
let component = 'Hello'

$t: if (component === 'Hello') {
  <>
    <Hello name={name} />
    <input value={e => name = e.target.value} onChange={onChange} />
    {' '}
    <button onClick={() => name = 'World'}>Reset</button>
  </>
} else if (component === 'A') {
  <A />
} else if (component === 'B') {
  <B />
}

$t:
<p>
  <span>Show Component: </span>
  {['Hello', 'A', 'B'].map(name => (
    <button
      style={{ display: 'inline', marginLeft: 6 }}
      onClick={() => component = name}
      disabled={name === component}
      key={name}
    >{name}</button>
  ))}
</p>
