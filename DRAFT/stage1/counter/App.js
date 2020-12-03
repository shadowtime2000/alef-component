import { Component, Element, Text, listen, setTextContent } from '../helper.js'

export default class App extends Component {
  constructor() {
    super()

    let n = 0

    // button[0].onClick
    const handler = () => {
      n--
    }
    // button[1].onClick
    const handler2 = () => {
      n++
    }

    let p = Element('p')
    let t = Text('current count is ', p)
    let t2 = Text(n, p)
    let button = Element('button')
    let t3 = Text('-', button)
    let t4 = Text('\n')
    let button2 = Element('button')
    let t5 = Text('+', button2)

    this.nodes = [p, button, t4, button2]
    this.disposes = [
      listen(button, 'click', handler, () => {
        setTextContent(t2, n)
      }),
      listen(button2, 'click', handler2, () => {
        setTextContent(t2, n)
      })
    ]
  }
}
