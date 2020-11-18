use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::*;
use web_sys::{HtmlCanvasElement, Document, WebGlRenderingContext as GL};

fn wait_until_canvas_is_rendered(document: Document, canvas_id: &str) -> Result<HtmlCanvasElement, Element> {
    let canvas: Option<Element> = document.get_element_by_id(canvas_id);
    match canvas {
        Some(elem) => {
            elem.dyn_into::<web_sys::HtmlCanvasElement>()
        }
        None => {
            return wait_until_canvas_is_rendered(document, canvas_id);
        }
    }
}

pub fn initialize_webgl_context(canvas: &HtmlCanvasElement) -> Result<WebGlRenderingContext, JsValue> {
    let gl: WebGlRenderingContext = canvas.get_context("webgl")?.unwrap().dyn_into()?;
    gl.clear_color(0.0, 0.0, 0.0, 1.0); //RGBA
    gl.clear(GL::COLOR_BUFFER_BIT);

    Ok(gl)
}

pub fn get_canvas(canvas_id: &str) -> HtmlCanvasElement {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas: HtmlCanvasElement = wait_until_canvas_is_rendered(document, canvas_id).unwrap();
    canvas
}
