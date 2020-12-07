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
    const s = Space()
    const button = Element('button')
    const t5 = Text('Add a number', button)

    // create actions
    function addNumber() {
      numbers = [...numbers, (numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1]] // dirty data: numbers
    }

    // create updates
    const numbers_up = () => {
      t2.setText($1())
      t4.setText($sum())
    }

    // listen events
    button.listen('click', addNumber, numbers_up)

    // register nodes
    this.register(p, s, button)
  }
}
