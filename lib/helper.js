/* Alef Component Helpers */

/** Alef basic component class. */
export class Component {
  nodes = []
  slots = []
  listeners = new Map()
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
      this.nodes.forEach(node => dom.appendNode(el, node))
    }
  }
  unmount() {
    if (!this.mounted) {
      this.mounted = false
      this.nodes.forEach(node => dom.removeNode(node))
    }
  }
  onMount(...effects) {

  }
  update(key, value) {
    this.props[key] = value
    this.listeners.get(key)?.forEach(callback => callback()) // todo: push to asynchronous update queue
  }
  listen(key, ...callbacks) {
    let a = this.listeners.get(key)
    if (!a) {
      a = []
      this.listeners.set(key, a)
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
        switch (key) {
          case 'className':
            this.el.className = String(props[key])
            break
          case 'value':
            this.el.value = String(props[key])
            break
          default:
            this.el.setAttribute(key, String(props[key]))
            break
        }
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
    const { el } = this
    const val = String(value)
    switch (key) {
      case 'className':
        el.className = val
        break
      case 'value':
        if (el.value.length != val.length && el.value != val) {
          el.value = val
        }
        break
      default:
        el.setAttribute(key, val)
        break
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
      this.nodes.forEach(node => dom.appendNode(this.el, node))
      this.events.forEach(({ name, callback, updates }) => {
        const cb = e => {
          if (callback(e) !== false) {
            updates.forEach(update => update()) // todo: push to asynchronous update queue
          }
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
      this.nodes.forEach(node => dom.removeNode(node))
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
  isTrue = false
  constructor(validate, parent) {
    this.validate = validate
    if (parent) {
      parent.appendChild(this)
    }
  }
  appendChild(child) {
    this.nodes.push(child)
  }
  update() {
    if (this.validate()) {
      this.truify()
    } else {
      this.falsify()
    }
  }
  truify() {
    if (!this.isTrue) {
      this.isTrue = true
      this.nodes.forEach(node => dom.insertNode(node, this.placeholder))
    }
  }
  falsify() {
    if (this.isTrue) {
      this.isTrue = false
      this.nodes.forEach(node => dom.removeNode(node))
    }
  }
}

/** Create and return a new if block. */
export function If(validate, parent) {
  return new IfBlock(validate, parent)
}

/** If-Else block to handle conditional rendering. */
export class IfElseBlock {
  constructor(validate, parent) {
    this.if = new IfBlock(validate )
    this.else = new IfBlock(()=>!validate() )
    if (parent) {
      parent.appendChild(this)
    }
  }
  update() {
    this.if.update()
    this.else.update()
  }
}

/** Create and return a new if-else block. */
export function IfElse(validate, parent) {
  return new IfElseBlock(validate, parent)
}

/** List node for List block. */
export class ListNode {
  constructor(item, index, { node, key, up }) {
    this.item = item
    this.index = index
    this.node = node
    if (typeof key === 'string' && key.length > 0) {
      this.key = '$' + key
    } else if (typeof key === 'number' && !Number.isNaN(key)) {
      this.key = '%' + key.toString(16)
    } else {
      this.key = '#' + key.toString(16)
    }
    this.update = up
  }
}

/** List block for map rendering. */
export class ListBlock {
  nodes = []
  placeholder = document.createTextNode('')
  constructor(get, block, parent) {
    this.get = get
    this.block = block
    const items = get()
    if (Array.isArray(items)) {
      this.nodes = items.map((item, index) => new ListNode(item, index, block(item)))
    }
    if (parent) {
      parent.appendChild(this)
    }
  }
  update() {
    this.nodes.forEach(node => node.update())
  }
}

/** Create and return a new list block. */
export function List(get, block, parent) {
  return new ListBlock(get, block, parent)
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
  update(text) {
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

const dom = {
  /** Append the node to DOM */
  appendNode(root, node) {
    if (isComponent(node)) {
      node.nodes.forEach(node => this.appendNode(root, node))
    } else if (node instanceof AlefElement) {
      root.appendChild(node.el)
      node.activate()
    } else if (node instanceof IfBlock) {
      root.appendChild(node.placeholder)
      node.update()
    } else if (node instanceof IfElseBlock) {
      root.appendChild(node.if.placeholder)
      root.appendChild(node.else.placeholder)
      node.if.update()
      node.else.update()
    }  else if (node instanceof AlefStyle) {
      document.head.appendChild(node.el)
      node.update()
    } else if (node instanceof AlefText) {
      root.appendChild(node.node)
    }
  },
  /** insert the node before given anchor. */
  insertNode(node, anchor) {
    const { parentNode } = anchor
    if (parentNode) {
      if (isComponent(node)) {
        node.nodes.forEach(node => this.insertNode(node, anchor))
      } else if (node instanceof AlefElement) {
        parentNode.insertBefore(node.el, anchor)
        node.activate()
      } else if (child instanceof IfBlock) {
        parentNode.insertBefore(node.placeholder, anchor)
        node.update()
      } else if (child instanceof IfElseBlock) {
        parentNode.insertBefore(node.if.placeholder, anchor)
        parentNode.insertBefore(node.else.placeholder, anchor)
        node.if.update()
        node.else.update()
      } else if (child instanceof AlefStyle) {
        document.head.appendChild(node.el)
        node.update()
      } else if (child instanceof AlefText) {
        parentNode.insertBefore(node.node, anchor)
      }
    }
  },
  /** Remove the node from its parent. */
  removeNode(node) {
    if (isComponent(node)) {
      node.nodes.forEach(node => this.removeNode(node))
    } else if (node instanceof AlefElement) {
      node.deactivate()
      removeEl(node.el)
    } else if (node instanceof IfBlock) {
      node.falsify()
      removeEl(node.placeholder)
    } else if (node instanceof IfBlock) {
      node.if.falsify()
      node.else.falsify()
      removeEl(node.if.placeholder)
      removeEl(node.else.placeholder)
    } else if (node instanceof AlefStyle) {
      removeEl(node.el)
    } else if (node instanceof AlefText) {
      removeEl(node.node)
    }
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
