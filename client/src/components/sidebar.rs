use yew::prelude::*;
// use yew_router::prelude::*;

// use crate::routes::AppRoute;

pub struct Sidebar {
    pub link: ComponentLink<Self>,
    pub menu: bool,
    pub props: Props,
}

pub enum Msg {
    MobileMenu(bool),
}

#[derive(Clone, PartialEq)]
pub struct Link {
    pub name: String,
    pub root: String,
    pub slug: String,
}

#[derive(Clone, PartialEq)]
pub struct Section {
    pub title: String,
    pub links: Vec<Link>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub menu: Vec<Section>,
}

impl Component for Sidebar {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Sidebar { link, menu: false, props }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
       match msg {
           Msg::MobileMenu(show) => {
               if self.menu != show {
                   self.menu = show;
                   true
               } else {
                   false
               }
           }
       }
    }

    fn view(&self) -> Html {
        html! {
            <nav class="sidebar">
                <div class="sidebar-desktop">
                    { for self.props.menu.iter().map(|section| self.view_section(section)) }
                </div>
                <div class="sidebar-mobile">
                    { self.view_mobile() }
                </div>
            </nav>
        }
    }
}

impl Sidebar {
    fn view_link(&self, link: &Link) -> Html {
        if link.slug == "" {
            html! {
                <li>
                    <a href=format!("/{}", link.root) alt=link.name.clone()>
                        { link.name.clone() }
                    </a>
                    // TODO: subroute change not detected by router...
                    // <RouterAnchor<AppRoute> route=AppRoute::Wiki>
                    //     { link.name.clone() }
                    // </RouterAnchor<AppRoute>>
                </li>
            }
        } else {
            html! {
                <li>
                    <a href=format!("/{}/{}", link.root, link.slug.clone()) alt=link.name.clone()>
                        { link.name.clone() }
                    </a>
                    // TODO: subroute change not detected by router...
                    // <RouterAnchor<AppRoute> route=AppRoute::WikiArticle(link.slug.clone())>
                    //     { link.name.clone() }
                    // </RouterAnchor<AppRoute>>
                </li>
            }
        }
    }

    fn view_section(&self, section: &Section) -> Html {
        html! {
            <div class="sidebar-secion">
                <h2>
                    { section.title.clone() }
                </h2>
                <ul>
                    { for section.links.iter().map(|link| self.view_link(link)) }
                </ul>
            </div>
        }
    }

    fn view_mobile(&self) -> Html {
        if self.menu {
            html! {
                <>
                    <button
                        onclick=self.link.callback(|_| Msg::MobileMenu(false))
                    >
                        { "hide menu" }
                    </button>
                    <div class="sidebar-mobile-menu">
                        { for self.props.menu.iter().map(|section| self.view_section(section)) }
                    </div>
                </>
            }
        } else {
            html! {
                <>
                    <button
                        onclick=self.link.callback(|_| Msg::MobileMenu(true))
                    >
                        { "show menu" }
                    </button>
                </>
            }
        }
    }
}