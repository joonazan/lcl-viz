use rand::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};
use std::cell::RefCell;
use std::convert::TryInto;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;
mod grid;
use grid::*;

#[wasm_bindgen]
extern "C" {
    fn requestAnimationFrame(closure: &Closure<dyn FnMut()>) -> u32;
}

const N: usize = 25;
type G = Grid<u8, N>;

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

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

    let color_names = ["a", "b", "c", "d", "e"];
    let mut rng = rand::rngs::StdRng::seed_from_u64(0);

    let closure = Rc::new(RefCell::new(None));
    let closure2 = closure.clone();

    let mut g = G::new();

    let vertices = (0..N).flat_map(|x| (0..N).map(move |y| (x, y)));

    {
        let mut order = vertices.clone().collect::<Vec<_>>();
        order.shuffle(&mut rng);
        for &p in &order {
            g[p] = smallest_color(&g, p);
        }
    }

    for p in vertices.clone() {
        nodes[p].set_class_name(color_names[g[p] as usize]);
    }
    let mut view_state = g.clone();

    *closure2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        loop {
            let p = (rng.gen_range(0..N), rng.gen_range(0..N));
            let old = g[p];
            g[p] = rng.gen_range(0..5);
            if G::neighbors(p).all(|p2| g[p2] != g[p]) {
                break;
            }
            g[p] = old;
        }

        let mut g2 = g.clone();
        for i in 0..5 {
            for p in vertices.clone() {
                if g2[p] != i {
                    continue;
                }
                let c = smallest_color(&g2, p);
                g2[p] = c;
            }
        }

        for p in vertices.clone() {
            let c = g2[p];
            if view_state[p] != c {
                nodes[p].set_class_name(color_names[c as usize]);
                view_state[p] = c;
            }
        }

        requestAnimationFrame(closure.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    requestAnimationFrame(closure2.borrow().as_ref().unwrap());

    Ok(())
}

fn smallest_color<const N: usize>(g: &Grid<u8, N>, p: Coord) -> u8 {
    for c in 0.. {
        if G::neighbors(p).all(|x| g[x] != c) {
            return c;
        }
    }
    unreachable!()
}
