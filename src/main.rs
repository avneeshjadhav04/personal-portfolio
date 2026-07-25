use leptos::mount::mount_to_body;
use personal_portfolio::App;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("init logger");
    mount_to_body(App);
}