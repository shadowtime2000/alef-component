import {
  Component,
  Element,
  Text,
  If,
  IfElse,
  banchUpdate
} from '../../../lib/helper.js'
import { nope } from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types
    let text = 'El Aleph'
    let show = false
    let ok = false
    function toggle() {
      ok = !ok // dirty data: ok
    }

    // create blocks
    const $if_block = () => {
      // create blocks
      const $if_block2 = () => {
        // create nodes
        const code = Element('code')
        /**/ const text2 = Text(text, code)

        return { node: code, update: nope }
      }
      const $if_block3 = () => {
        // create nodes
        const code2 = Element('code')
        /**/ const text3 = Text('*'.repeat(text.length), code2)

        return { node: code2, update: nope }
      }
      const $if_block4 = () => {
        // create nodes
        const button = Element('button')
        /**/ const text4 = Text('Hide', button)

        // listen events
        button.listen('click', () => { show = false }, show_up)

        return { node: button, update: nope }
      }
      const $if_block5 = () => {
        // create nodes
        const button2 = Element('button')
        /**/ const text5 = Text('Show', button2)

        // listen events
        button2.listen('click', () => { show = true }, show_up)

        return { node: button2, update: nope }
      }

      // create nodes
      const p = Element('p')
      /**/ const if1 = If(() => show, $if_block2, false, p)
      /**/ const if2 = If(() => !show, $if_block3, false, p)
      /**/ const text6 = Text(' ', p)
      /**/ const if3 = If(() => show, $if_block4, false, p)
      /**/ const if4 = If(() => !show, $if_block5, false, p)

      // create updates
      const show_up = () => banchUpdate(
        if1,
        if2,
        if3,
        if4
      )

      return { node: p, update: nope }
    }
    const $if_block6 = () => {
      // create nodes
      const button3 = Element('button')
      /**/ const text4 = Text('OFF', button3)

      // listen events
      button3.listen('click', toggle, ok_up)

      return { node: button3, update: nope }
    }
    const $if_block7 = () => {
      // create nodes
      const button4 = Element('button')
      /**/ const text5 = Text('ON', button4)

      // listen events
      button4.listen('click', toggle, ok_up)

      return { node: button4, update: nope }
    }

    // create nodes
    const if5 = If(() => ok, $if_block, false)
    const if6 = IfElse([
      [() => ok, $if_block6],
      [() => true, $if_block7]
    ])

    // create updates
    const ok_up = () => banchUpdate(
      if5,
      if6
    )

    // register nodes
    this.register(if5, if6)
  }
}
