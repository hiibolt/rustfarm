use crate::{Result, Context, DEBUG};

use std::{collections::HashMap, path::Path};

use crossterm::event::KeyEvent;
use image::DynamicImage;
use spectrust::locate_image;


pub enum AppEvent {
    KeyPressed(KeyEvent),
    UpdateStatus((String, String)),
    AddDebug(String)
}

#[macro_export]
macro_rules! sleep {
    ($time:expr) => {
        std::thread::sleep(std::time::Duration::from_millis($time));
    };
}

pub fn load_images<'a>(
    paths: &'a[&'a str]
) -> Result<HashMap<&'a str, DynamicImage>> {
    // Create a new hashmap for the game
    let mut hashmap = HashMap::new();

    // Load each image into the hashmap
    for &path in paths {
        let img = image::open(Path::new("assets").join(path))
            .with_context(|| format!("Unable to locate file: `{path}`"))?;
        hashmap.insert(path, img);
    }

    Ok(hashmap)
}
pub fn is_on_screen(
    image:      &DynamicImage,
    confidence: Option<f32>,
    tolerance:  Option<u8>
) -> bool {
    let location_of_image = locate_image(
        image,
        None,
        confidence,
        tolerance,
    );

    if DEBUG {
        println!("Location of image: {:?}", location_of_image);
    }
    
    location_of_image.is_some()
}