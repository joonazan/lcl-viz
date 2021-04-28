mod no_ids;
mod utils;
use utils::*;
use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Necklace {
    link: ComponentLink<Self>,
    container: NodeRef,
    section: usize,
}

impl Necklace {
    fn set_css_var(&mut self, name: &str, value: &str) {
        self.container
            .cast::<HtmlElement>()
            .unwrap()
            .style()
            .set_property(name, value)
            .unwrap();
    }
}

const COLOR1: &str = "#9be54d";
const COLOR2: &str = "#383cc0";

pub enum Msg {
    ChangeColor(&'static str, String),
    SectionDone(usize),
}
use Msg::*;

impl Component for Necklace {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            container: NodeRef::default(),
            section: 0,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.set_css_var("--color1", COLOR1);
            self.set_css_var("--color2", COLOR2);
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ChangeColor(color, value) => {
                self.set_css_var(color, &value);
                false
            }
            SectionDone(n) => {
                self.section = self.section.max(n + 1);
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let sections: Vec<Html> =
            vec![html! {<no_ids::NoIds finished=self.link.callback(|_| SectionDone(0))/>}];
        html! {
            <div id="necklace" ref=self.container.clone()>
                <h2>{"Pick two colors"}</h2>
                <input type="color" value=COLOR1 oninput=self.link.callback(|input: InputData| ChangeColor("--color1", input.value))/>
                <input type="color" value=COLOR2 oninput=self.link.callback(|input: InputData| ChangeColor("--color2", input.value))/>
                <h2>{"Proper coloring"}</h2>
                <p>{"Your job here is to color beads so that the chain doesn't have two identical colors next to each other. Below is an example of a proper coloring."}</p>
                <Chain colors=vec![Some(false), Some(true), Some(false), Some(true), Some(false)] />
                {for sections.into_iter().take(self.section + 1)}
            {if self.section != 0 {
                html!{<>
                      <h2>{"Ids"}</h2>
                      <p>{"It turns out the beads have serial numbers. Maybe we can use that to our advantage!"}</p>
                      <Chain colors=vec![None, None, None, None], numbers=Some(vec![34, 76, 2, 51])/>
                </>}
            } else {
                html!{}
            }}
            </div>
        }
    }
}
