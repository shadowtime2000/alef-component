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
    let n = 0

    // create nodes
    let p = Element('p')
    let t = Text('current count is ', p)
    let t2 = Text(n, p)
    let s = space()
    let button = Element('button')
    let t3 = Text('-', button)
    let s2 = space()
    let button2 = Element('button')
    let t4 = Text('+', button2)

    // event handles
    const _1 /* button[0].onClick */ = () => {
      n-- // dirty data: n
    }
    // 
    const _2 /* button[1].onClick */ = () => {
      n++ // dirty data: n
    }

    // register nodes
    this.nodes = [p, s, button, s2, button2]

    // listen events
    this.disposes = [
      listen(button, 'click', _1, () => {
        setText(t2, n) // <- n
      }),
      listen(button2, 'click', _2, () => {
        setText(t2, n) // <- n
      })
    ]
  }
}
