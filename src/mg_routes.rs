

pub use dioxus::prelude::*;
pub use crate::components::main_layout::MainLayout;
pub use crate::views::radical_view::RadicalView;
pub use crate::views::home::Home;

#[derive(Routable, Clone, PartialEq)]
pub(crate) enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home,

    #[route("/radicals")]
    RadicalView,
}

#[cfg(test)]
mod tests {
    use super::Route;

    #[test]
    fn routes_match_their_paths() {
        assert!(matches!("/".parse::<Route>(), Ok(Route::Home)));
        assert!(matches!("/radicals".parse::<Route>(), Ok(Route::RadicalView)));
        assert!("/unknown".parse::<Route>().is_err());
    }
}