import {
  Component,
  Element,
  listen,
  Text,
  IfBlock,
} from '../helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let ok = false

    // create nodes  
    let button = Element('button')
    let t = Text('OFF', button)
    let button2 = Element('button')
    let t2 = Text('ON', button2)

    // create blocks 
    let block = new IfBlock(
      () => !ok,
      [button],
      [listen(button, 'click', toggle, () => {
        block.toggle() // <- ok
        block2.toggle() // <- ok
      })]
    )
    let block2 = new IfBlock(
      () => ok,
      [button2],
      [listen(button2, 'click', toggle, () => {
        block.toggle() // <- ok
        block2.toggle() // <- ok
      })]
    )

    // event handles
    function toggle() {
      ok = !ok // dirty data: ok
    }

    // register nodes
    this.nodes = [block, block2]
  }
}
