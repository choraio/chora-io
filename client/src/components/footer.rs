use yew::prelude::*;

pub struct Footer;

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Footer {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <footer class="footer">
                <a
                    href="https://github.com/choraio"
                    rel="noopener noreferrer"
                    target="_blank"
                >
                    { "github" }
                </a>
            </footer>
        }
    }
}