import {
  Component,
  Element,
  Space,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let numbers /* Array */ = [1]
    function addNumber() {
      numbers.push((numbers[numbers.length - 2] || 0) + numbers[numbers.length - 1]) // dirty data: numbers
    }

    // create memos
    const $sum = () => numbers.reduce((t, n) => t + n, 0)  // dep: numbers
    const $1 /* {numbers.join(' + ')} */ = () => numbers.join(' + ') // dep: numbers

    // create nodes
    const p = Element('p')
    /**/ const text = Text('0 + ', p)
    /**/ const text2 = Text($1(), p)
    /**/ const text3 = Text(' = ', p)
    /**/ const text4 = Text($sum(), p)
    const s = Space()
    const button = Element('button')
    /**/ const text5 = Text('Add a number', button)

    // create updates
    const numbers_up = () => {
      text2.update($1())
      text4.update($sum())
    }

    // listen events
    button.listen('click', addNumber, numbers_up)

    // register nodes
    this.register(p, s, button)
  }
}
