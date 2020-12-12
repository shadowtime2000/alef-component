import {
  banchUpdate,
  Component,
  Element,
  If,
  IfElse,
  List,
  Memo,
  New,
  Space,
  Text
} from '../../lib/helper.js'
import A from './A.js'
import B from './B.js'
import Hello from './Hello.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let name = 'World'
    let component = 'Hello'

    // create list blocks
    const $list_block = name => ({
      create: () => {
        // create memos
        const $1 = Memo(() => name === component)

        // create nodes
        const button = Element('button', { disabled: $1.value })
        /**/ const text = Text(name, button)

        // create updates
        const up = () => {
          banchUpdate(
            $1,
            [button, 'disabled', () => $1.value]
          )
        }

        // listen events
        button.listen('click', e => {
          e.preventDefault()
          component = name
        }, component_up)

        return { node: button, update: up }
      },
      key: name,
    })

    // create nodes
    const block = IfElse(() => component === 'Hello')
    /**/ const c_hello = New(Hello, { name }, block.if)
    /**/ const block2 = IfElse(() => component === 'A', block.else)
    /***/ const c_a = New(A, null, block2.if)
    /***/ const block3 = If(() => component === 'B', block2.else)
    /****/ const c_b = New(B, null, block3)
    const p = Element('p')
    /**/ const text = Text('Show Component: ', p)
    /**/ const list = List(['Hello', 'A', 'B'], $list_block, p)
    const s = Space()
    const s2 = Space()
    const input = Element('input', { value: name })
    const s3 = Space()
    const button = Element('button')
    /**/ const text2 = Text('Reset', button)

    // create updates
    const name_up = () => banchUpdate(
      [c_hello, 'name', name],
      [input, 'value', name],
    )
    const component_up = () => banchUpdate(
      block,
      block2,
      block3,
      list
    )

    // listen events
    input.listen('input', e => name = e.target.value, name_up)
    button.listen('click', () => name = 'world', name_up)

    // register nodes
    this.register(block, s, p, s2, input, s3, button)
  }
}
