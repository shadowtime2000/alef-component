import {
  banchUpdate,
  Component,
  Element,
  Space,
  Style,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types
    let n = 0

    // create styles
    const style = Style(id => `
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

    // create nodes
    const p = Element('p', { className: style.id })
    /**/ const text = Text('current count is ', p)
    /**/ const text2 = Text(n, p)
    const s = Space()
    const button = Element('button', { className: style.id })
    /**/ const text3 = Text('-', button)
    const s2 = Space()
    const button2 = Element('button', { className: style.id })
    /**/ const text4 = Text('+', button2)

    // create updates
    const n_up = () => banchUpdate(
      [text2, n],
      style
    )

    // listen events
    button.listen('click', () => { n-- }, n_up)
    button2.listen('click', () => { n++ }, n_up)

    // register nodes
    this.register(style, p, s, button, s2, button2)
  }
}
