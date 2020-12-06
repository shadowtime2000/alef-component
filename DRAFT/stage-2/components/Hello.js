import {
  Component,
  Element,
  Text
} from '../../../lib/helper.js'

export default class Hello extends Component {
  constructor(props) {
    super(props)

    // props
    const $name = () => this.props.name

    // create nodes
    const p = Element('p')
    const text = Text('Hello ', p)
    const text2 = Text($name(), p)
    const text3 = Text('!', p)

    // create updates
    const $name_up = () => {
      text2.setText($name())
    }

    // listen props changes
    this.onPropChange('name', $name_up)

    // register nodes
    this.register(p)
  }
}
