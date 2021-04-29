use super::utils::*;
use yew::prelude::*;

pub struct WithIds {
    color1: Option<bool>,
    color2: Option<bool>,
    color3: Option<bool>,
    link: ComponentLink<Self>,
    finished: Callback<()>,
}

pub enum Msg {
    Color1(bool),
    Color2(bool),
    Color3(bool),
}
use Msg::*;

impl Component for WithIds {
    type Message = Msg;
    type Properties = SectionProps;
    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            color1: None,
            color2: None,
            color3: None,
            link,
            finished: p.finished,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Color1(c) => self.color1 = Some(c),
            Color2(c) => self.color2 = Some(c),
            Color3(c) => self.color3 = Some(c),
        }
        true
    }
    fn change(&mut self, p: Self::Properties) -> ShouldRender {
        self.finished = p.finished;
        false
    }
    fn view(&self) -> Html {
        let ids = vec![34, 76, 2, 51];
        let no_colors = vec![None, None, None, None];
        html! { <section>
            <h2>{"Ids"}</h2>
            <p>{"It turns out the beads have serial numbers. Two beads in a chain can never have the same number. Maybe we can use that to our advantage!"}</p>
            <Chain colors=no_colors.clone(), numbers=Some(ids.clone())/>
            <p>{"Can you figure out a way of choosing colors based on the numbers?"}</p>

            <div style="display:flex" data-vertical="yes">
                 <div>
                    <Chain colors=vec![self.color1, None, None, None] numbers=Some(ids.clone()) highlight=Some(0)/>
                    {colorpick(self.link.callback(|c| Color1(c)))}
                 </div>
                 <div>
                    <Chain colors=vec![self.color2, None, None, None] numbers=Some(ids.clone().into_iter().rev().collect()) highlight=Some(0)/>
                    {colorpick(self.link.callback(|c| Color2(c)))}
                 </div>
                 <div>
                    <Chain colors=vec![self.color3, None, None, None] numbers=Some(ids.clone()) highlight=Some(0)/>
                    {colorpick(self.link.callback(|c| Color3(c)))}
                 </div>
            </div>

            {if let (Some(a), Some(b), Some(c)) = (self.color1, self.color2, self.color3) {
                if a != c {
                    html!{"You chose two different colors in the same situation! Please correct your choices."}
                } else {
                    html!{ <>
                      <h3>{"Result"}</h3>
                       <Chain colors=vec![Some(a), None, None, Some(b)] numbers=ids/>
                       {if a == b {
                           html!{<p>{"This is no good. You can't color the middle now."}</p>}
                       } else {
                           html!{<>
                                 <p>{"Good job!"}</p>
                                 <p>{"I don't know how you chose the colors, but one way of doing it is to use the first color if the bead's number is larger than the opposite one's and vice versa."}</p>
                                 <button onclick=self.finished.reform(|_| ())>{"Next section!"}</button>
                            </>}
                       }}
                    </> }
                }
            } else {
                html!{}
            }}
        </section> }
    }
}
