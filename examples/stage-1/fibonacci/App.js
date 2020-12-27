import {
  Component,
  Element,
  Memo,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let numbers /* Array */ = [1]
    function addNumber() {
      numbers.push((numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1])
    }

    // create memos
    const $sum = Memo(() => numbers.reduce((t, n) => t + n, 0), [0])

    // create nodes
    const nodes = [
      Element(
        'p',
        null,
        '0 + ',
        Text(Memo(() => numbers.join(' + '), [0])),
        ' = ',
        Text($sum)
      ),
      Element('button', {
        onClick: Dirty(addNumber, [0])
      }, 'Add a number')
    ]

    // register nodes
    this.register(nodes)
  }
}
