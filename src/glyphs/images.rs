use std::{io::Cursor, ops::Deref};

use ::image::{io::Reader as ImageReader, GrayImage, ImageFormat};
use image::ImageBuffer;
use once_cell::sync::Lazy;

// TODO: Make more macros to simplify this code

macro_rules! include_dir_pngs {
    ($dir:literal: $($file:literal),* $(,)?) => {
        [$(std::include_bytes!(std::concat![$dir, "/", $file, ".png"])),*]
     };
}

pub static PAIRS_GLYPHS: Lazy<[GrayImage; 2_usize.pow(2)]> = Lazy::new(|| {
    let pngs: [&[u8]; 4] = include_dir_pngs!("pairs": 0, 1, 2, 3);

    pngs.map(into_image)
});

pub static TRIPLETS_GLYPHS: Lazy<[GrayImage; 8]> = Lazy::new(|| {
    let pngs: [&[u8]; 8] = include_dir_pngs!("triplets": 0, 1, 2, 3, 4, 5, 6, 7);

    pngs.map(into_image)
});

pub static QUADRUPLETS_GLYPHS: Lazy<[GrayImage; 16]> = Lazy::new(|| {
    let pngs: [&[u8]; 16] =
        include_dir_pngs!("quadruplets": 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 , 11, 12, 13, 14, 15);

    pngs.map(into_image)
});

fn into_image(png: &[u8]) -> GrayImage {
    let image = ImageReader::with_format(Cursor::new(png), ImageFormat::Png)
        .decode()
        .unwrap();

    image.into_luma8()
}

pub static GLYPH_HEIGHT: Lazy<u32> = Lazy::new(|| {
    let mut heights = PAIRS_GLYPHS
        .iter()
        .chain(TRIPLETS_GLYPHS.deref())
        .chain(QUADRUPLETS_GLYPHS.deref())
        .map(|i| i.dimensions().1);
    let first = heights.next().unwrap();

    assert!(heights.all(|elem| elem == first));

    first
});

macro_rules! tuple_image {
    ($name:ident) => {
        paste::paste! {
            pub static [<$name _GLYPH_WIDTH>]: Lazy<u32> = Lazy::new(|| {
                let mut width = [<$name _GLYPHS>].iter().map(|i| i.dimensions().0);
                let first = width.next().unwrap();

                assert!(width.all(|elem| elem == first));

                first
            });

            pub fn [<generate_random_ $name:lower _image>](amount: u8) -> (u64, GrayImage) {
                let input = super::[<gen_rand_ $name:lower>](amount);

                let glyphs = super::[<make_ $name:lower _iter>](amount, input);

                let image = create_image(glyphs, &*[<$name _GLYPHS>], *[<$name _GLYPH_WIDTH>]);

                (input, image)
            }

            pub fn [<create_ $name:lower _image>](input: u64) -> GrayImage {
                let amount = (u64::BITS - input.leading_zeros()).div_ceil(super::$name as u32) as u8;

                let glyphs = super::[<make_ $name:lower _iter>](amount, input);

                create_image(glyphs, &*[<$name _GLYPHS>], *[<$name _GLYPH_WIDTH>])
            }
        }
    };
}

tuple_image!(PAIRS);
tuple_image!(TRIPLETS);
tuple_image!(QUADRUPLETS);

pub fn create_image(
    glyphs_iter: impl Iterator<Item = u64>,
    glyphs: &[GrayImage],
    glyph_width: u32,
) -> GrayImage {
    let mut glyphs = glyphs_iter
        .map(|x| {
            let glyph = &glyphs[x as usize];
            glyph.rows()
        })
        .collect::<Vec<_>>();

    let mut buffer = ImageBuffer::new(glyph_width * glyphs.len() as u32, *GLYPH_HEIGHT);

    for mut row in buffer.rows_mut() {
        for glyph_row in glyphs.iter_mut().map(|x| x.next().unwrap()) {
            for (p, p2) in glyph_row.zip(row.by_ref()) {
                *p2 = *p;
            }
        }
    }
    buffer
}
