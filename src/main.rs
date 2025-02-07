#![windows_subsystem = "windows"]
// https://docs.slint.dev/latest/docs/slint/guide/platforms/desktop/
use std::cmp::Ordering;
use rand::Rng;
use slint::SharedString;

slint::include_modules!();
fn main()  -> Result<(), slint::PlatformError>{
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    let secret:i32 = rand::rng().random_range(0..100);

    ui.on_check(move |str: SharedString| {
        let ui = ui_handle.unwrap();
        if !str.trim().is_empty() {
            let num:i32 = str.trim().parse::<i32>().unwrap();
            let out = match num.cmp(&secret) {
                Ordering::Less => { "Too short"}
                Ordering::Equal => { "You Win ! "}
                Ordering::Greater => { "Too big"}
            };
            ui.set_result(out.into());
        }
    });

    ui.run()
}