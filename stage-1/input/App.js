import {
  Component,
  Element,
  Space,
  Text
} from '../../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let name = 'World'
    function onChange(e) {
      name = e.target.value // dirty data: name
    }
    function reset(e) {
      name = 'World' // dirty data: name
    }

    // create nodes
    const p = Element('p')
    const text = Text('Hello ', p)
    const text2 = Text(name, p)
    const text3 = Text('!', p)
    const s = Space()
    const input = Element('input', { value: name })
    const s2 = Space()
    const button = Element('button')
    const text4 = Text('Reset', button)

    // create updates
    const name_up = () => {
      text2.update(name)
      input.update('value', name)
    }

    // listen events
    input.listen('input', onChange, name_up)
    button.listen('click', reset, name_up)

    // register nodes
    this.register(p, s, input, s2, button)
  }
}
