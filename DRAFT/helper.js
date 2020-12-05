/* alef component helpers */

/** Alef basic component class */
export class Component {
  el = null
  nodes = []
  disposes = []
  styles = []
  mount(el) {
    this.el = el
    this.nodes.forEach(node => this._append(node))
  }
  unmount() {
    this.disposes.forEach(dispose => dispose())
    if (this.el) {
      this.nodes.forEach(node => {
        if (node instanceof IfBlock) {
          node.disposes.forEach(dispose => dispose())
        }
        this._remove(node)
      })
    }
  }
  _append(child) {
    if (child instanceof IfBlock) {
      this.el.appendChild(child.placeholder)
      child.toggle()
    } else if (child instanceof Style) {
      document.head.appendChild(child.el)
      child.update()
    } else {
      this.el.appendChild(child)
    }
  }
  _remove(child) {
    if (child instanceof IfBlock) {
      child.nodes.forEach(node => removeChild(this.el, node))
      removeChild(this.el, child.placeholder)
    } else if (child instanceof Style) {
      removeChild(document.head, child.el)
    } else {
      removeChild(this.el, child)
    }
  }
}

/** A style component apply style */
export class Style {
  el = document.createElement('style')
  constructor(id, templateFn) {
    this.id = id
    this.templateFn = templateFn
    this.el.setAttribute('id', 'alef-' + id)
  }
  update() {
    this.el.innerHTML = this.templateFn(this.id)
  }
}

/** A block component to handle conditional rendering */
export class IfBlock {
  placeholder = document.createComment('if-block')
  validate = () => false
  nodes = []
  disposes = []
  constructor(validate, init) {
    this.validate = validate
    init(this)
  }
  toggle() {
    const { parentNode } = this.placeholder
    if (parentNode) {
      if (this.validate()) {
        this.nodes.forEach(node => parentNode.insertBefore(node, this.placeholder))
      } else {
        this.nodes.forEach(node => removeChild(parentNode, node))
      }
    }
  }
}

/** Create a document element */
export function Element(name, props, parent) {
  const el = document.createElement(name)
  if (parent) {
    parent.appendChild(el)
  }
  if (typeof props === 'object' || props !== null) {
    for (const key in props) {
      el.setAttribute(key, String(props[key]))
    }
  }
  return el
}

/** Create a Text node */
export function Text(text, parent) {
  const tn = document.createTextNode(String(text))
  if (parent) {
    parent.appendChild(tn)
  }
  return tn
}

/** A shortcut for `Text(' ')` */
export function space() {
  return Text(' ')
}

/** Set the text content of Text node */
export function setText(node, text) {
  node.textContent = String(text)
}

/** Set the value of the form Element */
export function setValue(node, text) {
  node.value = String(text)
}

/** Listen an event for the element */
export function listen(el, evName, callback, update) {
  const cb = e => {
    callback(e)
    update()
  }
  el.addEventListener(evName, cb)
  return () => el.removeEventListener(evName, cb)
}

/** Remove the child from its parent */
export function removeChild(parent, child) {
  if (child.parentNode === parent) {
    parent.removeChild(child)
  }
}

const idTable = '1234567890abcdefghijklmnopqrstuvwxyz'
const idLen = 6

/** Create and return a style unique ID */
export function StyleId() {
  let id = idTable.slice(10).charAt(Math.floor(26 * Math.random())) // starts with a-z
  for (let i = 0; i < idLen - 1; i++) {
    id += idTable.charAt(Math.floor(36 * Math.random()))
  }
  return id
}
