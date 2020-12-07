import {
  Component,
  Element,
  If,
  Space,
  Text
} from '../../lib/helper.js'

export default class App extends Component {
  constructor() {
    super()

    // initiate state
    let todos /* Array<Todo> */ = JSON.parse(localStorage.getItem('todomvc') || '[]')
    let newTodo = ''
    let editedTodo = null
    let visibility = 'all'

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

    // create nodes 
    const header = Element('header', { class: 'header' }, section)
    /**/ const h1 = Element('h1', header)
    /***/ const text = Text('todos', h1)
    /**/ const input = Element('input', { class: 'new-todo', autofocus: true, autocomplete: 'off', placeholder: 'What needs to be done?', value: newTodo }, header)
    const block = If(() => todos.lenght > 0, section)
    /**/ const section = Element('section', { class: 'main' }, block)
    const block2 = If(() => todos.lenght > 0, section)
    /**/ const footer = Element('footer', { class: 'footer' }, block2)

    // create actions
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
    var beforeEditCache = ''
    function editTodo(todo) {
      beforeEditCache = todo.title
      editedTodo = todo
    }
    function cancelEdit(todo) {
      editedTodo = null
      todo.title = beforeEditCache
    }
    function doneEdit(todo) {
      if (editedTodo) {
        editedTodo = null
        todo.title = todo.title.trim()
        if (!todo.title) {
          removeTodo(todo)
        }
      }
    }
    function removeTodo(todo) {
      todos.splice(todos.indexOf(todo), 1)
    }
    function clearCompleted() {
      todos = filters.active(todos);
    }

    // create updates
    const todos_up = () => {

    }
    const newTodo_up = () => {

    }
    const editedTodo_up = () => {

    }
    const visibility_up = () => {

    }

    this.register(header, block, block2)
  }
}
