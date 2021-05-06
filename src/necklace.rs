mod neighborhoods;
mod utils;
mod with_ids;
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
        fn s<T: Component<Properties = SectionProps>>() -> fn(Callback<()>) -> Html {
            |cb| html! {<T finished=cb/>}
        }
        let section_constructors = [s::<neighborhoods::Neighborhoods>()];
        let nof_sections = section_constructors.len();
        let sections = section_constructors
            .iter()
            .enumerate()
            .map(|(i, f)| f(self.link.callback(move |_| SectionDone(i))));

        html! { <>
            <div id="necklace" ref=self.container.clone()>
                <p>{"Pick two of your favourite colors!"}</p>
                <input type="color" value=COLOR1 oninput=self.link.callback(|input: InputData| ChangeColor("--color1", input.value))/>
                <input type="color" value=COLOR2 oninput=self.link.callback(|input: InputData| ChangeColor("--color2", input.value))/>
                {for sections.take(self.section + 1)}
            </div>
            <div id="footer"></div>
            {for (self.section + 1 .. nof_sections).map(|s| html!{
                <a class="skip" onclick=self.link.callback(move |_| SectionDone(s-1))>{format!("Skip to section {}", s)}</a>
            })}
        </> }
    }
}
