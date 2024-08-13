use serde_json::Value;
use yew::prelude::*;

#[derive(PartialEq)]
pub struct ReturnedImage {
    img_id: u8,
    img_path: String,
}

pub trait RequestedImage {
    fn fetch_images(
        images: UseStateHandle<Vec<ReturnedImage>>,
    ) -> UseStateHandle<Vec<ReturnedImage>>;

    fn serialize_json(val: &Value) -> ReturnedImage;
}

impl RequestedImage for ReturnedImage {
    fn serialize_json(val: &Value) -> ReturnedImage {
        ReturnedImage {
            img_id: val["img_id"].as_u64().unwrap() as u8,
            img_path: val["img_path"].to_string(),
        }
    }

    fn fetch_images(
        images: UseStateHandle<Vec<ReturnedImage>>,
    ) -> UseStateHandle<Vec<ReturnedImage>> {
        {
            let images = images.clone();
            wasm_bindgen_futures::spawn_local(async move {
                images.set(
                    reqwest::get("http://localhost:8000/all-images")
                        .await
                        .unwrap()
                        .json::<Vec<Value>>()
                        .await
                        .unwrap()
                        .iter()
                        .map(ReturnedImage::serialize_json)
                        .collect::<Vec<ReturnedImage>>(),
                );
            });
        }
        images
    }
}

#[hook]
fn use_fetch_image_vec() -> UseStateHandle<Vec<ReturnedImage>> {
    #[allow(clippy::redundant_closure)]
    <ReturnedImage as RequestedImage>::fetch_images(use_state_eq(|| Vec::<ReturnedImage>::new()))
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
                                    image_array.img_id,
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
