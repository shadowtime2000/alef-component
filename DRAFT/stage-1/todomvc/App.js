import {
  Component,
  Element,
  If,
  List,
  Space,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // strip types 
    let todos /* Array */ = JSON.parse(localStorage.getItem('todomvc') || '[]')
    let newTodo = ''
    let editedTodo = null
    let visibility = 'all'
    var beforeEditCache = ''
    function addTodo() {
      var title = newTodo.trim()
      if (title) {
        todos.push({
          id: Date.now() + '.' + Math.random(),
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
      todos = filters.active(todos);
    }

    // create memos
    const $filteredTodos /* dep: todos, visibility */ = () => todos.filter(todo => {
      switch (visibility) {
        case 'all': return true
        case 'active': return !todo.completed
        case 'completed': return todo.completed
      }
    })
    const $remaining /* dep: todos */ = () => todos.filter(todo => !todo.completed)

    // create effects
    const $$effect /* dep: todos */ = () => {
      localStorage.setItem('todomvc', JSON.stringify(todos))
    }

    // create list blocks
    const $list_block = todo => {
      // create memos
      const $1 = () => ['todo', todo === editedTodo && 'editing'].filter(Boolean).join(' ')

      // create nodes
      const li = Element('li', { className: $1() })
      /**/ const div = Element('div', { className: 'view' }, li)
      /***/ const input = Element('input', { id: 'toggle-all', className: 'toggle-all', type: 'checkbox', checked: todo.completed }, div)
      /***/ const label = Element('label', div)
      /****/ const text = Text(todo.title, label)
      /***/ const button = Element('button', { className: 'destroy' }, div)
      /**/ const input2 = Element('input', { className: 'edit', type: 'text', value: todo.title }, li)

      // create updates
      const up = (refresh, value) => {
        if (refresh) {
          todo = value
        }
        li.update('className', $1())
        input.update('checked', todo.completed)
        text.update(todo.title)
        input2.update('value', todo.title)
      }
      const _editedTodo_up = () => {
        editedTodo_up()
        li.update('className', $1())
      }

      // listen events
      input.listen('change', e => { todo.completed = e.target.checked }, up)
      label.listen('dblclick', () => editTodo(todo), up, _editedTodo_up)
      button.listen('click', () => removeTodo(todo), todos_up)
      input2.listen('input', e => todo.title = e.target.value, up)
      input2.listen('blur', () => doneEdit(todo), up, _editedTodo_up)
      input2.listen('keyup', () => e.key === 'Enter' && doneEdit(todo), up, _editedTodo_up)
      input2.listen('keyup', () => e.key === 'Escape' && cancelEdit(todo), up, _editedTodo_up)

      return {
        node: li,
        key: todo.id,
        up
      }
    }

    // create nodes 
    const header = Element('header', { className: 'header' })
    /**/ const h1 = Element('h1', header)
    /***/ const text = Text('todos', h1)
    /**/ const input = Element('input', { className: 'new-todo', autofocus: true, autocomplete: 'off', placeholder: 'What needs to be done?', value: newTodo }, header)
    const block = If(() => todos.length > 0)
    /**/ const section = Element('section', { className: 'main' }, block)
    /***/ const input2 = Element('input', { id: 'toggle-all', className: 'toggle-all', type: 'checkbox', checked: $remaining() === 0 }, section)
    /***/ const label = Element('label', { for: 'toggle-all' }, section)
    /***/ const list = List(() => $filteredTodos(), $list_block, section)
    const block2 = If(() => todos.length > 0)
    /**/ const footer = Element('footer', { className: 'footer' }, block2)

    // create updates
    const todos_up = () => {
      input2.update('checked', $remaining())
      block.toggle()
      block2.toggle()
      list.update()
    }
    const newTodo_up = () => {
      input.update('value', newTodo)
    }
    const editedTodo_up = () => {

    }
    const visibility_up = () => {

    }

    // listen events
    input.listen('input', e => newTodo = e.target.value, newTodo_up)
    input.listen('keyup', e => e.key === 'Enter' && addTodo(), newTodo_up, todos_up)
    input2.listen('change', () => {
      todos.forEach(todo => {
        todo.completed = value
      })
    }, todos_up)

    this.register(header, block, block2)
  }
}
