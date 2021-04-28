use super::utils::*;
use yew::prelude::*;

pub struct NoIds {
    color: Option<bool>,
    color2: Option<bool>,
    link: ComponentLink<Self>,
    finished: Callback<()>,
}
pub enum Msg {
    FirstPick(bool),
    SecondPick(bool),
    Reset,
}
use Msg::*;

impl Component for NoIds {
    type Message = Msg;
    type Properties = SectionProps;

    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            color: None,
            color2: None,
            finished: p.finished,
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

    fn change(&mut self, p: Self::Properties) -> ShouldRender {
        self.finished = p.finished;
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
                     <button onclick=self.finished.reform(|_|())>{"This is impossible"}</button>
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
