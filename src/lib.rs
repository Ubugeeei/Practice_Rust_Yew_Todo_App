use wasm_bindgen::prelude::*;

use yew::{html, Component, ComponentLink, Html, InputData, NodeRef, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    state: State,
    focus_ref: NodeRef,
}
pub struct State {
    todos: Vec<Todo>,
    value: String,
    edit_value: String,
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

impl Model {
    fn view_todo(&self, (_, todo): (usize, &Todo)) -> Html {
        html! {
            <li>
                { &todo.description }
            </li>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut todos = Vec::new();
        let new_todo = Todo {
            description: String::from("aaaa"),
        };
        todos.push(new_todo);

        let state = State {
            todos,
            value: "".into(),
            edit_value: "".into(),
        };
        let focus_ref = NodeRef::default();
        Model {
            link,
            state,
            focus_ref,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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