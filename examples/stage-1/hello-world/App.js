import {
  Component,
  Element,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let name = 'world'

    // create nodes
    const p = Element('p')
    /**/ const text = Text('Hello ', p)
    /**/ const text2 = Text(name, p)
    /**/ const text3 = Text('!', p)

    // register nodes
    this.register(p)
  }
}
