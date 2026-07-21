use crate::palette::Palette;

pub fn css(

    palette: &Palette

) -> String {

    let mut text =

        String::from(":root {\n\n");

    for (i, color) in

        palette.colors.iter().enumerate()

    {

        text.push_str(

            &format!(

                "--color-{}:{};\n",

                i + 1,

                color.hex

            )

        );

    }

    text.push_str("\n}");

    text

}
