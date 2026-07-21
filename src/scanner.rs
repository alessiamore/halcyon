use crate::image::ImageFile;
use crate::sample;

pub struct Scanner;

impl Scanner {

    pub fn scan() -> ImageFile {

        sample::load()

    }

}
