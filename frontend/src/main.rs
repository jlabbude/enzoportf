use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

struct ReturnedImage {
    img_id: u8,
    img_path: String,
}

pub trait RequestedImage {
    fn handle_images(images: UseStateHandle<Vec<Value>>) -> UseStateHandle<Vec<Value>> {
        let images = images.clone();
        spawn_local(async move {
            reqwest::get("http://localhost:8000/all-images")
                .await
                .unwrap()
                .json::<Vec<Value>>()
                .await
                .unwrap();
        });
        images
    }
}

impl RequestedImage for ReturnedImage {}

#[hook]
fn use_fetch_image_vec() -> UseStateHandle<Vec<Value>> {
    #[allow(clippy::redundant_closure)]
    <ReturnedImage as RequestedImage>::handle_images(use_state_eq(|| Vec::<Value>::new()))
}

#[function_component]
fn App() -> Html {
    let images = use_fetch_image_vec();

    html! {
        <>
            <header> <h1 style="text-align: center; font-size: 50px;" class="font-sans">{"TODO"}</h1></header>
            <div id="images" class="absolute right-0">
                {
                    (*images).iter().map(|image_array| {
                        html! {
                            <img src={format!("/api/images?id={}",
                                    image_array["img_id"],
                                )} class="rounded-s-full "/>
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
