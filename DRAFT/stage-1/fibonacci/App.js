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
    const p = Element('p')
    const t = Text('0 + ', p)
    const t2 = Text($1(), p)
    const t3 = Text(' = ', p)
    const t4 = Text($sum(), p)
    const s = space()
    const button = Element('button')
    const t5 = Text('Add a number', button)

    // event handles
    function addNumber() {
      numbers = [...numbers, (numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1]] // dirty data: numbers
    }

    // create updates
    const numbers_up = () => {
      setText(t2, $1())
      setText(t4, $sum())
    }

    // register nodes
    this.nodes = [p, s, button]

    // listen events
    this.disposes = [
      listen(button, 'click', addNumber, numbers_up)
    ]
  }
}
