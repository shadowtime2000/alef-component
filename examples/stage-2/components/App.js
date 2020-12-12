import {
  banchUpdate,
  Component,
  Element,
  Fragment,
  IfElse,
  List,
  Memo,
  New,
  nope,
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

    // create blocks
    const $if_block = () => {
      // create nodes
      const fragment = Fragment()
      /**/ const c_hello = New(Hello, { name }, fragment)
      /**/ const input = Element('input', { value: name }, fragment)
      /**/ const s3 = Space(fragment)
      /**/ const button = Element('button', fragment)
      /***/ const text2 = Text('Reset', button)

      // create updates
      const name_up = () => banchUpdate(
        [c_hello, 'name', name],
        [input, 'value', name],
      )

      // listen events
      input.listen('input', e => name = e.target.value, name_up)
      button.listen('click', () => name = 'world', name_up)

      return { node: fragment, update: nope }
    }
    const $if_block2 = () => {
      // create nodes
      const c_a = New(A)

      return { node: c_a, update: nope }
    }
    const $if_block3 = () => {
      // create nodes
      const c_b = New(B)

      return { node: c_b, update: nope }
    }
    const $list_block = name => ({
      create: () => {
        // create memos
        const $1 = Memo(() => name === component)

        // create nodes
        const button = Element('button', { style: { display: 'inline', marginLeft: 3 }, disabled: $1.value })
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
    const block = IfElse([
      [() => component === 'Hello', $if_block],
      [() => component === 'A', $if_block2],
      [() => component === 'B', $if_block3],
    ], false)
    const s = Space()
    const p = Element('p')
    /**/ const text = Text('Show Component: ', p)
    /**/ const list = List(['Hello', 'A', 'B'], $list_block, p)

    // create updates
    const component_up = () => banchUpdate(
      block,
      list
    )

    // register nodes
    this.register(block, s, p)
  }
}
