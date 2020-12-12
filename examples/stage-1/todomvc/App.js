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
    const $1 /* dep: todos */ = Memo(() => $remaining.value === 1 ? 'item' : 'items')

    // create effects
    const $$effect /* dep: todos */ = Effect(() => {
      if (save) {
        localStorage.setItem('todomvc', JSON.stringify(todos))
      } else {
        save = true
      }
    })

    // create list blocks
    const $list_block = todo => ({
      create: () => {
        // create memos
        const $1 = Memo(() => [
          'todo',
          todo.completed && 'completed',
          todo === editedTodo && 'editing'
        ].filter(Boolean).join(' '))

        // create nodes
        const li = Element('li', { className: $1.value })
        /**/ const div = Element('div', { className: 'view' }, li)
        /***/ const input = Element('input', { className: 'toggle', type: 'checkbox', checked: todo.completed }, div)
        /***/ const label = Element('label', div)
        /****/ const text = Text(todo.title, label)
        /***/ const button = Element('button', { className: 'destroy' }, div)
        /**/ const block = If(() => todo === editedTodo, li)
        /**/ const input2 = Element('input', { className: 'edit', type: 'text', value: todo.title }, block)

        // create updates
        const up = (refresh, value) => {
          if (refresh === true) {
            todo = value
          }
          banchUpdate(
            $1,
            [li, 'className', () => $1.value],
            [input, 'checked', todo.completed],
            [text, todo.title],
            block,
            [input2, 'value', todo.title]
          )
        }
        const _todos_up = () => {
          up()
          todos_up('list') // to avoid extra list update
        }

        // listen events
        input.listen('change', e => { todo.completed = e.target.checked }, _todos_up)
        label.listen('dblclick', e => {
          editTodo(todo)
          setTimeout(() => {
            const input = e.target.parentNode.nextSibling
            input.focus()
          }, 0)
        }, up, editedTodo_up)
        button.listen('click', () => removeTodo(todo), todos_up)
        input2.listen('input', e => todo.title = e.target.value, _todos_up)
        input2.listen('keyup', e => e.key === 'Escape' && cancelEdit(todo), _todos_up, editedTodo_up)
        input2.listen('keyup', e => e.key === 'Enter' && doneEdit(todo), _todos_up, editedTodo_up)
        input2.listen('blur', () => doneEdit(todo), _todos_up, editedTodo_up)

        return { node: li, update: up }
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

        // create updates
        const up = () => {
          banchUpdate(
            $1,
            [a, 'className', () => $1.value]
          )
        }

        // listen events
        li.listen('click', e => {
          e.preventDefault()
          visibility = status
        }, visibility_up)

        return { node: li, update: up }
      },
      key: status,
    })

    // create nodes 
    const header = Element('header', { className: 'header' })
    /**/ const h1 = Element('h1', header)
    /***/ const text = Text('todos', h1)
    /**/ const input = Element('input', { className: 'new-todo', type: 'text', autofocus: true, autocomplete: 'off', placeholder: 'What needs to be done?', value: newTodo }, header)
    const block = If(() => todos.length > 0)
    /**/ const section = Element('section', { className: 'main' }, block)
    /***/ const input2 = Element('input', { id: 'toggle-all', className: 'toggle-all', type: 'checkbox', checked: $remaining.value === 0 }, section)
    /***/ const label = Element('label', { for: 'toggle-all' }, section)
    /***/ const ul = Element('ul', { className: 'todo-list' }, section)
    /***/ const list = List(() => $filteredTodos.value, $list_block, ul)
    const block2 = If(() => todos.length > 0)
    /**/ const footer = Element('footer', { className: 'footer' }, block2)
    /***/ const span = Element('span', { className: 'todo-count' }, footer)
    /****/ const strong = Element('strong', span)
    /*****/ const text2 = Text($remaining.value, strong)
    /****/ const s = Space(span)
    /****/ const text3 = Text($1.value, span)
    /****/ const text4 = Text(' left', span)
    /***/ const ul2 = Element('ul', { className: 'filters' }, footer)
    /****/ const list2 = List(['all', 'active', 'completed'], $list_block2, ul2)
    /***/ const block3 = If(() => todos.length > $remaining.value, footer)
    /****/ const button = Element('button', { className: 'clear-completed' }, block3)
    /*****/ const text5 = Text('Clear completed', button)

    // create updates
    const todos_up = (arg0) => banchUpdate(
      $filteredTodos,
      $remaining,
      $1,
      [input2, 'checked', () => $remaining.value === 0],
      block,
      block2,
      arg0 !== 'list' && list,
      [text2, () => $remaining.value],
      [text3, () => $1.value],
      block3,
      $$effect
    )
    const newTodo_up = () => banchUpdate(
      [input, 'value', newTodo]
    )
    const editedTodo_up = () => banchUpdate(
      $filteredTodos,
      list,
    )
    const visibility_up = () => banchUpdate(
      $filteredTodos,
      list,
      list2,
    )

    // listen events
    input.listen('input', e => newTodo = e.target.value, newTodo_up)
    input.listen('keyup', e => e.key === 'Enter' && addTodo(), newTodo_up, todos_up)
    input2.listen('change', () => {
      todos = todos.map(todo => ({ ...todo, completed: $remaining.value > 0 }))
    }, todos_up)
    button.listen('click', clearCompleted, todos_up)

    // listen effects
    this.onMount($$effect)

    // register nodes
    this.register(header, block, block2)
  }
}
