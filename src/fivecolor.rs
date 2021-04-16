use crate::grid::*;

use rand::prelude::*;
use std::convert::TryInto;

use rand::seq::{SliceRandom};
use web_sys::Element;

const N: usize = 25;
const COLOR_NAMES: [&'static str; 5] = ["a", "b", "c", "d", "e"];
type G = Grid<u8, N>;

pub struct FiveColor {
    g: G,
    pub container: Element,
    nodes: Grid<Element, N>,
    view_state: G,
}

impl FiveColor {
    pub fn new(rng: &mut StdRng, document: &web_sys::Document) -> Self {
        let mut g = G::new();

        let mut order = G::vertices().collect::<Vec<_>>();
        order.shuffle(rng);
        for &p in &order {
            g[p] = smallest_color(&g, p);
        }

        let nodes = Grid(
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
        for p in G::vertices() {
            nodes[p].set_class_name(COLOR_NAMES[g[p] as usize]);
        }

        let container = document.create_element("div").unwrap();
        container.set_class_name("container");
        for row_nodes in &nodes.0 {
            let row = document.create_element("div").unwrap();
            row.set_class_name("row");
            container.append_child(&row).unwrap();
            for node in row_nodes {
                row.append_child(node).unwrap();
            }
        }

        let mut view_state = g.clone();
        Self {
            g,
            nodes,
            container,
            view_state,
        }
    }

    pub fn evolve(&mut self, rng: &mut StdRng) {
        loop {
            let p = (rng.gen_range(0..N), rng.gen_range(0..N));
            let old = self.g[p];
            self.g[p] = rng.gen_range(0..5);
            if G::neighbors(p).all(|p2| self.g[p2] != self.g[p]) {
                break;
            }
            self.g[p] = old;
        }

        let mut g2 = self.g.clone();
        for i in 0..5 {
            for p in G::vertices() {
                if g2[p] != i {
                    continue;
                }
                let c = smallest_color(&g2, p);
                g2[p] = c;
            }
        }

        for p in G::vertices() {
            let c = g2[p];
            if self.view_state[p] != c {
                self.nodes[p].set_class_name(COLOR_NAMES[c as usize]);
                self.view_state[p] = c;
            }
        }
    }
}

fn smallest_color<const N: usize>(g: &Grid<u8, N>, p: Coord) -> u8 {
    for c in 0.. {
        if G::neighbors(p).all(|x| g[x] != c) {
            return c;
        }
    }
    unreachable!()
}
