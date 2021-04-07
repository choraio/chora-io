use yew::prelude::*;
use yew_router::prelude::*;

// use crate::components::footer::Footer;
// use crate::components::header::Header;

use crate::routes::{
    AppRoute,
    home::Home,
};

pub struct App {
    current_route: Option<AppRoute>,
    #[allow(unused)]
    router_agent: Box<dyn Bridge<RouteAgent>>,
}

pub enum Msg {
    Route(Route),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router_agent = RouteAgent::bridge(link.callback(Msg::Route));
        let route_service: RouteService = RouteService::new();
        let route = route_service.get_route();
        App {
            current_route: AppRoute::switch(route),
            router_agent,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Route(route) => {
                self.current_route = AppRoute::switch(route);
                true
            }
        }
    }

    fn view(&self) -> Html {
        if let Some(route) = &self.current_route {
            match route {
                // home
                AppRoute::Home => html! {
                    <main class="app-dark">
                        <div class="background-image" />
                        // <Header />
                        <Home />
                        // <Footer />
                    </main>
                },
            }
        } else {
            html! { "route not found" }
        }
    }
}
