/* alef component helpers */

export class Component {
  el = null
  nodes = []
  disposes = []
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

export function Element(name, parent) {
  const el = document.createElement(name)
  if (parent) {
    append(parent, el)
  }
  return el
}

export function Text(text, parent) {
  const node = document.createTextNode(String(text))
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

export function setTextContent(node, text) {
  node.textContent = String(text)
}

export function listen(el, evName, callback, update) {
  const cb = () => {
    callback()
    update()
  }
  el.addEventListener(evName, cb)
  return () => el.removeEventListener(evName, cb)
}
