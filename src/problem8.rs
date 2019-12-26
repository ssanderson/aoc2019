/*!

--- Day 8: Space Image Format ---

The Elves' spirits are lifted when they realize you have an opportunity to
reboot one of their Mars rovers, and so they are curious if you would spend a
brief sojourn on Mars. You land your ship near the rover.

When you reach the rover, you discover that it's already in the process of
rebooting! It's just waiting for someone to enter a BIOS password. The Elf
responsible for the rover takes a picture of the password (your puzzle input)
and sends it to you via the Digital Sending Network.

Unfortunately, images sent via the Digital Sending Network aren't encoded with
any normal encoding; instead, they're encoded in a special Space Image
Format. None of the Elves seem to remember why this is the case. They send you
the instructions to decode it.

Images are sent as a series of digits that each represent the color of a single
pixel. The digits fill each row of the image left-to-right, then move downward
to the next row, filling rows top-to-bottom until every pixel of the image is
filled.

Each image actually consists of a series of identically-sized layers that are
filled in this way. So, the first digit corresponds to the top-left pixel of
the first layer, the second digit corresponds to the pixel to the right of that
on the same layer, and so on until the last digit, which corresponds to the
bottom-right pixel of the last layer.

For example, given an image 3 pixels wide and 2 pixels tall, the image data
123456789012 corresponds to the following image layers:

Layer 1: 123
         456

Layer 2: 789
         012
The image you received is 25 pixels wide and 6 pixels tall.

To make sure the image wasn't corrupted during transmission, the Elves would
like you to find the layer that contains the fewest 0 digits. On that layer,
what is the number of 1 digits multiplied by the number of 2 digits?

--- Part Two ---

Now you're ready to decode the image. The image is rendered by stacking the
layers and aligning the pixels with the same positions in each layer. The
digits indicate the color of the corresponding pixel: 0 is black, 1 is white,
and 2 is transparent.

The layers are rendered with the first layer in front and the last layer in
back. So, if a given position has a transparent pixel in the first and second
layers, a black pixel in the third layer, and a white pixel in the fourth
layer, the final image would have a black pixel at that position.

For example, given an image 2 pixels wide and 2 pixels tall, the image data
0222112222120000 corresponds to the following image layers:

Layer 1: 02
         22

Layer 2: 11
         22

Layer 3: 22
         12

Layer 4: 00
         00
Then, the full image can be found by determining the top visible pixel in each position:

The top-left pixel is black because the top layer is 0.
The top-right pixel is white because the top layer is 2 (transparent), but the second layer is 1.
The bottom-left pixel is white because the top two layers are 2, but the third layer is 1.
The bottom-right pixel is black because the only visible pixel in that position is 0 (from layer 4).
So, the final image looks like this:

01
10
What message is produced after decoding your image?

 */

use crate::utils::{bail, read_problem_file, ProblemResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pixel {
    Black,
    White,
    Transparent,
}

impl Pixel {
    pub fn from_byte(byte: u8) -> ProblemResult<Pixel> {
        match byte {
            b'0' => Ok(Pixel::Black),
            b'1' => Ok(Pixel::White),
            b'2' => Ok(Pixel::Transparent),
            _ => bail(format!("Invalid pixel value: {}", byte)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Pixel::Black => '\u{2588}',
            Pixel::White => '\u{2591}',
            Pixel::Transparent => ' ',
        }
    }
}

struct Layer<'a> {
    pixels: &'a [Pixel],
    width: usize,
    height: usize,
}

impl Layer<'_> {
    pub fn count(&self, byte: Pixel) -> usize {
        self.pixels.iter().cloned().filter(|&b| b == byte).count()
    }

    pub fn at(&self, col: usize, row: usize) -> Pixel {
        if col >= self.width {
            panic!("Column {} is out of bounds for width {}", col, self.width);
        }

        if row >= self.height {
            panic!("Row {} is out of bounds for width {}", row, self.height);
        }

        return self.pixels[row * self.width + col];
    }
}

struct Layers<'a> {
    ix: usize,
    max_ix: usize,
    image: &'a Image,
}

impl<'a> Iterator for Layers<'a> {
    type Item = Layer<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix == self.max_ix {
            None
        } else {
            let out = self.image.layer(self.ix);
            self.ix += 1;
            Some(out)
        }
    }
}

struct Image {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(pixels: Vec<Pixel>, width: usize, height: usize) -> Image {
        let total_pixels = pixels.len();

        if total_pixels % (width * height) != 0 {
            panic!(
                "Pixel count ({}) doesn't match dimensions ({}, {})",
                pixels.len(),
                width,
                height,
            );
        }

        Image {
            pixels,
            width,
            height,
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::new();

        for j in 0..self.height {
            for i in 0..self.width {
                out.push(self.effective_pixel(i, j).to_char());
            }
            out.push('\n');
        }
        out
    }

    pub fn effective_pixel(&self, i: usize, j: usize) -> Pixel {
        for layer in self.layers() {
            let pixel = layer.at(i, j);
            if pixel != Pixel::Transparent {
                return pixel;
            }
        }
        return Pixel::Transparent;
    }

    pub fn layers(&self) -> impl Iterator<Item = Layer> {
        Layers {
            ix: 0,
            max_ix: self.num_layers(),
            image: self,
        }
    }

    pub fn layer(&self, i: usize) -> Layer {
        let nlayers = self.num_layers();
        let pixels_per_layer = self.pixels.len() / nlayers;

        if i >= nlayers {
            panic!("Index out of bounds: {} >= {}", i, nlayers);
        }
        let pixels = &self.pixels[i * pixels_per_layer..(i + 1) * pixels_per_layer];

        Layer {
            pixels: pixels,
            width: self.width,
            height: self.height,
        }
    }

    fn num_layers(&self) -> usize {
        self.pixels.len() / self.layer_size()
    }

    fn layer_size(&self) -> usize {
        (self.width * self.height)
    }
}

fn read_input(width: usize, height: usize) -> ProblemResult<Image> {
    let file_content: String = read_problem_file(8)?;
    let pixels: ProblemResult<Vec<Pixel>> =
        file_content.trim().bytes().map(Pixel::from_byte).collect();

    Ok(Image::new(pixels?, width, height))
}

pub fn run() -> ProblemResult<()> {
    let image = read_input(25, 6)?;
    let most_zeros = image
        .layers()
        .min_by_key(|layer| layer.count(Pixel::Black))
        .unwrap();

    println!(
        "{}",
        most_zeros.count(Pixel::White) * most_zeros.count(Pixel::Transparent)
    );

    println!("{}", image.render());

    Ok(())
}
