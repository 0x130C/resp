use yew::prelude::*;

pub struct AdminPanel;

impl Component for AdminPanel {
    type Message = ();
    type Properties = ();

    fn create(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        AdminPanel{}
    }

    fn update(&mut self, msg: <Self as Component>::Message) -> bool {
        true
    }
}

impl Renderable<AdminPanel> for AdminPanel {
    fn view(&self) -> Html<AdminPanel> {
        html! {
        <div id="app",>
            <section class="hero app-header animated slideInDown",>
                <div class="hero-head",>
                    <div class="nav app-navbar",>
                        <div class="nav-left",>
                            <a class="nav-item is-hidden-tablet",>
                                <i aria-hidden="true", class="fa fa-bars",></i>
                            </a>
                        </div>
                    </div>
                </div>
            </section>
            <section class="app-main",>
                <div class="container is-marginless app-content",>
                    <nav class="breadcrumb", aria-label="breadcrumbs",>
                        <ul>
                            <li><a href="../",>{ "General" }</a></li>
                            <li class="is-active",><a href="#", aria-current="page",>{"Dashboard"}</a></li>
                        </ul>
                    </nav>
                </div>
            </section>
            <aside class="menu app-menu animated slideInLeft",>
                <h4 class="title is-4 logo",>
                    <a href="/",>
                        <span class="icon",>
                            <i class="fas fa-cog fa-spin",></i>
                        </span>
                        {" Respx"}
                    </a>
                </h4>
                <p class="menu-label",>
                    { "General" }
                </p>
                <ul class="menu-list",>
                    <li><a>{"Dashboard"}</a></li>
                    <li><a>{"Customers"}</a></li>
                </ul>
            </aside>
            </div>
        }
    }
}

