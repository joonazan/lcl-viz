use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct SectionProps {
    pub finished: Callback<()>,
}

pub fn colorpick(cb: Callback<bool>) -> Html {
    html! { <>
        <button class="colorbutton" data-color="A" onclick=cb.reform(|_|false)/>{" or "}<button class="colorbutton" data-color="B" onclick=cb.reform(|_|true)/>{"?"}
    </> }
}

#[derive(Clone, Properties, PartialEq)]
pub struct Chain {
    pub colors: Vec<Option<bool>>,
    #[prop_or_default]
    pub numbers: Option<Vec<usize>>,
    #[prop_or_default]
    pub open_start: bool,
    #[prop_or_default]
    pub open_end: bool,
    #[prop_or_default]
    pub highlight: Option<usize>,
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

            let content = if let Some(nums) = &self.numbers {
                format!("{}", nums[i])
            } else {
                "".into()
            };

            if let Some(c) = color {
                html! {<div class=class data-color=if *c {"B"} else {"A"}>{content}</div>}
            } else {
                html! {<div class=class>{content}</div>}
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
