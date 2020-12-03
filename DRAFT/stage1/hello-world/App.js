/* --- alef helper code --- */

function append(parent, child) {
    parent.appendChild(child)
}

function remove(parent, child) {
    parent.removeChild(child)
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

/* --- END --- */

export default class App {
    constructor() {
        let name = 'world'

        const p = Element('p')
        const t = Text(`hello ${name}!`, p)
        
        this.nodes = [p]
    }
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
