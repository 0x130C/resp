use yew::prelude::*;

use super::services::router::{Route, Request, RouterAgent};
use super::routes::Page;
use super::components::{Counter, Menu, AdminPanel};

pub struct RootComponent {
    router_agent: Box<Bridge<RouterAgent<()>>>,
    page: Page
}

pub enum Msg {
    Nope,
    HandleRoute(Route<()>),
    NavigateTo(Page)
}



impl Component for RootComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(|route| Msg::HandleRoute(route));
        let mut router_agent = RouterAgent::bridge(callback);

        router_agent.send(Request::GetCurrentRoute);

        Self {
            router_agent,
            page: Page::Loading
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::HandleRoute(route) => {
                info!("Routing: {}", route.to_route_string());
                self.page = route.into();
            },
            Msg::NavigateTo(router) => {

            },
            Msg::Nope => {

            }
        };
        true
    }
}


impl Renderable<RootComponent> for RootComponent {
    fn view(&self) -> Html<Self> {
        if self.page.is_full_view() {
            self.page.view()
        } else {
            html! {
                <>
                    <section class="hero is-medium",>
                        <div class="hero-head",>
                            <Menu:/>
                        </div>
                        <div class="hero-body",>
                            <div class="container-fluid",>
                                <h1 class="title",>
                                    { "The new standard in <insert industry here>" }
                                </h1>
                                <h2 class="subtitle",>
                                    {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}
                                </h2>
                            </div>
                        </div>
                   </section>
                   <section class="container",>
                    { self.page.view() }
                   </section>
                </>
            }
        }

    }
}

impl Renderable<RootComponent> for Page {
    fn view(&self) -> Html<RootComponent> {
        match *self {
            Page::Loading => html! {
                <div> { "Loading..." } </div>
            },
            Page::Admin => html! {
                <AdminPanel: />
            },
            _ => html! {
                <Counter: />
            }
        }
    }
}

impl Page {
    pub fn is_full_view(&self) -> bool {
        match *self {
            Page::Admin | Page::NotFound => true,
            _ => false
        }
    }
}