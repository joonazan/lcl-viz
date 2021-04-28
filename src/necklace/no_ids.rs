use super::utils::*;
use yew::prelude::*;

pub struct NoIds {
    colors: Vec<Option<bool>>,
    link: ComponentLink<Self>,
    finished: Callback<()>,
}
pub enum Msg {
    PickColor(usize, bool),
    Reset,
}
use Msg::*;

impl Component for NoIds {
    type Message = Msg;
    type Properties = SectionProps;

    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            colors: vec![None; 4],
            finished: p.finished,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PickColor(i, c) => self.colors[i] = Some(c),
            Reset => self.colors = vec![None; 4],
        }
        true
    }

    fn change(&mut self, p: Self::Properties) -> ShouldRender {
        self.finished = p.finished;
        false
    }

    fn view(&self) -> Html {
        fn differ(a: Option<bool>, b: Option<bool>) -> bool {
            a.and_then(|a| b.map(|b| a != b)).unwrap_or(false)
        }
        let contradiction1 = differ(self.colors[0], self.colors[3]);
        let contradiction2 = differ(self.colors[1], self.colors[2]);
        let improper_coloring = self
            .colors
            .iter()
            .zip(self.colors.iter().skip(1))
            .any(|(&a, &b)| a.and_then(|a| b.map(|b| a == b)).unwrap_or(false));
        let error = contradiction1 || contradiction2 || improper_coloring;

        let choice_chain = |i: usize| {
            let cols = self
                .colors
                .iter()
                .enumerate()
                .map(|(j, c)| if i == j { *c } else { None })
                .collect::<Vec<_>>();
            let not_chosen = self.colors[i] == None;
            let message = [
                "What color should the below bead have?",
                "Now, what should this one's color be?",
                "One left!",
                "What about this one?",
            ][i];
            let chain = html! {<Chain colors=cols highlight=i />};

            html! {<>
                  {if not_chosen {html!{<p>{message}</p>}} else {html!{}}}
                  {if contradiction1 && i == 3 || contradiction2 && i == 2 {
                      html!{<>
                            <p>{"But this contradicts the previous choice!"}</p>
                            <div class="rotate">{chain}</div>
                      </>}
                  } else {
                      chain
                  }}
                  {if not_chosen {colorpick(self.link.callback(move |c| PickColor(i, c)))} else {html!{}}}
            </> }
        };
        let nof_colored = self.colors.iter().filter(|&&x| x != None).count();
        let order = [0, 3, 1, 2];

        html! { <section>
            <h2>{"Independence"}</h2>
            <p>{"Each bead has to independently choose its color using the same reasoning. Let's look at a length four chain as an example."}</p>
            <Chain colors=self.colors.clone() />

            {if nof_colored != 0 {
                html!{<>
                    <h3>{"Past choices"}</h3>
                    {for order.iter().take(nof_colored).cloned().map(choice_chain)}
                </>}
            } else {
                html!{}
            }}

            {if improper_coloring {
                html!{<p>{"But now two beads with the same color are next to each other!"}</p>}
            } else {
                html!{}
            }}
            {if error {
                html!{ <>
                    <button onclick=self.link.callback(|_| Reset)>{"Try again"}</button>
                    <button onclick=self.finished.reform(|_|())>{"This is impossible"}</button>
                </> }
            } else {
                {choice_chain(order[nof_colored])}
            }}
        </section> }
    }
}
