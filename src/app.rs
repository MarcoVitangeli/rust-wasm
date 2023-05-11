use yew::prelude::*;
use crate::components::video_list::LinkList;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <LinkList />
        </div>
    }
}
