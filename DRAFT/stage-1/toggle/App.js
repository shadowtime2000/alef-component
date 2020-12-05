import {
  Component,
  Element,
  listen,
  Text,
  IfBlock,
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let ok = false

    // create blocks 
    const block = new IfBlock(
      () => !ok,
      (self) => {
        // create nodes  
        const button = Element('button')
        const t = Text('OFF', button)

        // register nodes
        self.nodes = [button]

        // listen events
        self.disposes = [
          listen(button, 'click', toggle, () => {
            ok_up()
          })
        ]
      }
    )
    const block2 = new IfBlock(
      () => ok,
      (self) => {
        // create nodes  
        const button = Element('button')
        const t = Text('ON', button)

        // register nodes
        self.nodes = [button]

        // listen events
        self.disposes = [
          listen(button, 'click', toggle, () => {
            ok_up()
          })
        ]
      }
    )

    // create updates
    const ok_up = () => {
      block.toggle()
      block2.toggle()
    }

    // event handles
    function toggle() {
      ok = !ok // dirty data: ok
    }

    // register nodes
    this.nodes = [block, block2]
  }
}
