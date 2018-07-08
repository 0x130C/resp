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
            <section class="no-element-before container",>
                <div class="columns",>
                    <div class="column is-one-quarter",>
                        <h4 class="title is-4 logo",>
                            <a href="/",>
                                <span class="icon",>
                                    <i class="fas fa-cog fa-spin",></i>
                                </span>
                                {" Respx"}
                            </a>
                        </h4>
                        <aside class="menu",>
                            <p class="menu-label",>
                                { "General" }
                            </p>
                            <ul class="menu-list",>
                                <li><a>{"Dashboard"}</a></li>
                                <li><a>{"Customers"}</a></li>
                            </ul>
                        </aside>
                    </div>
                    <div class="column is-three-quarters",>
                        <nav class="breadcrumb", aria-label="breadcrumbs",>
                            <ul>
                                <li><a href="../",>{ "General" }</a></li>
                                <li class="is-active",><a href="#", aria-current="page",>{"Dashboard"}</a></li>
                            </ul>
                        </nav>
                    </div>
                </div>
            </section>
        }
    }
}

