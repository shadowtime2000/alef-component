import {
  Component,
  Element,
  Memo
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let n = 0

    // create nodes
    const nodes = [
      Element(
        'p',
        null,
        'current count is ',
        Memo(() => n, [0])
      ),
      Element('button', { onClick: Dirty(() => { n-- }, [0]) }, '-'),
      Element('button', { onClick: Dirty(() => { n++ }, [0]) }, '+')
    ]

    // register nodes
    this.register(nodes)
  }
}
