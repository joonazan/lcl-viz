use rand::prelude::*;
use rand::seq::IteratorRandom;
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
mod grid;
use grid::Grid;

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    const N: usize = 25;
    let nodes: Grid<Element, N> = Grid(
        (0..N)
            .map(|_| {
                (0..N)
                    .map(|_| document.create_element("div").unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    let container = document.create_element("div")?;
    container.set_class_name("container");
    for row_nodes in &nodes.0 {
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

    let mut g = Grid::<u8, N>::new();

    for y in 0..N {
        for x in 0..N {
            let color = (x ^ y) & 1;
            g[(x, y)] = color as u8;
            nodes[(x, y)].set_class_name(color_names[color]);
        }
    }

    *closure2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut p;
        let mut p2;
        let mut c;
        let mut c2;
        loop {
            p = (rng.gen_range(0..N), rng.gen_range(0..N));
            p2 = Grid::<u8, N>::neighbors(p).choose(&mut rng).unwrap();

            c = rng.gen_range(0..3);
            c2 = rng.gen_range(0..3);

            let old = g[p];
            let old2 = g[p2];

            g[p] = c;
            g[p2] = c2;

            if Grid::<u8, N>::neighbors(p).all(|n| g[n] != g[p])
                && Grid::<u8, N>::neighbors(p2).all(|n| g[n] != g[p2])
            {
                break;
            }

            g[p] = old;
            g[p2] = old2;
        }

        nodes[p].set_class_name(color_names[c as usize]);
        nodes[p2].set_class_name(color_names[c2 as usize]);

        requestAnimationFrame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    requestAnimationFrame(closure2.borrow().as_ref().unwrap());

    Ok(())
}
