use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[hook]
fn use_fetch_image_vec() -> UseStateHandle<Vec<Value>> {
    #[allow(clippy::redundant_closure)] // Idk why Clippy keeps bugging about this, when the function can only take closures as args
    let images = use_state_eq(|| Vec::<Value>::new());
    {
        let images = images.clone();
        spawn_local(async move {
            let fetched_images = reqwest::get("http://127.0.0.1:8000/all-images")
                .await
                .unwrap()
                .json::<Vec<Value>>()
                .await
                .unwrap();
            images.set(fetched_images);
        });
    }
    images
}

#[function_component]
fn App() -> Html {
    let images = use_fetch_image_vec();

    html! {
        <div id="images">
            {
                (*images).iter().map(|image_array| {
                    html! {
                        <img src={format!("http://127.0.0.1:8000/images?id={}",
                                  image_array["img_id"],
                             )}
                             width="128"
                             height="128" />
                    }
                }).collect::<Html>()
            }
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
