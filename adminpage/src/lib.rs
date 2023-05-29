use gloo::console::log;
use stylist::Style;
use yew::prelude::*;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

mod components;
use components::footer::Footer;
// TODO: uncomment import when the API for OAuth is ready.
//use components::msgbox::{MsgBox, MsgBoxVariant};
use components::navbar::Navbar;

mod pages;
use pages::home::Home;
use pages::page_not_found::PageNotFound;

mod api;
mod stores;
use api::{api_get_auth_config, OAuthClientConfigResponse};

#[derive(Clone, Copy, Routable, PartialEq)]
pub enum Route {
    #[at("/adminpage")]
    Home,
    #[not_found]
    #[at("/adminpage/404")]
    NotFound,
}

#[function_component(AppMain)]
pub fn app_main() -> Html {
    let agent = use_auth_agent().expect("Requires OAuth2Context");

    let logout = Callback::from(move |_: MouseEvent| {
        if let Err(err) = agent.start_login() {
            log!(format!("Failed to logout: {err}"));
        }
    });

    html! {
        <BrowserRouter>
            <div class="container-fluid mx-auto px-4 min-width">
                <div class="row">
                    <div class="col"> <Navbar logout={logout}/> </div>
                </div>
                <div class="row">
                    <div class="col"> <Switch<Route> render={switch} /> </div>
                </div>
                <div class="row">
                    <div class="col"> <Footer /> </div>
                </div>
            </div>
        </BrowserRouter>
        // TODO: Replace the above with the following when the API for OAuth is ready.
        // <BrowserRouter>
        //     <div class="container-fluid mx-auto px-4 min-width">
        //         <Failure>
        //             <div class="row">
        //                 <div class="col"> <Navbar /> </div>
        //             </div>
        //         </Failure>
        //         <Authenticated>
        //             <div class="row">
        //                 <div class="col"> <Navbar logout={logout.clone()}/> </div>
        //             </div>
        //             <div class="row">
        //                 <div class="col"> <Switch<Route> render={switch} /> </div>
        //             </div>
        //         </Authenticated>
        //         <NotAuthenticated>
        //             <MsgBox
        //                 title="Failed to authenticate"
        //                 text="Please try again later."
        //                 variant={MsgBoxVariant::Danger}>
        //             </MsgBox>
        //         </NotAuthenticated>
        //         <div class="row">
        //             <div class="col"> <Footer /> </div>
        //         </div>
        //     </div>
        // </BrowserRouter>
    }
}

const STYLESHEET: &str = include_str!("assets/main.css");

#[function_component(App)]
pub fn app() -> Html {
    // Set up the scoped stylesheet (global)
    let stylesheet = Style::new(STYLESHEET).unwrap();

    // The state of the oauth config request
    let _config_state = use_state(|| None as Option<OAuthClientConfigResponse>);
    let config_state = _config_state.clone();
    let config_state_changes = Callback::from(move |config: OAuthClientConfigResponse| {
        _config_state.set(Some(config));
    });

    // Fetch the oauth config if it hasn't been fetched yet
    if config_state.is_none() {
        wasm_bindgen_futures::spawn_local(async move {
            let result = api_get_auth_config().await;
            config_state_changes.emit(result);
        });
    }

    html! {
        <div class={stylesheet}>
            if config_state.is_some() {
                <OAuth2 config={
                    Config {
                        client_id: config_state.as_ref().unwrap().client_id.clone(),
                        auth_url: config_state.as_ref().unwrap().auth_url.clone(),
                        token_url: config_state.as_ref().unwrap().token_url.clone(),
                    }} >
                    <AppMain />
                </OAuth2>
            } else {
                <div class="spinner-wrapper text-primary">
                    <div class="spinner-border" role="status" style="width: 6rem; height: 6rem;">
                        <span class="visually-hidden"> { "Loading..." } </span>
                    </div>
                </div>
            }
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::NotFound => html! { <PageNotFound /> },
    }
}
