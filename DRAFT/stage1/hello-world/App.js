import { Component, Element, Text } from '../helper.js'

export default class App extends Component {
  constructor() {
    super()

    let name = 'world'

    const p = Element('p')
    const t = Text(`hello ${name}!`, p)

    this.nodes = [p]
  }
}
