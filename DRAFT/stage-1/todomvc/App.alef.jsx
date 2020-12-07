// declear types with `Todo[]` to make array reactive
let todos = JSON.parse(localStorage.getItem('todomvc') || '[]')
let newTodo = ''
let editedTodo = null
let visibility = 'all'

$: filteredTodos = todos.filter(todo => {
  switch (visibility) {
    case 'all': return true
    case 'active': return !todo.completed
    case 'completed': return todo.completed
  }
})
$: remaining = todos.filter(todo => !todo.completed)

$: () => {
  localStorage.setItem('todomvc', JSON.stringify(todos))
}

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

$t: (
  <header className="header">
    <h1>todos</h1>
    <input
      className="new-todo"
      autofocus
      autocomplete="off"
      placeholder="What needs to be done?"
      value={newTodo}
      onChange={e => newTodo = e.target.value}
      onKeyup={e => e.key === 'Entry' && addTodo}
    />
  </header>
)

$t: if (todos.lenght > 0) {
  <section className="main">
    <input
      id="toggle-all"
      className="toggle-all"
      type="checkbox"
      checked={remaining === 0}
      onChange={() => {
        todos.forEach(todo => {
          todo.completed = value
        })
      }}
    />
    <label for="toggle-all"></label>
    <ul className="todo-list">
      {filteredTodos.map(todo => (
        <li
          className={
            [
              'todo',
              todo.completed && 'completed',
              todo === editedTodo && 'editing'
            ].filter(Boolean).join(' ')
          }
          key={todo.id}
        >
          <div className="view">
            <input
              className="toggle"
              type="checkbox"
              checked={todo.completed}
              onChange={e => { todo.completed = e.target.checked }}
            />
            <label
              onDoubleClick={() => editTodo(todo)}
            >{todo.title}</label>
            <button
              className="destroy"
              onClick={() => removeTodo(todo)}
            />
          </div>
          <input
            className="edit"
            type="text"
            value={todo.title}
            onBlur={() => doneEdit(todo)}
            onKeyUp={e => e.key === 'Enter' && doneEdit(todo)}
            onKeyup={e => e.key === 'Escape' && cancelEdit(todo)}
          />
        </li>
      ))}
    </ul>
  </section>
}

$t: if (todos.length > 0) {
  <footer className="footer">
    <span className="todo-count">
      <strong>{remaining}</strong> {remaining === 1 ? 'item' : 'items'} left
    </span>
    <ul className="filters">
      <li>
        <a
          href="#all"
          className={visibility === 'all' ? 'selected' : undefined}
          onClick={e => {
            e.preventDefault()
            visibility = 'all'
          }}
        >All</a>
      </li>
      <li>
        <a
          href="#active"
          className={visibility === 'active' ? 'selected' : undefined}
          onClick={e => {
            e.preventDefault()
            visibility = 'active'
          }}
        >Active</a>
      </li>
      <li>
        <a
          href="#completed"
          className={visibility === 'completed' ? 'selected' : undefined}
          onClick={e => {
            e.preventDefault()
            visibility = 'completed'
          }}
        >Completed</a>
      </li>
    </ul>
    {todos.length > remaining && (
      <button
        className="clear-completed"
        onClick={clearCompleted}
      >Clear completed</button>
    )}
  </footer>
}
