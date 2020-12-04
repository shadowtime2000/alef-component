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
      this.nodes.forEach(node => {
        if (node instanceof IfBlock) {
          node.disposes.forEach(dispose => dispose())
        }
        remove(this.el, node)
      })
    }
  }
}

export class IfBlock {
  nodes = []
  check = () => false
  disposes = []
  placeholder = document.createComment('if-block')
  constructor(check, nodes, disposes = []) {
    this.check = check
    this.nodes = nodes
    this.disposes = disposes
  }
  toggle() {
    const { parentNode } = this.placeholder
    if (parentNode) {
      if (this.check()) {
        this.nodes.forEach(node => parentNode.insertBefore(node, this.placeholder))
      } else {
        this.nodes.forEach(node => remove(parentNode, node))
      }
    }
  }
}

export function Element(name, props, parent) {
  const el = document.createElement(name)
  if (parent) {
    append(parent, el)
  }
  if (typeof props === 'object' || props !== null) {
    for (const key in props) {
      el.setAttribute(key, String(props[key]))
    }
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

export function space() {
  return Text(' ')
}

export function setText(node, text) {
  node.textContent = String(text)
}

export function setValue(node, text) {
  node.value = String(text)
}

export function listen(el, evName, callback, update) {
  const cb = e => {
    callback(e)
    update()
  }
  el.addEventListener(evName, cb)
  return () => el.removeEventListener(evName, cb)
}

export function append(parent, child) {
  if (child instanceof IfBlock) {
    parent.appendChild(child.placeholder)
    child.toggle(child.check())
  } else {
    parent.appendChild(child)
  }
}

export function remove(parent, child) {
  if (child instanceof IfBlock) {
    child.nodes.forEach(node => parent.remove(node))
    parent.removeChild(child.placeholder)
  } else {
    if (child.parentNode === parent) {
      parent.removeChild(child)
    }
  }
}
