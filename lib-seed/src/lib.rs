
use seed::{prelude::*, *};

mod stepping;
mod requests;
mod automatic;
mod views;
mod runs;

#[wasm_bindgen(start)]
pub fn start() {

    let root_element = document()
        .get_element_by_id("app")
        .expect("`section` as a root element");

    match root_element.class_name().as_str() {
        "stepping" => stepping::start(root_element),
        "automatic" => automatic::start(root_element),
        "runspanel" => runs::start(root_element),
        s => log!("wrong app!:", s)
    }

}