mod color;
mod css_writer;
mod extractor;
mod formatter;
mod image;
mod palette;
mod report;
mod sample;
mod scanner;

use css_writer::CssWriter;
use extractor::Extractor;
use formatter::css;
use report::show;
use scanner::Scanner;

fn main() {

    let image =

        Scanner::scan();

    let palette =

        Extractor::extract(&image);

    show(

        &image,

        &palette

    );

    let css =

        css(&palette);

    CssWriter::save(&css);

}
