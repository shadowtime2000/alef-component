import {
  Component,
  Element,
  listen,
  setText,
  space,
  Style,
  StyleId,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // create style ids
    const sid = StyleId() // todo(stage-3): get ssr id

    // initiate state
    let n = 0

    // create nodes
    const p = Element('p', { class: sid })
    const t = Text('current count is ', p)
    const t2 = Text(n, p)
    const s = space()
    const button = Element('button', { class: sid })
    const t3 = Text('-', button)
    const s2 = space()
    const button2 = Element('button', { class: sid })
    const t4 = Text('+', button2)

    // create styles
    const style = new Style(sid, id => `
/* unused h1 */
/*
  h1 {
    font-size: 200%;
  }
*/
p.${id} {
  color: ${Math.abs(n) >= 10 ? 'red' : 'green'}    
}
button.${id} {
  display: inline-block;
  width: 24px;
  height: 24px;
  font-weight: bold;
}
`)

    // event handles
    const _1 /* button[0].onClick */ = () => {
      n-- // dirty data: n
    }
    const _2 /* button[1].onClick */ = () => {
      n++ // dirty data: n
    }

    // create updates
    const n_up = () => {
      setText(t2, n)
      style.update()
    }

    // register nodes
    this.nodes = [style, p, s, button, s2, button2]

    // listen events
    this.disposes = [
      listen(button, 'click', _1, n_up),
      listen(button2, 'click', _2, n_up)
    ]
  }
}
