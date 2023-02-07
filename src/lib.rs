mod utils;

use chrono::Duration;
use js_sys::{Uint8Array};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, Request, Response, HtmlAudioElement};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn wasm_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub async fn play() {
    console_log!("started");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");
    let performance = window
        .performance()
        .expect("performance should be avaliable");

    const WIDTH: i32 = 96;
    const HEIGHT: i32 = 64;
    const BPF: usize = (WIDTH * HEIGHT) as usize;

    let request = Request::new_with_str("/data/frames.bin").expect("error");
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.expect("error");
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let bytes = JsFuture::from(resp.array_buffer().expect("error")).await.expect("error");
    let array = Uint8Array::new(&bytes);
    let video: Vec<u8> = array.to_vec();
    console_log!("loaded");

    let audio = HtmlAudioElement::new_with_src("/data/badapple.mp3").expect("error loading audio");

    // Manufacture the element we're gonna append
    let grid = document.create_element("div").expect("error");
    grid.set_class_name("grid");

    let mut screen: Vec<Element> = Vec::new();

    for i in 0..BPF {
        let pixel = document.create_element("div").expect("error");
        screen.push(pixel);
        grid.append_child(&screen[i]).expect("error");
    }

    _ = audio.play();
    for times in 0..6000 {

        let start = performance.now();
        let mut i = 0;
        for pixel in &screen {
            let byte = video.get(times*BPF+i).expect("error");
            let color = if byte == &0u8 {"p-white"} else {"p-black"};
            pixel.set_class_name(color);
            i+=1;
        }
        let end = performance.now();

        body.append_child(&grid).expect("error");

        let time_taken = end-start;
        let time = (1000f64/30f64) - (time_taken /*+ if time_taken<=4f64 {4f64} else {0f64}*/);

        _ = utils::sleep(time as i32).await;
        console_log!("taken: {}ms", end-start);
    }
}

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    Ok(())
}
