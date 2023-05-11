use std::rc::Rc;vi

use web_sys::MouseEvent;
use yew::{function_component, Html, html, use_node_ref, Callback, Properties, Reducible, use_reducer};

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
    Delete,
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
            },
            Action::Delete => {
                let mut todos = self.todos.clone();
                todos.remove(0);
                Self { todos }.into()
            }
        }
    }
}

#[function_component(LinkList)]
pub fn video_list() -> Html {
    let todos = use_reducer(ListState::default);

    let url_node_ref = use_node_ref();
    let title_node_ref = use_node_ref();

    let on_click_add = {
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
            let elem = ListElem{title: title_input, url: url_input};
            todos.dispatch(Action::Add(elem));
        })
    };

    let on_click_delete = {
        let todos = todos.clone();
        Callback::from(move |_: MouseEvent| {
            todos.dispatch(Action::Delete);
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
                <span class="button-bar">
                    <button class="create_button" type="button" onclick={on_click_add}>
                    {"create"}
                    </button>
                    <button class="create_button delete_button" type="button" onclick={on_click_delete}>
                        {"delete"}
                    </button>
                </span>
            </form>
            <ul class="list_data">
            {
                todos
                    .to_owned()
                    .todos
                    .iter()
                    .map(|l| html! {
                        <ListElement title={l.title.clone()} url={l.url.clone()}/>
                    })
                    .collect::<Html>()
            }
            </ul>
        </div>
    }
}

#[function_component(ListElement)]
pub fn list_element(props: &Props) -> Html {
    html! {
        <li key={props.title.clone()} class="list_elem">
            <span id="title">{format!("Title: {}", props.title)}</span>
            <span>{format!("URL: {}", props.url)}</span>
        </li>
    }
}
