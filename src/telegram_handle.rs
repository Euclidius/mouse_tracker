use teloxide::prelude::*;
use teloxide::types;

use std::env;
use std::fs;

use crate::image_handle::save_image_to_file;

pub async fn send_screenshots(screenshots: Vec<opencv::prelude::Mat>) {

    let bot = Bot::from_env();
    let chat_id = env::var("CHAT_ID").unwrap();
    let recipient = types::Recipient::from(chat_id);

    for photo in screenshots {
        save_image_to_file(photo);
        let photo = types::InputFile::file("tmp.jpg");

        bot.send_photo(recipient.clone(), photo)
            .send()
            .await 
            .expect("Failed to send request");
    }

    let _ = fs::remove_file("tmp.jpg");
}

