use super::utils::*;
use wasm_bindgen::JsCast;
use web_sys::{DomRect, Element};
use yew::prelude::*;
use yew::services::{render::RenderTask, RenderService};

pub struct Neighborhoods {
    arrow_svg: NodeRef,
    arrow_source: NodeRef,
    arrow_targets: Vec<NodeRef>,
    arrows: Vec<NodeRef>,
    _rendertask: Option<RenderTask>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Render,
}
use Msg::*;

impl Component for Neighborhoods {
    type Message = Msg;
    type Properties = SectionProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            arrow_svg: NodeRef::default(),
            arrow_source: NodeRef::default(),
            arrow_targets: (0..4).map(|_| NodeRef::default()).collect(),
            arrows: (0..4).map(|_| NodeRef::default()).collect(),
            _rendertask: None,
            link,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Render => {
                self.fix_arrows();
                self._rendertask = Some(RenderService::request_animation_frame(
                    self.link.callback(|_| Render),
                ));
                false
            }
        }
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
    fn view(&self) -> Html {
        let colors = vec![None; 4];

        let arrows = self.arrows.iter().map(|arrow| html!{
            <line ref=arrow.clone() stroke="#666" stroke-width="2" marker-end="url(#arrowhead)" />
        });

        html! { <section>
            <h2>{"Neighborhoods"}</h2>
            <p>{"A node's distance-1 neighborhood in a graph includes the node and all its neighbors. \
                 A distance-2 neighborhood contains the neighbors of those neighbors as well etc."}</p>
            <NeighborhoodExplorer/>
            <h2>{"Ambiguity"}</h2>
            <p>{"A network of computers can be coordinated by having each computer map its neighborhood \
                 and then decide what to do based on that."}</p>
            <div style="display:flex; position:relative">
                 <svg ref=self.arrow_svg.clone() class="svg-overlay">
                    <defs>
                        <marker id="arrowhead" markerWidth="10" markerHeight="7"
                         refX="7" refY="3" orient="auto">
                            <polygon points="0 0, 7 3, 0 6" style="fill:#666"/>
                        </marker>
                    </defs>
                    {for arrows}
                 </svg>
                 <div ref=self.arrow_targets[0].clone()>{neighborhood(&colors, 0, 1, true)}</div>
                 <div ref=self.arrow_targets[3].clone() style="transform: rotate(180deg)">{neighborhood(&colors, 0, 1, true)}</div>
                 <div style="margin: 2em 2em"><Chain ref=self.arrow_source.clone() vertical=true colors=colors.clone()/></div>
                 <div ref=self.arrow_targets[1].clone()>{neighborhood(&colors, 1, 1, true)}</div>
                 <div ref=self.arrow_targets[2].clone() style="transform: rotate(180deg)">{neighborhood(&colors, 1, 1, true)}</div>
            </div>
            <p>{"Above: what each computer sees after mapping its distance-1 neighborhood."}</p>

            <p>{"Suppose you want to color nodes so that two colors alternate. Below is an example of a proper coloring."}</p>
            <Chain colors=vec![Some(false), Some(true), Some(false), Some(true), Some(false)] />
            <p>{"It would be great if the computers could coordinate without talking to \
                 – let's say the whole internet – first. \
                 Somewhat surprisingly, mapping the neighborhood always requires as little time \
                 as any other way of communicating."}</p>

        </section> }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.fix_arrows();
            self._rendertask = Some(RenderService::request_animation_frame(
                self.link.callback(|_| Render),
            ));
        }
    }
}

impl Neighborhoods {
    fn fix_arrows(&self) {
        fn attr(el: &Element, name: &str, value: f64) {
            el.set_attribute(name, &format!("{}", value)).unwrap();
        }

        let svg = self
            .arrow_svg
            .cast::<Element>()
            .unwrap()
            .get_bounding_client_rect();
        let svg_x = svg.x();
        let svg_y = svg.y();

        for i in 0..4 {
            let vb = self
                .arrow_source
                .cast::<Element>()
                .unwrap()
                .query_selector_all(".bead")
                .unwrap()
                .get(i)
                .unwrap()
                .unchecked_into::<Element>()
                .get_bounding_client_rect();
            let b2 = self.arrow_targets[i as usize]
                .cast::<Element>()
                .unwrap()
                .query_selector(".highlight")
                .unwrap()
                .unwrap()
                .get_bounding_client_rect();
            let arrow = self.arrows[i as usize].cast::<Element>().unwrap();

            fn one_end(el: DomRect) -> (f64, f64, f64) {
                let h = el.height();
                let w = el.width();
                let y = el.top() + h / 2.0;
                let x = el.left() + w / 2.0;
                (x, y, (w + h) * 0.3)
            }

            let (x, y, r) = one_end(vb);
            let (x2, y2, r2) = one_end(b2);
            let dx = x2 - x;
            let dy = y2 - y;
            let l = (dx * dx + dy * dy).sqrt();
            let dx = dx / l;
            let dy = dy / l;

            let x = x + r * dx;
            let y = y + r * dy;
            let x2 = x2 - r2 * dx;
            let y2 = y2 - r2 * dy;

            attr(&arrow, "x1", x - svg_x);
            attr(&arrow, "y1", y - svg_y);
            attr(&arrow, "x2", x2 - svg_x);
            attr(&arrow, "y2", y2 - svg_y);
        }
    }
}

pub struct NeighborhoodExplorer {
    distance: usize,
    selected: usize,
    link: ComponentLink<Self>,
}
pub enum NeMsg {
    DistanceSelected(usize),
    NodeClicked(usize),
}
use NeMsg::*;

impl Component for NeighborhoodExplorer {
    type Message = NeMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            distance: 1,
            selected: 1,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            DistanceSelected(x) => self.distance = x,
            NodeClicked(x) => self.selected = x,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let distancedropdown = html! {
            <select onchange=self.link.callback(|e: ChangeData| {
                match e {
                    ChangeData::Select(e) => {
                        let index = e.selected_index();
                        if index == -1 {
                            DistanceSelected(0)
                        } else {
                            DistanceSelected(e.selected_index() as usize)
                        }
                    }
                    _ => unreachable!()
                }
            })>{
                for (0..=3).map(|i| html!{<option selected={i == self.distance}>{i}</option>})
            }</select>
        };

        let chain = vec![None, None, None, None];

        html! { <div class="side-by-side">
            <div>
                <h3>{"Graph"}</h3>
                <Chain colors=chain.clone() nodeclick=self.link.callback(NodeClicked) highlight=Some(self.selected) />
                <p>{"Click on a node to select it."}</p>
            </div>
            <div>
                <h3>{"Distance-"}{distancedropdown}{" Neighborhood"}</h3>
                {neighborhood(&chain, self.selected, self.distance, false)}
            </div>
        </div>}
    }
}

fn neighborhood(chain: &[Option<bool>], selected: usize, distance: usize, vert: bool) -> Html {
    let open_start = selected > distance;
    let start = if open_start { selected - distance } else { 0 };

    let open_end = selected + distance + 1 < chain.len();
    let end = if open_end {
        selected + 1 + distance
    } else {
        chain.len()
    };
    html! {<Chain vertical=vert colors=chain[start..end].to_owned() open_start=open_start open_end=open_end highlight=Some(selected - start) />}
}
