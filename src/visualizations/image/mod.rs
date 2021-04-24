pub mod color_palettes;

use ::image::{ImageBuffer, RgbImage};

use self::color_palettes::ColorPalette;

use super::SortingVisualization;

struct CurrentLine {
    data: Vec<CurrentLinePixel>,
    count: u32,
    color_palette: ColorPalette,
}

#[derive(Clone)]
struct CurrentLinePixel {
    red: u32,
    green: u32,
    blue: u32,
}

fn clamp(val: u32) -> u8 {
    if val > 255 {
        255
    } else {
        val as u8
    }
}

impl CurrentLine {
    fn new(width: usize, color_palette: ColorPalette) -> Self {
        Self {
            data: vec![
                CurrentLinePixel {
                    red: 0,
                    green: 0,
                    blue: 0
                };
                width
            ],
            count: 0,
            color_palette,
        }
    }

    fn add(&mut self, data: &Vec<u32>) {
        assert!(data.len() == self.data.len());

        let color_palette = self.color_palette;

        for (pos, [red, green, blue]) in data
            .iter()
            .map(|&val| color_palette(val as f32 / (data.len() - 1) as f32))
            .enumerate()
        {
            self.data[pos].red += red as u32;
            self.data[pos].green += green as u32;
            self.data[pos].blue += blue as u32;
        }

        self.count += 1;
    }

    fn write_to_image(&self, image: &mut RgbImage, row: usize) {
        for (pos, element) in self.data.iter().enumerate() {
            let red = clamp(element.red / self.count);
            let green = clamp(element.green / self.count);
            let blue = clamp(element.blue / self.count);
            image.put_pixel(pos as u32, row as u32, image::Rgb::<u8>([red, green, blue]));
        }
    }
}

pub struct ImageVisualization {
    width: usize,
    height: usize,
    num_frames: usize,
    image: RgbImage,
    current_frame: usize,
    current_line: CurrentLine,
    current_line_pos: usize,
    color_palette: ColorPalette,
}

impl ImageVisualization {
    pub fn new(width: usize, height: usize, num_frames: usize) -> Self {
        Self {
            width,
            height,
            num_frames,
            image: ImageBuffer::new(width as u32, height as u32),
            current_frame: 0,
            current_line: CurrentLine::new(width, color_palettes::grayscale),
            current_line_pos: 0,
            color_palette: color_palettes::grayscale,
        }
    }

    pub fn use_color_palette(mut self, palette: ColorPalette) -> Self {
        self.color_palette = palette;
        self
    }

    pub fn save(&self, path: &str) {
        self.image.save(path).unwrap();
    }
}

impl SortingVisualization for ImageVisualization {
    fn on_start(&mut self) {
        self.current_frame = 0;
        self.current_line = CurrentLine::new(self.width, self.color_palette);
        self.current_line_pos = 0;
        self.image = ImageBuffer::new(self.width as u32, self.height as u32);
    }

    fn on_data_changed(&mut self, data: &Vec<u32>) {
        let desired_line_pos = (((self.current_frame * (self.height - 1)) as f32)
            / ((self.num_frames - 1) as f32))
            .round() as usize;

        self.current_frame += 1;

        if desired_line_pos != self.current_line_pos {
            while self.current_line_pos < desired_line_pos {
                self.current_line
                    .write_to_image(&mut self.image, self.current_line_pos);
                self.current_line_pos += 1;
            }

            self.current_line = CurrentLine::new(self.width, self.color_palette);
            self.current_line_pos = desired_line_pos;
        }

        self.current_line.add(data);
    }

    fn on_finished(&mut self) {
        self.current_line
            .write_to_image(&mut self.image, self.current_line_pos);
    }
}
