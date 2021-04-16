use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
mod fivecolor;
mod grid;
use fivecolor::*;

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();


    let mut rng = rand::rngs::StdRng::seed_from_u64(0);
    let mut fc = FiveColor::new(&mut rng, &document);
    document.body().unwrap().append_child(&fc.container)?;

    let closure = Rc::new(RefCell::new(None));
    let closure2 = closure.clone();
    *closure2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        fc.evolve(&mut rng);
        requestAnimationFrame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    requestAnimationFrame(closure2.borrow().as_ref().unwrap());

    Ok(())
}
