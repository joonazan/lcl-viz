use rand::prelude::*;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    const N: usize = 25;
    let nodes: [[Element; N]; N] = (0..N)
        .map(|_| {
            (0..N)
                .map(|_| document.create_element("div").unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let container = document.create_element("div")?;
    container.set_class_name("container");
    for row_nodes in &nodes {
        let row = document.create_element("div")?;
        row.set_class_name("row");
        container.append_child(&row)?;
        for node in row_nodes {
            row.append_child(node)?;
        }
    }
    document.body().unwrap().append_child(&container)?;

    let color_names = ["a", "b", "c"];
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);

    let closure = Rc::new(RefCell::new(None));
    let closure2 = closure.clone();

    for y in 0..N {
        for x in 0..N {
            let color = (x ^ y) & 1;
            nodes[y][x].set_class_name(color_names[color]);
        }
    }

    *closure2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        nodes[rng.gen_range(0..N)][rng.gen_range(0..N)]
            .set_class_name(color_names[rng.gen_range(0..3)]);

        requestAnimationFrame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    requestAnimationFrame(closure2.borrow().as_ref().unwrap());

    Ok(())
}
