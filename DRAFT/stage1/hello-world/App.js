/* --- alef helper code --- */

class Component {
  el = null
  disposes = []
  nodes = []
  mount(el) {
    this.el = el
    this.nodes.forEach(node => append(el, node))
  }
  unmount() {
    this.disposes.forEach(dispose => dispose())
    if (this.el) {
      this.nodes.forEach(node => remove(this.el, node))
    }
  }
}

function Element(name, parent) {
  const el = document.createElement(name)
  if (parent) {
    append(parent, el)
  }
  return el
}

function Text(text, parent) {
  const node = document.createTextNode(text)
  if (parent) {
    append(parent, node)
  }
  return node
}

function append(parent, child) {
  parent.appendChild(child)
}

function remove(parent, child) {
  parent.removeChild(child)
}

/* --- END --- */

export default class App extends Component {
  constructor() {
    super()

    let name = 'world'

    const p = Element('p')
    const t = Text(`hello ${name}!`, p)

    this.nodes = [p]
  }
}
