use web_sys::HtmlElement;
use yew::prelude::*;

pub struct Necklace {
    link: ComponentLink<Self>,
    container: NodeRef,
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
}
use Msg::*;

impl Component for Necklace {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            container: NodeRef::default(),
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
            ChangeColor(color, value) => self.set_css_var(color, &value),
        }
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div id="necklace" ref=self.container.clone()>
                <h2>{"Pick two colors"}</h2>
                <input type="color" value=COLOR1 oninput=self.link.callback(|input: InputData| ChangeColor("--color1", input.value))/>
                <input type="color" value=COLOR2 oninput=self.link.callback(|input: InputData| ChangeColor("--color2", input.value))/>
                <h2>{"Proper coloring"}</h2>
                <p>{"Your job here is to color beads so that the chain doesn't have two identical colors next to each other. Below is an example of a proper coloring."}</p>
                <Chain colors=vec![Some(false), Some(true), Some(false), Some(true), Some(false)] />
                <NoIds/>
            </div>
        }
    }
}

struct NoIds {
    color: Option<bool>,
    color2: Option<bool>,
    link: ComponentLink<Self>,
}
enum NoIdsMsg {
    FirstPick(bool),
    SecondPick(bool),
    Reset,
}
use NoIdsMsg::*;

impl Component for NoIds {
    type Message = NoIdsMsg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            color: None,
            color2: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FirstPick(c) => self.color = Some(c),
            SecondPick(c) => self.color2 = Some(c),
            Reset => self.color2 = None,
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let chain = html! {<Chain colors=vec![self.color, None, None, None] highlight=Some(0) />};
        html! {
            <>
                <h2>{"Independence"}</h2>
                <p>{"Each bead has to independently choose its color using the same reasoning. Let's look at a length four chain as an example."}</p>
            {chain}
            {if let Some(c) = self.color {
                let chain2 = html!{<Chain colors=vec![None, None, None, self.color2] highlight=Some(3) />};
                if let Some(c2) = self.color2 {
                    html! { <>
                    {if c != c2 {
                        html!{ <>
                           <p>{"But this contradicts with the first choice!."}</p>
                            <div class="rotate">{chain2}</div>
                            </> }
                    } else {
                        html!{ <>
                             <p>{"But now it is impossible to color the beads in between!"}</p>
                              {chain2}
                        </> }
                    }}
                     <button onclick=self.link.callback(|_| Reset)>{"Try again"}</button>
                    </> }

                } else {
                    html!{ <>
                        <p>{"What about this one?"}</p>
                         {chain2}
                        {colorpick(self.link.callback(SecondPick))}
                        </> }
                }
            } else {
                html!{ <>
                     <p>{"What color should the above bead have?"}</p>
                      {colorpick(self.link.callback(FirstPick))}
                      </> }
            }}
            </>
        }
    }
}

fn colorpick(cb: Callback<bool>) -> Html {
    html! { <>
        <button class="colorbutton" data-color="A" onclick=cb.reform(|_|false)/>{" or "}<button class="colorbutton" data-color="B" onclick=cb.reform(|_|true)/>{"?"}
    </> }
}

#[derive(Clone, Properties, PartialEq)]
struct Chain {
    colors: Vec<Option<bool>>,
    #[prop_or_default]
    numbers: Option<Vec<usize>>,
    #[prop_or_default]
    open_start: bool,
    #[prop_or_default]
    open_end: bool,
    #[prop_or_default]
    highlight: Option<usize>,
}

impl Component for Chain {
    type Message = ();
    type Properties = Chain;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        props
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        *self = props;
        true
    }

    fn view(&self) -> Html {
        let beads = self.colors.iter().enumerate().map(|(i, color)| {
            let class = if Some(i) == self.highlight {
                "bead highlight"
            } else {
                "bead"
            };
            if let Some(c) = color {
                html! {<div class=class data-color=if *c {"B"} else {"A"}></div>}
            } else {
                html! {<div class=class></div>}
            }
        });

        let edge = html! {<div class="edge"></div>};
        let chain = if self.open_start {
            Some(edge.clone())
        } else {
            None
        }
        .into_iter()
        .chain(beads.intersperse(edge.clone()))
        .chain(if self.open_end {
            Some(edge.clone())
        } else {
            None
        });

        html! {
            <div class="chain">
                {for chain}
            </div>
        }
    }
}
