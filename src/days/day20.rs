use crate::common::{iAoc, AocError, AocResult, IntoAocResult};
use itertools::Itertools;
use std::str::FromStr;

type Point = (usize, usize);

const SQUARE: [(isize, isize); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

/// A single image.
///
/// Originally, this was represented as a HashMap of set points. However, there are
/// so many set points compared to unset that we lose a lot of efficiency over just
/// a vector of booleans representing each point.
///
/// Now, an image is represented as a flat vector of booleans. A single pixel indexes
/// to one position in the vector, and its boolean value represents if it is lit or
/// not. If the image is not inverted, a true value represents a lit pixel. If the
/// image is inverted, a true value represents an dark pixel.
struct Image {
    pixels: Vec<bool>,
    /// Height of the image.
    height: usize,
    /// Width of the image.
    width: usize,
    // Is the image inverted?
    inverted: bool,
}

impl Image {
    pub fn new(height: usize, width: usize, inverted: bool) -> Self {
        Image {
            pixels: vec![false; width * height],
            height,
            width,
            inverted,
        }
    }

    pub fn is_inverted(&self) -> bool {
        self.inverted
    }

    pub fn pixels(&self) -> impl Iterator<Item = Point> {
        (0..self.width).cartesian_product(0..self.height)
    }

    /// Maps a two-dimensional point to a flat index.
    fn get_index(&self, (x, y): Point) -> usize {
        // Assure x does not overflow to the next row.
        if x >= self.width {
            return usize::MAX;
        }

        let (offset, overflow) = y.overflowing_mul(self.width);
        if overflow {
            return usize::MAX;
        }
        let (index, overflow) = offset.overflowing_add(x);
        if overflow {
            return usize::MAX;
        }
        index
    }

    pub fn set(&mut self, pixel: Point) {
        let index = self.get_index(pixel);
        if let Some(b) = self.pixels.get_mut(index) {
            *b = true;
        }
    }

    pub fn is_lit(&self, pixel: Point) -> bool {
        self.pixels
            .get(self.get_index(pixel))
            .and_then(|&b| Some(b != self.inverted))
            .unwrap_or(self.inverted)
    }

    pub fn lit_pixels(&self) -> usize {
        if self.inverted {
            usize::MAX
        } else {
            self.pixels.iter().filter(|&b| *b).count()
        }
    }
}

struct ImageEnhancementAlgorithm {
    // 64 * 8 = 512
    bits: [u64; 8],
}

impl ImageEnhancementAlgorithm {
    pub fn new() -> Self {
        ImageEnhancementAlgorithm { bits: [0; 8] }
    }

    pub fn get(&self, bit: usize) -> bool {
        (self.bits[bit >> 6] & (1 << (bit & ((1 << 6) - 1)))) != 0
    }

    pub fn set(&mut self, bit: usize) {
        self.bits[bit >> 6] |= 1 << (bit & ((1 << 6) - 1));
    }

    pub fn enhance_once(&self, image: Image) -> Image {
        // Enhanced image extends one unit in all four directions.
        let mut new_image = Image::new(
            image.height + 2,
            image.width + 2,
            if image.is_inverted() {
                self.get(0b111111111)
            } else {
                self.get(0)
            },
        );

        // Check all pixels in the expanded image.
        for center in new_image.pixels() {
            // A pixel in the expanded image is (-1, -1) off from the same pixel
            // in the original image.
            //
            // So when appliyng the transformation, also subtract an additional unit.
            let algorithm_index = SQUARE
                .iter()
                .enumerate()
                .filter_map(|(i, &(dx, dy))| {
                    let pixel = (
                        center.0.overflowing_add((dx - 1) as usize).0,
                        center.1.overflowing_add((dy - 1) as usize).0,
                    );
                    if image.is_lit(pixel) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .fold(0usize, |acc, bit| acc | (1 << (8 - bit)));
            if self.get(algorithm_index) != new_image.is_inverted() {
                new_image.set(center);
            }
        }

        new_image
    }

    pub fn enhance(&self, mut image: Image, times: usize) -> Image {
        if times == 0 {
            return image;
        }

        for _ in 0..times {
            image = self.enhance_once(image);
        }

        image
    }
}

struct ImageEnhancement {
    algorithm: ImageEnhancementAlgorithm,
    image: Image,
}

impl FromStr for ImageEnhancement {
    type Err = AocError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let mut algorithm = ImageEnhancementAlgorithm::new();
        for (bit, ch) in lines.next().into_aoc_result()?.chars().enumerate() {
            if ch == '#' {
                algorithm.set(bit);
            }
        }

        lines.next();

        let height = lines.clone().count();
        let width = lines.clone().next().into_aoc_result_msg("no rows")?.len();
        let mut image = Image::new(height, width, false);

        for (y, line) in lines.enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    image.set((x, y));
                }
            }
        }

        Ok(ImageEnhancement { algorithm, image })
    }
}

pub fn solve_a(input: &str) -> AocResult<iAoc> {
    let ImageEnhancement { algorithm, image } = ImageEnhancement::from_str(input)?;
    let enhanced_image = algorithm.enhance(image, 2);
    Ok(enhanced_image.lit_pixels() as iAoc)
}

pub fn solve_b(input: &str) -> AocResult<iAoc> {
    let ImageEnhancement { algorithm, image } = ImageEnhancement::from_str(input)?;
    let enhanced_image = algorithm.enhance(image, 50);
    Ok(enhanced_image.lit_pixels() as iAoc)
}
