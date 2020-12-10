import {
  Component,
  Element,
  Text,
  If,
  IfElse,
  banchUpdate
} from '../../../lib/helper.js'

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

    // create nodes
    const block = If(() => ok)
    /**/ const p = Element('p', block)
    /***/ const block2 = If(() => show, p)
    /****/ const code = Element('code', block2)
    /*****/ const text2 = Text(text, code)
    /***/ const block3 = If(() => !show, p)
    /****/ const code2 = Element('code', block3)
    /*****/ const text3 = Text('*'.repeat(text.length), code2)
    /***/ const span = Element('span', p)
    /****/ const text4 = Text(' ' /* &nbsp; */, span)
    /***/ const block4 = If(() => show, p)
    /****/ const button = Element('button', block4)
    /*****/ const text5 = Text('Hide', button)
    /***/ const block5 = If(() => !show, p)
    /****/ const button2 = Element('button', block5)
    /*****/ const text6 = Text('Show', button2)
    const block6 = IfElse(() => !ok)
    /**/ const button3 = Element('button', block6.if)
    /***/ const text7 = Text('ON', button3)
    /**/ const button4 = Element('button', block6.else)
    /***/ const text8 = Text('OFF', button4)

    // create updates
    const show_up = () => banchUpdate(
      block2,
      block3,
      block4,
      block5
    )
    const ok_up = () => banchUpdate(
      block,
      block6
    )

    // listen events
    button.listen('click', () => { show = false }, show_up)
    button2.listen('click', () => { show = true }, show_up)
    button3.listen('click', toggle, ok_up)
    button4.listen('click', toggle, ok_up)

    // register nodes
    this.register(block, block6)
  }
}
