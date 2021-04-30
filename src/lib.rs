use wasm_bindgen::prelude::*;
use yew::events::MouseEvent;
use yew::{html, Component, ComponentLink, Html, InputData, ShouldRender};

pub struct TodoApp {
    state: State,
    link: ComponentLink<Self>,
}
pub struct State {
    todos: Vec<Todo>,
    new_todo_description: String,
}
struct Todo {
    description: String,
}
pub enum Msg {
    AddTodo,
    DeleteTodo(usize),
    EditTodo(usize),
    UpdateInput(String),
    Nope,
}

/**
 * create element
 */
impl TodoApp {
    fn view_todo(&self, (index, todo): (usize, &Todo)) -> Html {
        html! {
            <li class="todo">
                <p>{ &todo.description }</p>
                <button onclick=self.link.callback(move |_| Msg::DeleteTodo(index))>{ "delete" }</button>
            </li>
        }
    }

    fn view_form(&self) -> Html {
        html! {
            <div id="add-todo-form">
                <input
                    oninput=self.link.callback(|e: InputData| Msg::UpdateInput(e.value))
                    placeholder="add new!"
                    value=&self.state.new_todo_description
                />
                <button onclick=self.link.callback(|e: MouseEvent| Msg::AddTodo)>{ "Add" }</button>
            </div>
        }
    }
}

/**
 * main component
 */
impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let todos = Vec::new();
        let state = State {
            todos,
            new_todo_description: "".into(),
        };

        TodoApp { state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo => {
                if !self.state.new_todo_description.is_empty() {
                    let description = self.state.new_todo_description.trim();
                    self.state.todos.push(Todo {
                        description: description.to_string(),
                    });
                    self.state.new_todo_description = "".to_string();
                }
            }
            Msg::EditTodo(index) => {}
            Msg::DeleteTodo(index) => {
                self.state.todos.remove(index);
            }

            Msg::UpdateInput(val) => {
                self.state.new_todo_description = val;
            }
            Msg::Nope => {}
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    /**
     * templates
     */
    fn view(&self) -> Html {
        html! {
            <div id="todo-app">
                <h2>{ "Rust Todo App" }</h2>
                // form
                { self.view_form() }
                // list
                <ul>
                    { for self.state.todos.iter().enumerate().map(|e| self.view_todo(e)) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<TodoApp>();
}
