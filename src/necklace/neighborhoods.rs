use super::utils::*;
use yew::prelude::*;

pub struct Neighborhoods;

impl Component for Neighborhoods {
    type Message = ();
    type Properties = SectionProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        todo!()
    }
    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        todo!()
    }
    fn view(&self) -> Html {
        html! {<section>
            <h2>{"Neighborhoods"}</h2>
            <p>{"TODO"}</p>
        </section>}
    }
}
