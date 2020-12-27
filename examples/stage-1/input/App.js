import {
  Component,
  Dirty,
  Element,
  Memo,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let name = 'World'
    function onChange(e) {
      name = e.target.value
    }
    function reset() {
      name = 'World'
    }

    // create nodes
    const nodes = [
      Element(
        'p',
        null,
        'Hello ',
        Text(Memo(() => name, [0])),
        '!'
      ),
      Element(
        'input',
        {
          value: Memo(() => name, [0]),
          onInput: Dirty(onChange, [0])
        }
      ),
      Element(
        'button',
        {
          onClick: Dirty(reset, [0])
        },
        'Reset'
      )
    ]

    // register nodes
    this.register(nodes)
  }
}
