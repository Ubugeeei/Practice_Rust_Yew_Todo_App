use wasm_bindgen::prelude::*;

use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};

pub struct Model {
    state: State,
}
pub struct State {
    todos: Vec<Todo>,
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
 * create todo element
 */
impl Model {
    fn create_todo_element(&self, (_, todo): (usize, &Todo)) -> Html {
        html! {
            <li>
                { &todo.description }
            </li>
        }
    }
}

/**
 * main component
 */
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let todos = Vec::new();
        let state = State { todos };

        Model { state }
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
                <ul>
                    { for self.state.todos.iter().enumerate().map(|e| self.create_todo_element(e)) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
