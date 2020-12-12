import {
  Component,
  Effect,
  Element,
  Style,
  Text
} from '../../lib/helper.js'

export default class B extends Component {
  constructor(props) {
    super(props)

    // create effects
    const $$effect = Effect(() => {
      console.log('component B mounted')
      return () => console.log('component B unmounted')
    })

    // create styles
    const style = Style(id => `
p.${id} {
  padding: 12px;
  border: 1px dashed green;
  color: green;
}
`)

    // create nodes
    const p = Element('p', { className: style.id })
      /**/ const text = Text('A', p)

    // listen effects
    this.onMount($$effect)

    // register nodes
    this.register(style, p)
  }
}
