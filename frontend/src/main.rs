use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
           <img src="http://localhost:8000/images?id=1539493375"/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
