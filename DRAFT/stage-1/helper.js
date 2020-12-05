/* alef component helpers */

/** Alef basic component class */
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
        this.nodes.forEach(node => remove(parentNode, node))
      }
    }
  }
}

/** Create a document element */
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

/** Create a Text node */
export function Text(text, parent) {
  const node = document.createTextNode(String(text))
  if (parent) {
    append(parent, node)
  }
  return node
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

/** Append the child node to the parent */
export function append(parent, child) {
  if (child instanceof IfBlock) {
    parent.appendChild(child.placeholder)
    child.toggle(child.validate())
  } else {
    parent.appendChild(child)
  }
}

/** Remove the child from it's parent */ 
export function remove(parent, child) {
  if (child instanceof IfBlock) {
    child.nodes.forEach(node => parent.removeChild(node))
    parent.removeChild(child.placeholder)
  } else {
    if (child.parentNode === parent) {
      parent.removeChild(child)
    }
  }
}
