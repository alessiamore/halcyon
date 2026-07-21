use crate::image::ImageFile;
use crate::palette::Palette;

pub fn show(

    image: &ImageFile,

    palette: &Palette

) {

    println!("Loading image...\n");

    println!("{}", image.name);

    println!();

    println!("Detected colors\n");

    for color in &palette.colors {

        println!("{}", color.hex);

    }

    println!();

    println!("Generated CSS\n");

}
