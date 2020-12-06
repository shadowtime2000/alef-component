/* Alef Component Helpers */

/** Alef basic component class. */
export class Component {
  nodes = []
  slots = []
  propChangeCallbacks = new Map()
  mounted = false 
  constructor(props = {}) {
    this.props = props
  }
  register(...nodes) {
    this.nodes = nodes
  }
  appendChild(slot) {
    this.slots.push(slot)
  }
  mount(el) {
    if (!this.mounted) {
      this.mounted = true
      this.nodes.forEach(node => appendNodeToDom(el, node))
    }
  }
  unmount() {
    if (!this.mounted) {
      this.mounted = false
      this.nodes.forEach(node => removeNode(node))
    }
  }
  update(key, value) {
    this.props[key] = value
    this.propChangeCallbacks.get(key)?.forEach(callback => callback()) // todo: push to asynchronous update queue
  }
  onPropChange(key, ...callbacks) {
    let a = this.propChangeCallbacks.get(key)
    if (!a) {
      a = []
      this.propChangeCallbacks.set(key, a)
    }
    a.push(...callbacks)
  }
}

/** Alef element node. */
export class AlefElement {
  nodes = []
  events = []
  disposes = []
  activated = false
  isClosed = false
  constructor(name, props, parent) {
    this.el = document.createElement(name)
    if (isComponent(props) || props instanceof AlefElement || props instanceof IfBlock) {
      props.appendChild(this)
    } else if (typeof props === 'object' || props !== null) {
      for (const key in props) {
        this.el.setAttribute(key, String(props[key]))
      }
    }
    if (isComponent(parent) || parent instanceof AlefElement || parent instanceof IfBlock) {
      parent.appendChild(this)
    }
  }
  appendChild(child) {
    if (!this.isClosed) {
      this.nodes.push(child)
    } else {
      console.warn(this.el.tagName, 'is closed')
    }
  }
  update(key, value) {
    if (key === 'value') {
      this.el.value = value
    } else {
      this.el.setAttribute(key, value)
    }
  }
  listen(name, callback, ...updates) {
    this.events.push({
      name,
      callback,
      updates,
    })
  }
  activate() {
    if (!this.activated) {
      this.activated = true
      this.nodes.forEach(node => appendNodeToDom(this.el, node))
      this.events.forEach(({ name, callback, updates }) => {
        const cb = e => {
          callback(e)
          updates.forEach(update => update()) // todo: push to asynchronous update queue
        }
        this.el.addEventListener(name, cb)
        this.disposes.push(() => this.el.removeEventListener(name, cb))
      })
    }
  }
  deactivate() {
    if (this.activated) {
      this.activated = false
      this.disposes.forEach(dispose => dispose())
      this.disposes = []
      this.nodes.forEach(node => removeNode(node))
    }
  }
}

/** Create and return a new element node. */
export function Element(name, props, parent) {
  return new AlefElement(name, props, parent)
}

/** If block to handle conditional rendering. */
export class IfBlock {
  nodes = []
  placeholder = document.createTextNode('')
  constructor(validate, parent) {
    this.validate = validate
    if (parent) {
      parent.appendChild(this)
    }
  }
  appendChild(child) {
    this.nodes.push(child)
  }
  toggle() {
    if (this.nodes.length > 0 && this.validate()) {
      this.nodes.forEach(node => insertNode(node, this.placeholder))
    } else {
      this.falsify()
    }
  }
  falsify() {
    this.nodes.forEach(node => removeNode(node))
  }
}

/** Create and return a new if block. */
export function If(validate, parent) {
  return new IfBlock(validate, parent)
}

/** Alef style node to apply style. */
export class AlefStyle {
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

/** Create and return a new style node. */
export function Style(id, templateFn) {
  return new AlefStyle(id, templateFn)
}

/** Alef text node to display text. */
export class AlefText {
  constructor(text, parent) {
    this.node = document.createTextNode(text)
    if (parent) {
      parent.appendChild(this)
    }
  }
  setText(text) {
    this.node.textContent = text
  }
}

/** Create and return a new text node. */
export function Text(text, parent) {
  return new AlefText(text, parent)
}

/** A shortcut for `new Text(' ')`. */
export function Space(parent) {
  return new AlefText(' ', parent)
}

/** Append the node to DOM */
function appendNodeToDom(root, node) {
  if (isComponent(node)) {
    node.nodes.forEach(node => appendNodeToDom(root, node))
  } else if (node instanceof AlefElement) {
    root.appendChild(node.el)
    node.activate()
  } else if (node instanceof IfBlock) {
    root.appendChild(node.placeholder)
    node.toggle()
  } else if (node instanceof AlefStyle) {
    document.head.appendChild(node.el)
    node.update()
  } else if (node instanceof AlefText) {
    root.appendChild(node.node)
  }
}

/** insert the node before given anchor. */
function insertNode(node, anchor) {
  const { parentNode } = anchor
  if (parentNode) {
    if (isComponent(node)) {
      node.nodes.forEach(node => insertNode(node, anchor))
    } else if (node instanceof AlefElement) {
      parentNode.insertBefore(node.el, anchor)
      node.activate()
    } else if (child instanceof IfBlock) {
      parentNode.insertBefore(node.placeholder, anchor)
      node.toggle()
    } else if (child instanceof AlefStyle) {
      document.head.appendChild(node.el)
      node.update()
    } else if (child instanceof AlefText) {
      parentNode.insertBefore(node.node, anchor)
    }
  }
}

/** Remove the node from its parent. */
function removeNode(node) {
  if (isComponent(node)) {
    node.nodes.forEach(node => removeNode(node))
  } else if (node instanceof AlefElement) {
    removeEl(node.el)
    node.deactivate()
  } else if (node instanceof IfBlock) {
    removeEl(node.placeholder)
    node.falsify()
  } else if (node instanceof AlefStyle) {
    removeEl(node.el)
  } else if (node instanceof AlefText) {
    removeEl(node.node)
  }
}

/** Remove element from DOM. */
function removeEl(el) {
  if (el.parentNode) {
    el.parentNode.removeChild(el)
  }
}

/** Check object whether is a component. */
function isComponent(obj) {
  if (!(typeof obj === 'object' && obj !== null)) {
    return false
  }
  return obj instanceof Component || obj.__proto__ instanceof Component
}

const idTable = '1234567890abcdefghijklmnopqrstuvwxyz'
const idLen = 6

/** Create and return a style unique ID. */
export function StyleId() {
  let id = idTable.slice(10).charAt(Math.floor(26 * Math.random())) // starts with a-z
  for (let i = 0; i < idLen - 1; i++) {
    id += idTable.charAt(Math.floor(36 * Math.random()))
  }
  return id
}
