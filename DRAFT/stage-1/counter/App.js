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
    let n = 0

    // create nodes
    const p = Element('p')
    const t = Text('current count is ', p)
    const t2 = Text(n, p)
    const s = Space()
    const button = Element('button')
    const t3 = Text('-', button)
    const s2 = Space()
    const button2 = Element('button')
    const t4 = Text('+', button2)

    // create actions
    const _1 /* button[0].onClick */ = () => {
      n-- // dirty data: n
    }
    const _2 /* button[1].onClick */ = () => {
      n++ // dirty data: n
    }

    // create updates
    const n_up = () => {
      t2.setText(n)
    }

    // listen events
    button.listen('click', _1, n_up)
    button2.listen('click', _2, n_up)

    // register nodes
    this.register(p, s, button, s2, button2)
  }
}
