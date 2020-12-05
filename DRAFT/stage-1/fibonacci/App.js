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
    let numbers = [1]

    // create memos
    const $sum = () => numbers.reduce((t, n) => t + n, 0)  // dep: numbers
    const $1 /* {numbers.join(' + ')} */ = () => numbers.join(' + ') // dep: numbers

    // create nodes
    let p = Element('p')
    let t = Text('0 + ', p)
    let t2 = Text($1(), p)
    let t3 = Text(' = ', p)
    let t4 = Text($sum(), p)
    let s = space()
    let button = Element('button')
    let t5 = Text('Add a number', button)

    // event handles
    function addNumber() {
      numbers = [...numbers, (numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1]] // dirty data: numbers
    }

    // register nodes
    this.nodes = [p, s, button]

    // listen events
    this.disposes = [
      listen(button, 'click', addNumber, () => {
        setText(t2, $1()) // <- numbers
        setText(t4, $sum()) // <- numbers
      })
    ]
  }
}
