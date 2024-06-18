use std::collections::HashMap;

use ::icns::IconType;
use image::DynamicImage;

type ImageSet = HashMap<IconType, DynamicImage>;

pub mod icns;
pub mod ico;
pub mod tray;
pub mod helpers;
pub mod definition;
