use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[hook]
fn use_fetch_image_vec() -> UseStateHandle<Vec<Value>> {
    #[allow(clippy::redundant_closure)]
    // Idk why Clippy keeps bugging about this, when the function can only take closures as args
    let images = use_state_eq(|| Vec::<Value>::new());
    {
        let images = images.clone();
        spawn_local(async move {
            let fetched_images = reqwest::get("http://localhost:8000/all-images")
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
        <>
            <header> <h1 style="text-align: center; font-size: 50px;">{"TODO"}</h1></header>
            <div id="imagaes">
                {
                    (*images).iter().map(|image_array| {
                        html! {
                            <img src={format!("/api/images?id={}",
                                    image_array["img_id"],
                                )}class="rounded-full"/>
                        }
                    }).collect::<Html>()
                }
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
