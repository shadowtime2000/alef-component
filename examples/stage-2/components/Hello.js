import {
  Component,
  Element,
  Style,
  Text
} from '../../lib/helper.js'

export default class Hello extends Component {
  constructor(props) {
    super(props)

    // props
    const $name = () => this.props.name || 'World'

    // create styles
    const style = Style(id => `
      p {
        padding: 12px;
        border: 1px dashed #999;
      }
    `)

    // create nodes
    const p = Element('p', { className: style.id })
    /**/ const text = Text('Hello ', p)
    /**/ const text2 = Text($name(), p)
    /**/ const text3 = Text('!', p)

    // create updates
    const $name_up = () => {
      text2.update($name())
    }

    // listen props changes
    this.listen('name', $name_up)

    // register nodes
    this.register(style,p)
  }
}
