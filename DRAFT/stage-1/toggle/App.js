import {
  Component,
  Element,
  Text,
  If,
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let text = 'El Aleph'
    let show = false
    let ok = false

    // create memos
    const $1 /* {'*'.repeat(text.length)} */ = () => '*'.repeat(text.length) // dep: text

    // create nodes
    const block = If(() => ok)
    /**/ const p = Element('p', block)
    /***/ const block2 = If(() => show, p)
    /****/ const code = Element('code', block2)
    /*****/ const text2 = Text(text, code)
    /***/ const block3 = If(() => !show, p)
    /****/ const code2 = Element('code', block3)
    /*****/ const text3 = Text($1(), code2)
    /***/ const span = Element('span', p)
    /****/ const text4 = Text(' ' /* &nbsp; */, span)
    /***/ const block4 = If(() => show, p)
    /****/ const button = Element('button', block4)
    /*****/ const text5 = Text('Hide', button)
    /***/ const block5 = If(() => !show, p)
    /****/ const button2 = Element('button', block5)
    /*****/ const text6 = Text('Show', button2)
    const block6 = If(() => !ok)
    /**/ const button3 = Element('button', block6)
    /***/ const text7 = Text('ON', button3)
    const block7 = If(() => ok)
    /**/ const button4 = Element('button', block7)
    /***/ const text8 = Text('OFF', button4)

    // create updates
    const text_up = () => {
      // static
    }
    const show_up = () => {
      block2.toggle()
      block3.toggle()
      block4.toggle()
      block5.toggle()
    }
    const ok_up = () => {
      block.toggle()
      block6.toggle()
      block7.toggle()
    }

    // create actions
    const _1 /* button[0].onClick */ = () => {
      show = false // dirty data: show
    }
    const _2 /* button[1].onClick */ = () => {
      show = true // dirty data: show
    }
    function toggle() {
      ok = !ok // dirty data: ok
    }

    // listen events
    button.listen('click', _1, show_up)
    button2.listen('click', _2, show_up)
    button3.listen('click', toggle, ok_up)
    button4.listen('click', toggle, ok_up)

    // register nodes
    this.register(block, block6, block7)
  }
}
