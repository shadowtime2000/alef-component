import {
  Component,
  Element,
  Space,
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
    const s = Space()
    const input = Element('input', { value: name })
    const s2 = Space()
    const button = Element('button')
    const t4 = Text('Reset', button)

    // create actions
    function onChange(e) {
      name = e.target.value // dirty data: name
    }
    function reset(e) {
      name = 'World' // dirty data: name
    }

    // create updates
    const name_up = () => {
      t2.setText(name)
      input.update('value', name)
    }

    // listen events
    input.listen('input', onChange, name_up)
    button.listen('click', reset, name_up)

    // register nodes
    this.register(p, s, input, s2, button)
  }
}
