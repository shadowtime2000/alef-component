import {
  Component,
  Element,
  Text
} from '../../helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let name = 'world'

    // create nodes
    const p = Element('p')
    const t = Text(`hello ${name}!`, p)

    // register nodes
    this.nodes = [p]
  }
}
