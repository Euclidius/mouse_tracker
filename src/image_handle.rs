use opencv::{prelude::*, videoio, Result};
use opencv::imgcodecs::imwrite;

pub fn make_screenshot() -> Result<opencv::prelude::Mat, opencv::Error> {
	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}
    
	let mut frame = Mat::default();
	cam.read(&mut frame)?;

    Ok::<opencv::prelude::Mat, opencv::Error>(frame)
}

pub fn save_image_to_file(image: Mat) {
    imwrite("tmp.jpg", &image, &opencv::types::VectorOfi32::new()).unwrap();
}
