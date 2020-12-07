import {
  Component,
  Element,
  Space,
  Text
} from '../../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let n = 0

    // create nodes
    const p = Element('p')
    const text = Text('current count is ', p)
    const text2 = Text(n, p)
    const s = Space()
    const button = Element('button')
    const text3 = Text('-', button)
    const s2 = Space()
    const button2 = Element('button')
    const text4 = Text('+', button2)

    // create updates
    const n_up = () => {
      text2.update(n)
    }

    // listen events
    button.listen('click', () => { n-- }, n_up)
    button2.listen('click', () => { n++ }, n_up)

    // register nodes
    this.register(p, s, button, s2, button2)
  }
}
