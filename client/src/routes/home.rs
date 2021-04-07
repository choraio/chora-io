use yew::prelude::*;

use crate::utils::markdown;

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        // markdown
        let path = include_str!("../../../content/home/index.md");

        html! {
            <div class="container">
                <div class="banner">
                    <div>
                        <h1>
                            { "chora" }
                        </h1>
                        <p>
                            <i>
                                { "the grass is always greener on the decentralized web" }
                            </i>
                        </p>
                        <p>
                            { "decentralized protocol research" }
                        </p>
                    </div>
                </div>
                <div class="content">
                    <div class="content-item">
                        { markdown::render_markdown(path) }
                        <div class="button-container">
                            <a class="button" href="https://docs.chora.io" alt="docs" target="_blank">
                                { "learn more" }
                            </a>
                        </div>
                    </div>
                </div>
                <div class="banner">
                    <div>
                        <h1>
                            { "connect" }
                        </h1>
                        <p>
                            { "admin [ at ] chora.io" }
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}
