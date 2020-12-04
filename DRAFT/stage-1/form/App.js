import {
  Component,
  Element,
  listen,
  setText,
  space,
  Text
} from '../helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let name = 'World'

    // create nodes
    let input = Element('input', null, { value: name })
    let s = space()
    let p = Element('p')
    let t = Text('Hello ', p)
    let t2 = Text(name, p)
    let t3 = Text('!', p)
    let s2 = space()
    let button = Element('button')
    let t4 = Text('reset', button)

    // event handles
    function onChange(e) {
      name = e.target.value // dirty data: name
    }
    function reset(e) {
      name = 'World' // dirty data: name
    }

    // register nodes
    this.nodes = [input, s, p, s2, button]

    // listen events
    this.disposes = [
      listen(input, 'input', onChange, () => {
        setText(t2, name) // <- name
      }),
      listen(button, 'click', reset, () => {
        setText(t2, name) // <- name
        input.value = name // <- name
      })
    ]
  }
}
