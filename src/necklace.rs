use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Document, Element, HtmlElement, HtmlInputElement};

pub fn create(document: &Document) -> Rc<HtmlElement> {
    let container = document
        .create_element("div")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    container.set_id("necklace");
    let container = Rc::new(container);

    container
        .append_child(&make_text(document, "h2", "Pick two colors"))
        .unwrap();

    container
        .append_child(&color_picker(
            document,
            container.clone(),
            "--color1",
            "#9be54d",
        ))
        .unwrap();
    container
        .append_child(&color_picker(
            document,
            container.clone(),
            "--color2",
            "#383cc0",
        ))
        .unwrap();

    container
        .append_child(&make_text(document, "h2", "Proper coloring"))
        .unwrap();
    container
        .append_child(&make_text(document, "p",
                                 "Your job here is to color beads so that the chain doesn't have two identical colors next to each other. Below is an example of a proper coloring."))
        .unwrap();

    let (path, vertices) = make_necklace(document, 7, false, false);
    for (i, b) in vertices.iter().enumerate() {
        b.set_attribute("data-color", if i & 1 == 0 { "A" } else { "B" })
            .unwrap();
    }
    container.append_child(&path).unwrap();

    container.append_child(&make_text(document, "p", "But you only get to see a neighborhood of a certain size. And based on that, you have to choose a color.")).unwrap();

    let (path2, vertices2) = make_necklace(document, 3, true, true);
    for (i, b) in vertices2.iter().enumerate() {
        b.set_inner_html(&format!("{}", i));
    }
    container.append_child(&path2).unwrap();

    container
}

fn make_necklace(
    document: &Document,
    n: usize,
    open_start: bool,
    open_end: bool,
) -> (Element, Vec<Element>) {
    let chain = document.create_element("div").unwrap();
    chain.set_class_name("chain");

    let edge = || {
        let edge = document.create_element("div").unwrap();
        edge.set_class_name("edge");
        chain.append_child(&edge).unwrap();
    };

    let bead = || {
        let bead = document.create_element("div").unwrap();
        bead.set_class_name("bead");
        chain.append_child(&bead).unwrap();
        bead
    };

    let beads = if !open_start {
        Some(bead())
    } else {
        None
    };

    let beads = beads.into_iter().chain((0..n - if open_start { 0 } else { 1 })
        .map(|_| {
            edge();
            bead()
        }))
        .collect::<Vec<_>>();

    if open_end {
        edge();
    }

    (chain, beads)
}

fn make_text(document: &Document, tag: &'static str, text: &'static str) -> Element {
    let h = document.create_element(tag).unwrap();
    h.set_inner_html(text);
    h
}

fn color_picker(
    document: &Document,
    container: Rc<HtmlElement>,
    color_name: &'static str,
    init_color: &'static str,
) -> HtmlInputElement {
    let picker = document.create_element("input").unwrap();
    let picker = picker.dyn_into::<HtmlInputElement>().unwrap();
    picker.set_type("color");
    picker.set_value(init_color);
    container
        .style()
        .set_property(color_name, init_color)
        .unwrap();

    let closure = Closure::wrap(Box::new(move |event: web_sys::InputEvent| {
        container
            .style()
            .set_property(
                color_name,
                &event
                    .target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap()
                    .value(),
            )
            .unwrap();
    }) as Box<dyn FnMut(_)>);
    picker.set_oninput(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    picker
}
