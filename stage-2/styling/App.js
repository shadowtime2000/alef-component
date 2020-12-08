import {
  Component,
  Element,
  Space,
  Style,
  StyleId,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // create style ids
    const sid = StyleId() // todo(stage-3): get ssr id

    // strip types
    let n = 0

    // create nodes
    const p = Element('p', { className: sid })
    const text = Text('current count is ', p)
    const text2 = Text(n, p)
    const s = Space()
    const button = Element('button', { className: sid })
    const text3 = Text('-', button)
    const s2 = Space()
    const button2 = Element('button', { className: sid })
    const text4 = Text('+', button2)
    const style = Style(sid, id => `
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

    // create updates
    const n_up = () => {
      text2.update(n)
      style.update()
    }

    // listen events
    button.listen('click', () => { n-- }, n_up)
    button2.listen('click', () => { n++ }, n_up)

    // register nodes
    this.register(style, p, s, button, s2, button2)
  }
}
