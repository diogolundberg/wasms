use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas: HtmlCanvasElement = document
        .get_element_by_id("canvas")
        .ok_or("no canvas element")?
        .dyn_into()?;

    // Match canvas internal pixel dimensions to CSS size
    let width = canvas.client_width() as u32;
    let height = canvas.client_height() as u32;
    canvas.set_width(width);
    canvas.set_height(height);

    let context = canvas
        .get_context("2d")?
        .ok_or("no 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Clear background
    context.set_fill_style_str("black");
    context.fill_rect(0.0, 0.0, width as f64, height as f64);

    // Absolute 100px square centered
    let size = 100.0;
    let x = (width as f64 - size) / 2.0;
    let y = (height as f64 - size) / 2.0;

    context.set_fill_style_str("red");
    context.fill_rect(x, y, size, size);

    Ok(())
}

fn main() {}
