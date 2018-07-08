extern crate yew;
extern crate applib;
#[macro_use]
extern crate log;
extern crate web_logger;

use yew::prelude::*;
use applib::RootComponent;


fn main() {
    web_logger::init();
    yew::initialize();
    App::<RootComponent>::new().mount_to_body();
    yew::run_loop();
}