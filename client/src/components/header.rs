use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

pub struct Header;

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="header">
                <ul>
                    <li>
                        <RouterAnchor<AppRoute> route=AppRoute::Home>
                            { "home" }
                        </RouterAnchor<AppRoute>>
                    </li>
                </ul>
            </nav>
        }
    }
}
