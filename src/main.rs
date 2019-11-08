extern crate dbus;
extern crate systray;

use dbus::{BusType,Connection,Message};

fn main() {
    let mut app = systray::Application::new().unwrap();

    app.add_menu_item(&"Disable for 30min".to_string(), |_| {
        disable(Some(1800));
    }).ok();
    app.add_menu_item(&"Disable for 1h".to_string(), |_| {
        disable(Some(3600));
    }).ok();
    app.add_menu_item(&"Disable for 2h".to_string(), |_| {
        disable(Some(2*3600));
    }).ok();
    app.add_menu_item(&"Disable for 4h".to_string(), |_| {
        disable(Some(4*3600));
    }).ok();
    app.add_menu_item(&"Disable for 8h".to_string(), |_| {
        disable(Some(8*3600));
    }).ok();
    app.add_menu_item(&"Disable indefinitely".to_string(), |_| {
        disable(None);
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Enable".to_string(), |_| {
        enable();
    }).ok();
    app.add_menu_item(&"Activate".to_string(), |_| {
        activate();
    }).ok();
    app.add_menu_separator().ok();
    app.add_menu_item(&"Quit".to_string(), |window| {
        window.quit();
    }).ok();

    update_icon_and_tooltip(&mut app);
    app.set_loop_callback(Box::new(|a : &mut systray::Application| {
        update_icon_and_tooltip(a);
    }));

    app.wait_for_message();
}

fn disable(period : Option<u64>) {
    let connection = Connection::get_private(BusType::Session).unwrap();
    let mut m = Message::new_method_call("net.andresovi.xees", "/", "net.andresovi.xees", "Disable")
        .unwrap();
    if let Some(seconds) = period {
        m = m.append1(seconds);
    }
    let _ = connection.send_with_reply_and_block(m, 2000);
}

fn enable() {
    let connection = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call("net.andresovi.xees", "/", "net.andresovi.xees", "Enable")
        .unwrap();
    let _ = connection.send_with_reply_and_block(m, 2000);
}

fn activate() {
    let connection = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call("net.andresovi.xees", "/", "net.andresovi.xees", "Activate")
        .unwrap();
    let _ = connection.send_with_reply_and_block(m, 2000);
}

enum ScreenSaverStatus {
    Running,
    Disabled,
    Stopped,
}

fn is_screensaver_enabled() -> Result<ScreenSaverStatus,()> {
    let connection = Connection::get_private(BusType::Session).unwrap();
    let m = Message::new_method_call("net.andresovi.xees", "/", "net.andresovi.xees", "Status")
        .unwrap();
    match connection.send_with_reply_and_block(m, 2000) {
        Ok(result) => {
            match result.get1() {
                Some("Disabled") => {
                    Ok(ScreenSaverStatus::Disabled)
                }
                Some("Stopped") => {
                    Ok(ScreenSaverStatus::Stopped)
                }
                _ => {
                    Ok(ScreenSaverStatus::Running) // by default assume that screensaver runs
                }
            }
        }
        Err(_) => Err(())
    }
}

fn update_icon_and_tooltip(app : &mut systray::Application) {
    match is_screensaver_enabled() {
        Err(_) => {
            app.set_icon_from_file(&"./resources/eye_scratched.png".to_string()).ok();
        }
        Ok(ScreenSaverStatus::Stopped) => {
            app.set_icon_from_file(&"./resources/eye_scratched.png".to_string()).ok();
        }
        Ok(ScreenSaverStatus::Running) => {
            app.set_icon_from_file(&"./resources/eye_closed.png".to_string()).ok();
        }
        Ok(ScreenSaverStatus::Disabled) => {
            app.set_icon_from_file(&"./resources/eye_open.png".to_string()).ok();
        }
    }
}


