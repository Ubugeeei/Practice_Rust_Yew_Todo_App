use wasm_bindgen::prelude::*;

use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};

pub struct Model {
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
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
    Nope,
}

/**
 * create element
 */
impl Model {
    fn view_todo(&self, (_, todo): (usize, &Todo)) -> Html {
        html! {
            <li class="todo">
                { &todo.description }
            </li>
        }
    }

    fn view_form(&self) -> Html {
        html! {
            <div id="add-todo-form">
                <input
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    placeholder="add new!"
                    value=&self.state.new_todo_description
                />
                <button>{ "Add" }</button>
            </div>
        }
    }
}

/**
 * main component
 */
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let todos = Vec::new();
        let state = State {
            todos,
            new_todo_description: "".into(),
        };

        Model { state, link, }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
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
                { self.view_form() }
                <ul>
                    { for self.state.todos.iter().enumerate().map(|e| self.view_todo(e)) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
