use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct TodoApp {
    state: State,
    link: ComponentLink<Self>,
}
pub struct State {
    todos: Vec<Todo>,
    new_todo_description: String,
    new_todo_id: usize,
    is_showed_edit_input: bool,
    edit_todo_id: usize,
}
struct Todo {
    id: usize,
    description: String,
}
pub enum Msg {
    Nope,
    UpdateInput(String),
    AddTodo,
    DeleteTodo(usize),
    EditTodo(usize),
    ShowEditInput(usize, String),
}

/**
 *  methods
 */
impl State {
    fn add_todo(&mut self) {
        if !self.new_todo_description.is_empty() {
            let description = self.new_todo_description.trim();
            self.todos.push(Todo {
                id: self.new_todo_id,
                description: description.to_string(),
            });
            self.new_todo_id += 1;
            self.reset_form();
        }
    }

    fn delete_todo(&mut self, id: usize) {
        let nwe_list = self.todos.drain(..).filter(|todo| todo.id != id).collect();
        self.todos = nwe_list;
    }

    fn edit_todo(&mut self, id: usize) {
        let mut target_todo_index = 0;
        for (index, todo) in self.todos.iter().enumerate() {
            if todo.id == id {
                target_todo_index = index
            }
        }
        let new_description = self.new_todo_description.to_string();
        self.todos[target_todo_index].description = new_description;
        self.reset_form();
    }

    fn reset_form(&mut self){
        self.is_showed_edit_input = false;
        self.edit_todo_id = 0;
        self.new_todo_description = "".to_string();
    }
}

/**
 *  Model
 *  create element
 */
impl TodoApp {
    fn view_todo(&self, (_, todo): (usize, &Todo)) -> Html {
        let todo_description = todo.description.to_string();
        let todo_id = todo.id;
        html! {
            <li class="todo" style="display: flex;">
                <p style="margin-right: 20px;">{ &todo.description }</p>
                <button onclick=&self.link.callback(move |_| Msg::DeleteTodo(todo_id)) style="margin-right: 20px;">{ "delete" }</button>
                <button onclick=&self.link.callback(move |_| Msg::ShowEditInput(todo_id, String::from(&todo_description)))>{ "edit" }</button>
            </li>
        }
    }

    fn view_add_form(&self) -> Html {
        html! {
            <div id="add-todo-form">
                <input
                    oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
                    placeholder="add new!"
                    value=&self.state.new_todo_description
                />
                <button onclick=self.link.callback(|_| Msg::AddTodo)>{ "Add" }</button>
            </div>
        }
    }

    fn view_edit_form(&self) -> Html {
        let edit_todo_id = self.state.edit_todo_id;
        html! {
            <div id="edit-todo-form">
                <input
                    oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
                    placeholder="add new!"
                    value=&self.state.new_todo_description
                />
                <button onclick=self.link.callback(move |_| Msg::EditTodo(edit_todo_id))>{ "Save" }</button>
            </div>
        }
    }
}

/**
 *  main component
 */
impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    /**
     *  Life cycle
     */
    // onCreate
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let todos = Vec::new();
        let state = State {
            todos,
            new_todo_description: "".into(),
            new_todo_id: 1,
            is_showed_edit_input: false,
            edit_todo_id: 0,
        };
        TodoApp { state, link }
    }

    // onRendered
    fn rendered(&mut self, _first_render: bool) {}

    // onUpdated
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Nope => {}
            Msg::UpdateInput(val) => {
                self.state.new_todo_description = val;
            }
            Msg::AddTodo => {
                self.state.add_todo()
            }
            Msg::DeleteTodo(id) => {
                self.state.delete_todo(id);
            }
            Msg::EditTodo(id) => {
                self.state.edit_todo(id)
            }
            Msg::ShowEditInput(todo_id, description) => {
                self.state.is_showed_edit_input = true;
                self.state.edit_todo_id = todo_id;
                self.state.new_todo_description = description;
            }
        }
        true
    }

    //onChanged
    fn change(&mut self, _props: Self::Properties) -> ShouldRender { false }

    // onDestroyed
    fn destroy(&mut self) {}

    /**
     *  views
     */
    fn view(&self) -> Html {
        html! {
            <div id="todo-app">
                <h2>{ "Rust Todo App" }</h2>

                // input form
                {
                    if !self.state.is_showed_edit_input {
                        self.view_add_form()
                    } else {
                        self.view_edit_form()
                    }
                }

                // list
                <ul>
                    { for self.state.todos.iter().enumerate().map(|e| self.view_todo(e)) }
                </ul>
            </div>
        }
    }
}

/**
 *  start application
 */
#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<TodoApp>();
}
