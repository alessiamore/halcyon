use crate::color::Color;
use crate::image::ImageFile;
use crate::palette::Palette;

pub struct Extractor;

impl Extractor {

    pub fn extract(

        _image: &ImageFile

    ) -> Palette {

        Palette {

            colors: vec![

                Color {

                    hex: "#4F6D7A".into()

                },

                Color {

                    hex: "#D9A441".into()

                },

                Color {

                    hex: "#2E3F4F".into()

                },

                Color {

                    hex: "#F2F2F2".into()

                }

            ]

        }

    }

}
