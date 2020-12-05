import {
  Component,
  Element,
  listen,
  setText,
  space,
  Text
} from '../../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let n = 0

    // create nodes
    const p = Element('p')
    const t = Text('current count is ', p)
    const t2 = Text(n, p)
    const s = space()
    const button = Element('button')
    const t3 = Text('-', button)
    const s2 = space()
    const button2 = Element('button')
    const t4 = Text('+', button2)

    // event handles
    const _1 /* button[0].onClick */ = () => {
      n-- // dirty data: n
    }
    const _2 /* button[1].onClick */ = () => {
      n++ // dirty data: n
    }

    // create updates
    const n_up = () => {
      setText(t2, n)
    }

    // register nodes
    this.nodes = [p, s, button, s2, button2]

    // listen events
    this.disposes = [
      listen(button, 'click', _1, n_up),
      listen(button2, 'click', _2, n_up)
    ]
  }
}
