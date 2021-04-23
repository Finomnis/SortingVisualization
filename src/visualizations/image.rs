use ::image::{ImageBuffer, RgbImage};

use super::SortingVisualization;

struct CurrentLine {
    data: Vec<CurrentLinePixel>,
    count: u32,
}

#[derive(Clone)]
struct CurrentLinePixel {
    red: u32,
    green: u32,
    blue: u32,
}

impl CurrentLine {
    fn new(width: usize) -> Self {
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
        }
    }

    fn add(&mut self, data: &Vec<f32>) {
        assert!(data.len() == self.data.len());

        for (pos, data) in data
            .iter()
            .map(|val| -> palette::rgb::Rgb { palette::Hsv::new(val * 240.0, 1.0, 1.0).into() })
            .enumerate()
        {
            self.data[pos].red += (data.red * 255.0).round() as u32;
            self.data[pos].green += (data.green * 255.0).round() as u32;
            self.data[pos].blue += (data.blue * 255.0).round() as u32;
        }

        self.count += 1;
    }

    fn write_to_image(&self, image: &mut RgbImage, row: usize) {
        let clamp = |val: u32| -> u8 {
            if val > 255 {
                255
            } else {
                val as u8
            }
        };

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
    num_steps: usize,
    image: RgbImage,
    current_step: usize,
    current_line: CurrentLine,
    current_line_pos: usize,
}

impl ImageVisualization {
    pub fn new(width: usize, height: usize, num_steps: usize) -> Self {
        Self {
            width,
            height,
            num_steps,
            image: ImageBuffer::new(width as u32, height as u32),
            current_step: 0,
            current_line: CurrentLine::new(width),
            current_line_pos: 0,
        }
    }

    pub fn save(&self, path: &str) {
        self.image.save(path).unwrap();
    }
}

impl SortingVisualization for ImageVisualization {
    fn on_start(&mut self, data: &Vec<f32>) {
        self.current_step = 0;
        self.current_line = CurrentLine::new(self.width);
        self.current_line_pos = 0;
        self.image = ImageBuffer::new(self.width as u32, self.height as u32);

        self.current_line.add(data);
    }

    fn on_data_changed(&mut self, data: &Vec<f32>) {
        self.current_step += 1;

        let desired_line_pos = (((self.current_step * (self.height - 1)) as f32)
            / (self.num_steps as f32))
            .round() as usize;

        if desired_line_pos != self.current_line_pos {
            self.current_line
                .write_to_image(&mut self.image, self.current_line_pos);

            self.current_line = CurrentLine::new(self.width);
            self.current_line_pos = desired_line_pos;
        }

        self.current_line.add(data);
    }

    fn on_finished(&mut self) {
        self.current_line
            .write_to_image(&mut self.image, self.current_line_pos);
    }
}
