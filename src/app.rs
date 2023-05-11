use yew::prelude::*;
use crate::components::video_list::Create;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <Create />
        </div>
    }
}
