use color_detect::image::{Image, colorspace::rgb::RGB};
use gloo_events::EventListener;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::ImageData;
use yew::prelude::*;

enum Msg {
    FileUpload(Event),
    FileLoaded(Vec<u8>),
    CalculateGreenery,
    Test
}

struct App {
    canvas: NodeRef,
    canvas_ctx: Option<web_sys::CanvasRenderingContext2d>,
    image: Option<Image<RGB>>,
    is_loading: bool,
    percent_green: Option<f32>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            canvas: NodeRef::default(),
            canvas_ctx: None,
            image: None,
            is_loading: false,
            percent_green: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let canvas_width = if self.image.is_some() {
            self.image.as_ref().unwrap().get_width()
        } else {
            0
        };

        let canvas_height = if self.image.is_some() {
            self.image.as_ref().unwrap().get_height()
        } else {
            0
        };

        html! {
            <div>
                <input type="file" onchange={ctx.link().callback(|event: Event| Msg::FileUpload(event))} />
                <canvas
                    ref={self.canvas.clone()}
                    width={canvas_width.to_string()}
                    height={canvas_height.to_string()} />

                if self.image.is_some() {
                    <div>
                        <button onclick={ctx.link().callback(|_| Msg::CalculateGreenery)}>{"Calculate Greenery"}</button>
                        <button onclick={ctx.link().callback(|_| Msg::Test)}>{"Test"}</button>
                    </div>
                    <div>
                        <p>{format!("Green: {:.02}%", self.percent_green.unwrap_or(0.0) * 100.0)}</p>
                    </div>
                }
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FileUpload(event) => {
                self.is_loading = true;
                let file_cb = ctx.link().callback(|value: Vec<u8>| Msg::FileLoaded(value));
                let target = event.target().unwrap();
                let target: web_sys::HtmlInputElement = target.dyn_into().unwrap();
                let file = target.files().unwrap().get(0).unwrap();
                let file_reader = web_sys::FileReader::new().unwrap();
                file_reader.read_as_array_buffer(&file).unwrap();
                let listener = EventListener::new(&file_reader, "load", move |event| {
                    let target = event.target().unwrap();
                    let target: web_sys::FileReader = target.dyn_into().unwrap();
                    let result = target.result().unwrap();
                    let array = Uint8Array::new(&result);

                    file_cb.emit(array.to_vec());
                });
                listener.forget();

                true
            }
            Msg::FileLoaded(data) => {
                self.is_loading = false;
                self.image = Some(Image::new_with_data(data));

                true
            }
            Msg::CalculateGreenery => {
                let image = self.image.as_ref().unwrap();
                self.percent_green = Some(image.get_greenery_percentage());

                true
            }
            Msg::Test => {
                self.image = Some(self.image.as_ref().unwrap().test());

                true
            },
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();

            self.canvas_ctx = Some(ctx);
        }

        if let Some(image) = &self.image {
            let ctx = self.canvas_ctx.as_ref().unwrap();
            let image_data = ImageData::new_with_u8_clamped_array_and_sh(
                wasm_bindgen::Clamped(&image.get_bitmap()),
                image.get_width(),
                image.get_height(),
            )
            .unwrap();

            ctx.clear_rect(
                0.0,
                0.0,
                image.get_width().into(),
                image.get_height().into(),
            );
            ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
