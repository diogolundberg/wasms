use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    let canvas = document
        .get_element_by_id("canvas")
        .ok_or("no canvas element")?
        .dyn_into::<HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")?
        .ok_or("no 2d context")?
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Clear background
    context.set_fill_style_str("black");
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // Compute size relative to screen height (20%)
    let height = canvas.height() as f64;
    let width = canvas.width() as f64;
    let size = height.min(width) * 0.2;

    // Center the square
    let x = (width - size) / 2.0;
    let y = (height - size) / 2.0;

    // Draw red square
    context.set_fill_style_str("red");
    context.fill_rect(x, y, size, size);

    Ok(())
}
