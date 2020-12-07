import {
  Component,
  Element,
  Space,
  Text
} from '../../../lib/helper.js'
import Hello from './Hello.js'

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
    const hello = new Hello({ name })
    const s = Space()
    const input = Element('input', { value: name })
    const s2 = Space()
    const button = Element('button')
    const text3 = Text('Reset', button)

    // create updates
    const name_up = () => {
      hello.update('name', name)
      input.update('value', name)
    }

    // listen events
    input.listen('input', onChange, name_up)
    button.listen('click', reset, name_up)

    // register nodes
    this.register(hello, s, input, s2, button)
  }
}
