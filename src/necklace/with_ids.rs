use super::utils::*;
use yew::prelude::*;

pub struct WithIds {
    link: ComponentLink<Self>,
    finished: Callback<()>,
}

impl Component for WithIds {
    type Message = ();
    type Properties = SectionProps;
    fn create(p: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            finished: p.finished,
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }
    fn change(&mut self, p: Self::Properties) -> ShouldRender {
        self.finished = p.finished;
        false
    }
    fn view(&self) -> Html {
        html! { <section>
            <h2>{"Ids"}</h2>
            <p>{"It turns out the beads have serial numbers. Maybe we can use that to our advantage!"}</p>
            <Chain colors=vec![None, None, None, None], numbers=Some(vec![34, 76, 2, 51])/>
        </section> }
    }
}
