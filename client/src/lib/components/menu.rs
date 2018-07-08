use yew::prelude::*;

pub struct Menu {
    sections: Vec<MenuSection>,
    onselect: Option<Callback<()>>,
    onitemclicked: Option<Callback<()>>
}

#[derive(PartialEq, Clone)]
pub struct MenuSection {
    title: Option<String>,
    items: Vec<MenuItem>
}

#[derive(PartialEq, Clone)]
pub struct MenuItem {
    name: String,
    icon: Option<String>,
    submenu: Vec<MenuItem>
}

pub enum Msg {
    Selected(MenuSection),
    Active(MenuItem),
    Clicked(MenuItem)
}

#[derive(PartialEq, Clone)]
pub struct Props {
    sections: Vec<MenuSection>,
    onselect: Option<Callback<()>>,
    onitemclicked: Option<Callback<()>>
}

impl Default for Props {
    fn default() -> Self {
        Props {
            sections: vec![],
            onselect: None,
            onitemclicked: None
        }
    }
}

impl Component for Menu {
    type Message = Msg;
    type Properties = Props;

    fn create(props: <Self as Component>::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            sections: props.sections,
            onselect: props.onselect,
            onitemclicked: props.onitemclicked
        }
    }

    fn update(&mut self, msg: <Self as Component>::Message) -> bool {
        use self::Msg::*;
        match msg {
            Selected(section) => {

            },
            Active(item) => {

            },
            Clicked(item) => {

            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.sections = props.sections;
        self.onselect = props.onselect;
        self.onitemclicked = props.onitemclicked;
        true
    }
}

impl Renderable<Menu> for Menu {
    fn view(&self) -> Html<Self> {
        html! {
            <nav class="navbar", role="navigation", aria-label="main navigation",>
              <div class="navbar-brand",>
                <a class="navbar-item", href="https://bulma.io",>
                  <img src="https://bulma.io/images/bulma-logo.png", alt="Bulma: a modern CSS framework based on Flexbox", width="112", height="28",/>
                </a>

                <a role="button", class="navbar-burger", aria-label="menu", aria-expanded="false",>
                  <span aria-hidden="true",></span>
                  <span aria-hidden="true",></span>
                  <span aria-hidden="true",></span>
                </a>
              </div>
              <div class="navbar-menu",>
                { view_nav_start() }
                { view_nav_end() }
              </div>
            </nav>
        }
    }
}


fn view_nav_start() -> Html<Menu> {
    html! {
        <div class="navbar-start",>
          <a class="navbar-item", href="https://bulma.io/",>
            { "Home" }
          </a>
          <div class="navbar-item has-dropdown is-hoverable",>
            <a class="navbar-link", href="/documentation/overview/start/",>
              { "Docs" }
            </a>
            <div class="navbar-dropdown is-boxed",>
              <a class="navbar-item", href="/documentation/overview/start/",>
                { "Overview" }
              </a>
              <a class="navbar-item", href="https://bulma.io/documentation/modifiers/syntax/",>
                { "Modifiers" }
              </a>
              <a class="navbar-item", href="https://bulma.io/documentation/columns/basics/",>
                { "Columns" }
              </a>
              <a class="navbar-item", href="https://bulma.io/documentation/layout/container/",>
                { "Layout" }
              </a>
              <a class="navbar-item", href="https://bulma.io/documentation/form/general/",>
                { "Form" }
              </a>
              <hr class="navbar-divider",/>
              <a class="navbar-item", href="https://bulma.io/documentation/elements/box/",>
                { "Elements" }
              </a>
              <a class="navbar-item is-active", href="https://bulma.io/documentation/components/breadcrumb/",>
                { "Components" }
              </a>
            </div>
          </div>
        </div>
    }
}

fn view_nav_end() -> Html<Menu> {
    html! {
        <div class="navbar-end",>
          <a class="navbar-item", href="/",>
            { "Home" }
          </a>
          <a class="navbar-item", href="/signin",>
            { "Sign in" }
          </a>
          <a class="navbar-item", href="/signup",>
            { "Sign up" }
          </a>
        </div>
    }
}
