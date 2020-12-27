import {
  Component,
  Element
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let name = 'World'

    // create nodes
    const nodes = [
      Element(
        'p',
        null,
        `Hello ${name}!`,
      )
    ]

    // register nodes
    this.register(nodes)
  }
}
