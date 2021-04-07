pub mod home;

use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to="/"]
    Home
}