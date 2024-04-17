use mouse_tracker::image_handle::make_screenshot;
use mouse_tracker::telegram_handle::send_screenshots;
use mouse_tracker::mouse_handle;
use system_shutdown::shutdown;

use inputbot::*;
use std::thread::sleep;

/*
 * MOUSE_DEFAULT is a position where your mouse will be moved with script activating
 */
const MOUSE_DEFAULT:(f64, f64) = (100.0, 100.0);

/*
 * MOUSE_ERROR defines a square where you can move your mouse after script activating
 */
const MOUSE_ERROR:f64 = 20.0;


static mut STATE:bool = false;

async fn activate() {
    unsafe {
        STATE = true;
    }

    mouse_handle::move_mouse(MOUSE_DEFAULT);

    loop {
        let is_on_place = mouse_handle::check_location(MOUSE_DEFAULT, MOUSE_ERROR);
        if !is_on_place {
            alert().await;
        }
        
        unsafe {
            if !STATE {
                return;
            } 
        }

        sleep(std::time::Duration::new(1, 0));
    }
}

async fn alert() {
    println!("ALERT");
    let mut screenshots = Vec::new();
    for _ in 0..3 {
        let screen = make_screenshot();
        screenshots.push(screen.unwrap());
    }

    send_screenshots(screenshots).await;

    match shutdown() {
        Ok(_) => println!("Bye"),
        Err(error) => eprintln!("Failed to shutdown: {}", error),
    };
}


fn deactivate() {
    unsafe {
        STATE = false;
    }
    println!("deactivated");
}


#[tokio::main]
async fn main() {
    inputbot::KeybdKey::bind_all(|_| {
        if inputbot::KeybdKey::LShiftKey.is_pressed() && inputbot::KeybdKey::LControlKey.is_pressed() && inputbot::KeybdKey::SlashKey.is_pressed() {
            tokio::runtime::Runtime::new().unwrap().block_on(activate());
        }

        if inputbot::KeybdKey::LShiftKey.is_pressed() && inputbot::KeybdKey::LControlKey.is_pressed() && inputbot::KeybdKey::RShiftKey.is_pressed() {
            deactivate();
        }
    });

    handle_input_events();
}
