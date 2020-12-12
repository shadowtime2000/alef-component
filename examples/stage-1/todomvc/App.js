import {
  banchUpdate,
  Component,
  Effect,
  Element,
  If,
  List,
  Memo,
  Space,
  Text
} from '../../../lib/helper.js'
import { nope } from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let todos /* Array */ = JSON.parse(localStorage.getItem('todomvc') || '[]')
    let newTodo = ''
    let editedTodo = null
    let visibility = 'all'
    let beforeEditCache = ''
    let save = false
    function addTodo() {
      var title = newTodo.trim()
      if (title) {
        todos.push({
          id: Date.now() + Math.random().toFixed(3),
          completed: false,
          title,
        })
      }
      newTodo = ''
    }
    function editTodo(todo) {
      beforeEditCache = todo.title
      editedTodo = todo
    }
    function cancelEdit(todo) {
      editedTodo = null
      todo.title = beforeEditCache
    }
    function doneEdit(todo) {
      editedTodo = null
      todo.title = todo.title.trim()
      if (!todo.title) {
        removeTodo(todo)
      }
    }
    function removeTodo(todo) {
      todos.splice(todos.indexOf(todo), 1)
    }
    function clearCompleted() {
      todos = todos.filter(todo => !todo.completed)
    }

    // create memos
    const $filteredTodos /* dep: todos, visibility */ = Memo(() => todos.filter(todo => {
      switch (visibility) {
        case 'all': return true
        case 'active': return !todo.completed
        case 'completed': return todo.completed
      }
    }))
    const $remaining /* dep: todos */ = Memo(() => todos.filter(todo => !todo.completed).length)

    // create effects
    const $$effect /* dep: todos */ = Effect(() => {
      if (save) {
        localStorage.setItem('todomvc', JSON.stringify(todos))
      } else {
        save = true
      }
    })

    const $if_block = () => {
      const section = Element('section', { className: 'main' })
      /**/ const input2 = Element('input', { id: 'toggle-all', className: 'toggle-all', type: 'checkbox', checked: $remaining.value === 0 }, section)
      /**/ const label = Element('label', { for: 'toggle-all' }, section)
      /**/ const ul = Element('ul', { className: 'todo-list' }, section)
      /**/ const list = List(() => $filteredTodos.value, $list_block, ul)

      // listen events
      input2.listen('change', () => {
        todos = todos.map(todo => ({ ...todo, completed: $remaining.value > 0 }))
      }, todos_up)

      return {
        node: section,
        update: () => banchUpdate(
          [input2, 'checked', $remaining.value === 0],
          list
        )
      }
    }

    const $if_block2 = () => {
      const $1 /* dep: todos */ = Memo(() => $remaining.value === 1 ? 'item' : 'items')

      const footer = Element('footer', { className: 'footer' })
      /***/ const span = Element('span', { className: 'todo-count' }, footer)
      /****/ const strong = Element('strong', span)
      /*****/ const text2 = Text($remaining.value, strong)
      /****/ const s = Space(span)
      /****/ const text3 = Text($1.value, span)
      /****/ const text4 = Text(' left', span)
      /***/ const ul2 = Element('ul', { className: 'filters' }, footer)
      /****/ const list2 = List(['all', 'active', 'completed'], $list_block2, ul2)
      /***/ const if3 = If(() => todos.length > $remaining.value, $if_block3, false, footer)

      return {
        node: footer,
        update: () => banchUpdate(
          $1,
          [text2, () => $remaining.value],
          [text3, () => $1.value],
          list2,
          if3,
        )
      }
    }

    const $if_block3 = () => {
      // create nodes
      const button = Element('button', { className: 'clear-completed' })
      /**/ const text5 = Text('Clear completed', button)

      // listen events 
      button.listen('click', clearCompleted, todos_up)

      return {
        node: button,
        update: nope
      }
    }
    
    const $list_block = todo => ({
      create: () => {
        // create memos
        const $1 = Memo(() => [
          'todo',
          todo.completed && 'completed',
          todo === editedTodo && 'editing'
        ].filter(Boolean).join(' '))

        const $if_block4 = () => {
          const input2 = Element('input', { className: 'edit', type: 'text', value: todo.title })

          input2.listen('input', e => todo.title = e.target.value, todos_up)
          input2.listen('keyup', e => e.key === 'Escape' && cancelEdit(todo), todos_up, editedTodo_up)
          input2.listen('keyup', e => e.key === 'Enter' && doneEdit(todo), todos_up, editedTodo_up)
          input2.listen('blur', () => doneEdit(todo), todos_up, editedTodo_up)

          return {
            node: input2,
            update: () => banchUpdate(
              [input2, 'value', todo.title]
            )
          }
        }

        // create nodes
        const li = Element('li', { className: $1.value })
        /**/ const div = Element('div', { className: 'view' }, li)
        /***/ const input = Element('input', { className: 'toggle', type: 'checkbox', checked: todo.completed }, div)
        /***/ const label = Element('label', div)
        /****/ const text = Text(todo.title, label)
        /***/ const button = Element('button', { className: 'destroy' }, div)
        /**/ const if4 = If(() => todo === editedTodo, $if_block4, false, li)

        // create updates 
        const editedTodo_up = () => banchUpdate(
          $1,
          [li, 'className', () => $1.value],
          if4,
        )

        // listen events
        input.listen('change', e => { todo.completed = e.target.checked }, todos_up)
        label.listen('dblclick', e => {
          editTodo(todo)
          setTimeout(() => {
            const input = e.target.parentNode.nextSibling
            input.focus()
          }, 0)
        }, editedTodo_up)
        button.listen('click', () => removeTodo(todo), todos_up)

        return {
          node: li,
          update: (refresh, value) => {
            if (refresh === true) {
              todo = value
            }
            banchUpdate(
              $1,
              [li, 'className', () => $1.value],
              [input, 'checked', todo.completed],
              [text, todo.title],
              if4
            )
          }
        }
      },
      key: todo.id,
    })

    const $list_block2 = status => ({
      create: () => {
        // create memos
        const $1 = Memo(() => visibility === status ? 'selected' : undefined)

        // create nodes
        const li = Element('li')
        /**/ const a = Element('a', { href: `#${status}`, className: $1.value }, li)
        /***/ const text = Text(status.charAt(0).toUpperCase() + status.slice(1), a)

        // listen events
        li.listen('click', e => {
          e.preventDefault()
          visibility = status
        }, visibility_up)

        return {
          node: li,
          update: () => {
            banchUpdate(
              $1,
              [a, 'className', () => $1.value]
            )
          }
        }
      },
      key: status,
    }) 

    // create nodes 
    const header = Element('header', { className: 'header' })
    /**/ const h1 = Element('h1', header)
    /***/ const text = Text('todos', h1)
    /**/ const input = Element('input', { className: 'new-todo', type: 'text', autofocus: true, autocomplete: 'off', placeholder: 'What needs to be done?', value: newTodo }, header)
    const if1 = If(() => todos.length > 0, $if_block, false)
    const if2 = If(() => todos.length > 0, $if_block2, false)

    // create updates
    const todos_up = () => banchUpdate(
      $filteredTodos,
      $remaining,
      if1.block,
      if2.block,
      $$effect
    )
    const newTodo_up = () => banchUpdate(
      [input, 'value', newTodo]
    )
    const visibility_up = () => banchUpdate(
      $filteredTodos,
      if1.block,
      if2.block,
    )

    // listen events
    input.listen('input', e => newTodo = e.target.value, newTodo_up)
    input.listen('keyup', e => e.key === 'Enter' && addTodo(), newTodo_up, todos_up)

    // listen effects
    this.onMount($$effect)

    // register nodes
    this.register(header, if1, if2)
  }
}
