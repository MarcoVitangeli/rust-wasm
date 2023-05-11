use std::rc::Rc;

use web_sys::MouseEvent;
use yew::{function_component, Html, html, use_node_ref, Callback, Properties, use_state, Reducible};

#[derive(Properties,PartialEq)]
pub struct Props {
    title: String,
    url: String,
}

#[derive(Clone)]
pub struct ListElem {
    title: String,
    url: String,
}

enum Action {
    Add(ListElem),
}

struct ListState {
    todos: Vec<ListElem>,
}

impl Default for ListState {
    fn default() -> Self {
        ListState { todos: Vec::<ListElem>::new() }
    }
}

impl Reducible for ListState {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(new_todo) => {
                let mut todos = self.todos.clone();
                todos.push(new_todo);
                Self { todos }.into()
            }
        }
    }
}

#[function_component(Create)]
pub fn video_list() -> Html {
    let todos = use_state(|| Vec::<ListElem>::new());

    let url_node_ref = use_node_ref();
    let title_node_ref = use_node_ref();

    let on_click = {
        let url_node_ref = url_node_ref.clone();
        let title_node_ref = title_node_ref.clone();
        let todos = todos.clone();

        Callback::from(move |_: MouseEvent| {
            let url_input = url_node_ref.cast::<web_sys::HtmlInputElement>()
            .expect("url input ref do not attached to dom element")
            .value();
            
            let title_input = title_node_ref.cast::<web_sys::HtmlInputElement>()
            .expect("title input ref do not attached to dom element")
            .value();

            // como console.log pero desde rust
            // web_sys::console::log_2(&url_input.into(), &title_input.into());
            let new_todos = vec![ListElem{title: title_input, url: url_input}];
            todos.set(new_todos);

        })
    };
    html! {
        <div class="list_container">
            <form class="create_container">
                <input type="text" placeholder="title of the video" required={true} ref={title_node_ref}/>
                <input 
                    ref={url_node_ref}
                    type="url" 
                    name="url" 
                    id="create_url"
                    placeholder="https://youtube.com/watch?v=asd2123ni2123"
                    pattern="https://.*"
                    required={true} />
                <button class="create_button" type="button" onclick={on_click.clone()}>
                    {"create"}
                </button>
            </form>
            <ul class="list_data">
                {todos
                    .iter()
                    .map(|l| {
                        html! {
                            <ListElement title={l.title.as_str()} url={l.url.as_str()}/>
                        }
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}

#[function_component(ListElement)]
pub fn list_element(props: &Props) -> Html {
    html! {
        <li class="list_elem">
            <span id="title">{format!("Title: {}", props.title)}</span>
            <span>{format!("URL: {}", props.url)}</span>
        </li>
    }
}
