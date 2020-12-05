import {
  Component,
  Element,
  listen,
  setText,
  setValue,
  space,
  Text
} from '../../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let name = 'World'

    // create nodes
    const p = Element('p')
    const t = Text('Hello ', p)
    const t2 = Text(name, p)
    const t3 = Text('!', p)
    const s = space()
    const input = Element('input', { value: name })
    const s2 = space()
    const button = Element('button')
    const t4 = Text('Reset', button)

    // event handles
    function onChange(e) {
      name = e.target.value // dirty data: name
    }
    function reset(e) {
      name = 'World' // dirty data: name
    }

    // create updates
    const name_up = () => {
      setText(t2, name)
      setValue(input, name)
    }

    // register nodes
    this.nodes = [p, s, input, s2, button]

    // listen events
    this.disposes = [
      listen(input, 'input', onChange, name_up),
      listen(button, 'click', reset, name_up)
    ]
  }
}
