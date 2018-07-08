use std::time::Duration;
use stdweb::web::Date;
use yew::prelude::*;
use yew::services::{ConsoleService, IntervalService, Task};

pub struct Counter {
    console: ConsoleService,
    value: i64,
    job: Box<Task>
}

pub enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
    Tick
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {

        let callback = link.send_back(|_| Msg::Tick);
        let mut interval = IntervalService::new();
        let handle = interval.spawn(Duration::from_millis(1000), callback);

        Counter {
            console: ConsoleService::new(),
            value: 0,
            job: Box::new(handle)
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use self::Msg::*;
        let mut willupdate = true;
        match msg {
            Increment => {
                self.value = self.value + 1;
                self.console.log("plus one");
            }
            Decrement => {
                self.value = self.value - 1;
                self.console.log("minus one");
            }
            Bulk(list) => for msg in list {
                self.update(msg);
                self.console.log("Bulk action");
            },
            Tick => {
                willupdate = true;
            }
        }
        willupdate
    }
}

impl Renderable<Counter> for Counter {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <nav class="menu",>
                    <button class="button is-rounded", onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button class="button is-rounded", onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button class="button is-rounded", onclick=|_| Msg::Bulk(vec![Msg::Increment, Msg::Increment]),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Date::new().to_string() }</p>
            </div>
        }
    }
}