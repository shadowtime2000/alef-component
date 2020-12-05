import {
  Component,
  Element,
  listen,
  Text,
  IfBlock,
} from '../../helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let ok = false

    // create blocks 
    let block = new IfBlock(
      () => !ok,
      (self) => {
        // create nodes  
        let button = Element('button')
        let t = Text('OFF', button)

        // register nodes
        self.nodes = [button]

        // listen events
        self.disposes = [listen(button, 'click', toggle, () => {
          block.toggle() // <- ok
          block2.toggle() // <- ok
        })]
      }
    )
    let block2 = new IfBlock(
      () => ok,
      (self) => {
        // create nodes  
        let button = Element('button')
        let t = Text('ON', button)

        // register nodes
        self.nodes = [button]

        // listen events
        self.disposes = [listen(button, 'click', toggle, () => {
          block.toggle() // <- ok
          block2.toggle() // <- ok
        })]
      }
    )

    // event handles
    function toggle() {
      ok = !ok // dirty data: ok
    }

    // register nodes
    this.nodes = [block, block2]
  }
}
