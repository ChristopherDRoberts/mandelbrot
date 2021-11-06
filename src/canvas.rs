#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Pixel { red, green, blue }
    }
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!(
            "{red} {green} {blue}",
            red = self.red,
            green = self.green,
            blue = self.blue
        )
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}

#[derive(Debug)]
pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![
                vec![
                    Pixel {
                        red: 0,
                        green: 0,
                        blue: 0
                    };
                    width
                ];
                height
            ],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.pixels[y][x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Pixel) {
        self.pixels[y][x] = pixel;
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = format!(
            "P3\n{width} {height}\n{max_value}\n",
            width = self.width,
            height = self.height,
            max_value = u8::max_value()
        );
        for row in &self.pixels {
            for pixel in row {
                let pixel_str = format!("{}\n", pixel.to_string());
                ppm.push_str(&pixel_str);
            }
        }
        return ppm;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_set_pixel() {
        let mut canvas = Canvas::new(2, 2);
        let expected_pixel = Pixel::new(1, 2, 3);
        canvas.set_pixel(0, 0, expected_pixel);

        let actual_pixel = canvas.get_pixel(0, 0);
        assert_eq!(actual_pixel, expected_pixel);
    }

    #[test]
    fn test_to_ppm() {
        let mut canvas = Canvas::new(2, 1);
        canvas.set_pixel(0, 0, Pixel::new(255, 0, 0));
        canvas.set_pixel(1, 0, Pixel::new(255, 255, 255));
        let actual_string = canvas.to_ppm();
        let expected_string = String::from("P3\n2 1\n255\n255 0 0\n255 255 255\n");

        assert_eq!(actual_string, expected_string);
    }
}
