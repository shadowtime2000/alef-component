/* --- alef helper code --- */

class Component {
  el = null
  nodes = []
  disposes = []
  mount(el) {
    this.el = el
    this.nodes.forEach(node => append(el, node))
  }
  unmount() {
    if (this.el) {
      this.nodes.forEach(node => remove(this.el, node))
    }
    this.disposes.forEach(dispose => dispose())
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

function setTextContent(node, text) {
  node.textContent = String(text)
}

function listen(el, evName, callback, update) {
  const cb = () => {
    callback()
    update()
  }
  el.addEventListener(evName, cb)
  return () => el.removeEventListener(evName, cb)
}

/* --- END --- */

export default class App extends Component {
  constructor() {
    super()

    let n = 0

    // button[0].onClick
    const handler = () => {
      n--
    }
    // button[1].onClick
    const handler2 = () => {
      n++
    }

    let p = Element('p')
    let t = Text('current count is ', p)
    let t2 = Text(n, p)
    let button = Element('button')
    let t3 = Text('-', button)
    let t4 = Text('\n')
    let button2 = Element('button')
    let t5 = Text('+', button2)

    this.nodes = [p, button, t4, button2]
    this.disposes = [
      listen(button, 'click', handler, () => {
        setTextContent(t2, n)
      }),
      listen(button2, 'click', handler2, () => {
        setTextContent(t2, n)
      })
    ]
  }
}
