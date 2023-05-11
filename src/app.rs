use crate::components::video_list::LinkList;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <LinkList />
        </div>
    }
}
