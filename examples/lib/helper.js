/* Alef Component Helpers */

/** Alef basic component class. */
export class Component {
  nodes = []
  slots = []
  effects = []
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
      this.effects.forEach(effect => effect.update())
    }
  }
  unmount() {
    if (this.mounted) {
      this.mounted = false
      this.nodes.forEach(node => dom.removeNode(node))
    }
  }
  onMount(...effects) {
    this.effects.push(...effects)
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
        this.update(key, props[key])
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
    if (typeof value === 'function') {
      value = value()
    }
    const nullValue = value === undefined || value === null
    switch (key) {
      case 'class':
      case 'className':
        if (nullValue) {
          el.className = ''
          el.removeAttribute('class')
        } else {
          el.className = String(value)
        }
        break
      case 'value':
        if (nullValue) {
          el.value = ''
        } else {
          const val = String(value)
          if (el.value.length != val.length && el.value != val) {
            el.value = val
          }
        }
        break
      case 'disabled':
      case 'checked':
        el[key] = !!value && value !== 'false'
        break
      default:
        if (nullValue) {
          el.removeAttribute(key)
        } else {
          el.setAttribute(key, String(value))
        }
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
  mount() {
    if (!this.activated) {
      this.activated = true
      this.nodes.forEach(node => dom.appendNode(this.el, node))
      this.events.forEach(({ name, callback, updates }) => {
        const cb = e => {
          // todo: push to asynchronous update queue
          if (callback(e) !== false) {
            updates.forEach(update => update())
          }
        }
        this.el.addEventListener(name, cb)
        this.disposes.push(() => this.el.removeEventListener(name, cb))
      })
    }
  }
  unmount() {
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
    this.if = new IfBlock(validate)
    this.else = new IfBlock(() => !validate())
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

/** List block for map rendering. */
export class ListBlock {
  nodes = []
  placeholder = document.createTextNode('')
  constructor(get, block, parent) {
    this.get = get
    this.block = block
    if (parent) {
      parent.appendChild(this)
    }
  }
  update() {
    const items = Array.isArray(this.get) ? this.get : this.get()
    const newNodes = []
    if (Array.isArray(items)) {
      items.forEach((item, index) => {
        const block = this.block(item)
        const key = computeLiKey(index, block.key)
        const prev = this.nodes.find(n => n.item === item || n.key === key)
        if (prev) {
          if (prev.item !== item) {
            prev.item = item
            prev.update(true, item)
          } else {
            prev.update()
          }
          prev.index = index
          prev.key = key
          newNodes.push(prev)
        } else {
          const { node, update } = block.create()
          newNodes.push({ item, index, key, node, update })
        }
      })
    }
    const { parentNode } = this.placeholder
    if (parentNode) {
      const indexs = []
      this.nodes.forEach((node) => {
        if (newNodes.length === 0 || newNodes.findIndex(newNode => newNode === node) === -1) {
          // remove non-existent nodes
          dom.removeNode(node.node)
        } else {
          indexs.push([indexs.length, node.index])
        }
      })
      newNodes.forEach((newNode) => {
        if (this.nodes.length === 0 || this.nodes.findIndex(node => newNode === node) === -1) {
          // append new nodes
          dom.insertNode(newNode.node, this.placeholder)
          indexs.push([indexs.length, newNode.index])
        }
      })
      // fix order
      const { childNodes } = parentNode
      const moving = []
      let placeholderIndex = 0
      for (let i = 0; i < childNodes.length; i++) {
        if (childNodes.item(i) === this.placeholder) {
          placeholderIndex = i
          break
        }
      }
      const domStartIndex = placeholderIndex - newNodes.length
      indexs.forEach(([domIndex, treeIndex]) => {
        if (domIndex !== treeIndex) {
          const realDomIndex = domStartIndex + domIndex
          const node = childNodes.item(realDomIndex)
          moving.push([node, realDomIndex, domStartIndex + treeIndex])
        }
      })
      moving.forEach(([node, domIndex, treeIndex]) => {
        if (node !== childNodes.item(treeIndex)) {
          parentNode.insertBefore(node, childNodes.item(treeIndex + 1))
        }
      })
    }
    this.nodes = newNodes
  }
  unmount() {
    this.nodes.forEach(({ node }) => dom.removeNode(node))
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
export function Style(templateFn) {
  const id = StyleId() // todo(stage-3): get ssr id
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
    if (typeof text === 'function') {
      text = text()
    }
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

/** Create and return a memo. */
export function Memo(update) {
  return {
    value: update(),
    update() {
      return this.value = update()
    }
  }
}

/** Create and return a effect. */
export function Effect(update) {
  return {
    update
  }
}

/** Banch update helper. */
export function banchUpdate(...updates) {
  updates.forEach((updater, index) => {
    if (Array.isArray(updater) && updater.length > 0) {
      updater.shift().update(...updater)
    } else if (typeof updater === 'object' && updater !== null && typeof updater.update === 'function') {
      updater.update()
    } else if (updater !== false) {
      console.warn(`invald updater(${index}):`, updater)
    }
  })
}

/* Returns a empty callback */
export function nope() { }

const dom = {
  /** Append the node to DOM */
  appendNode(parentNode, node) {
    if (isComponent(node)) {
      node.nodes.forEach(node => dom.appendNode(parentNode, node))
    } else if (node instanceof AlefElement) {
      appendEl(parentNode, node.el)
      node.mount()
    } else if (node instanceof IfBlock) {
      appendEl(parentNode, node.placeholder)
      node.update()
    } else if (node instanceof IfElseBlock) {
      appendEl(parentNode, node.if.placeholder)
      appendEl(parentNode, node.else.placeholder)
      node.update()
    } else if (node instanceof ListBlock) {
      appendEl(parentNode, node.placeholder)
      node.update()
    } else if (node instanceof AlefStyle) {
      appendEl(document.head, node.el)
      node.update()
    } else if (node instanceof AlefText) {
      appendEl(parentNode, node.node)
    }
  },

  /** insert the node before given refEl. */
  insertNode(node, refEl) {
    if (isComponent(node)) {
      node.nodes.forEach(node => dom.insertNode(node, refEl))
    } else if (node instanceof AlefElement) {
      insertEl(node.el, refEl)
      node.mount()
    } else if (node instanceof IfBlock) {
      insertEl(node.placeholder, refEl)
      node.update()
    } else if (node instanceof IfElseBlock) {
      insertEl(node.if.placeholder, refEl)
      insertEl(node.else.placeholder, refEl)
      node.update()
    } else if (node instanceof ListBlock) {
      insertEl(node.placeholder, refEl)
      node.update()
    } else if (node instanceof AlefStyle) {
      appendEl(document.head, node.el)
      node.update()
    } else if (node instanceof AlefText) {
      insertEl(node.node, refEl)
    }
  },

  /** Remove the node from its parent. */
  removeNode(node) {
    if (isComponent(node)) {
      node.nodes.forEach(node => dom.removeNode(node))
    } else if (node instanceof AlefElement) {
      node.unmount()
      removeEl(node.el)
    } else if (node instanceof IfBlock) {
      node.falsify()
      removeEl(node.placeholder)
    } else if (node instanceof IfElseBlock) {
      node.if.falsify()
      node.else.falsify()
      removeEl(node.if.placeholder)
      removeEl(node.else.placeholder)
    } else if (node instanceof ListBlock) {
      node.unmount()
      removeEl(node.placeholder)
    } else if (node instanceof AlefStyle) {
      removeEl(node.el)
    } else if (node instanceof AlefText) {
      removeEl(node.node)
    }
  }
}

/** Append an element to parent element. */
function appendEl(parent, el) {
  parent.appendChild(el)
}

/** Insert an element to befor ref element. */
function insertEl(el, refEl) {
  const { parentNode } = refEl
  if (parentNode) {
    parentNode.insertBefore(el, refEl)
  }
}

/** Remove the element from DOM. */
function removeEl(el) {
  const { parentNode } = el
  if (parentNode) {
    parentNode.removeChild(el)
  }
}

/** Check object whether is a component. */
function isComponent(obj) {
  if (!(typeof obj === 'object' && obj !== null)) {
    return false
  }
  return obj instanceof Component || obj.__proto__ instanceof Component
}

/**
 * Compute list item key, use index instead of the key
 * when it is not string or number
 */
function computeLiKey(index, key) {
  if (typeof key === 'string' && key.length > 0) {
    return '$' + key
  } else if (typeof key === 'number' && !Number.isNaN(key)) {
    return '%' + key.toString(16)
  } else {
    return '#' + index.toString(16)
  }
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
