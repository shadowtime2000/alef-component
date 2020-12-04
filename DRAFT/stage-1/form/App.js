import {
  Component,
  Element,
  listen,
  setText,
  setValue,
  space,
  Text
} from '../helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let name = 'World'

    // create nodes
    let p = Element('p')
    let t = Text('Hello ', p)
    let t2 = Text(name, p)
    let t3 = Text('!', p)
    let s = space()
    let input = Element('input', { value: name })
    let s2 = space()
    let button = Element('button')
    let t4 = Text('Reset', button)

    // event handles
    function onChange(e) {
      name = e.target.value // dirty data: name
    }
    function reset(e) {
      name = 'World' // dirty data: name
    }

    // register nodes
    this.nodes = [p, s, input, s2, button]

    // listen events
    this.disposes = [
      listen(input, 'input', onChange, () => {
        setText(t2, name)
      }),
      listen(button, 'click', reset, () => {
        setText(t2, name) // <- name
        setValue(input, name) // <- name
      })
    ]
  }
}
