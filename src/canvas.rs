#[derive(Debug, Clone, Copy)]
pub struct Pixel{
    red: u8,
    green: u8,
    blue: u8
}

impl Pixel{
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Pixel{red, green, blue}
    }
}

impl PartialEq for Pixel{
    fn eq(&self, other: &Self) -> bool { 
        self.red == other.red &&
        self.green == other.green &&
        self.blue == other.blue
    }
}

#[derive(Debug)]
pub struct Canvas{
    width: usize,
    height: usize,
    pixels: Vec<Vec<Pixel>>
}

impl Canvas{
    pub fn new(width: usize, height: usize) -> Self {
        Canvas{width, height, pixels: vec![vec![Pixel{red:0,green:0,blue:0};height];width]}
    }

    pub fn get_pixel(self, x: usize, y:usize) -> Pixel{
        self.pixels[x][y]
    }

    pub fn set_pixel(&mut self, x:usize, y:usize, pixel: Pixel){
        self.pixels[x][y] = pixel;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_set_pixel() {
        let mut canvas = Canvas::new(2,2);
        let expected_pixel = Pixel::new(1, 2, 3);
        canvas.set_pixel(0, 0, expected_pixel);

        let actual_pixel = canvas.get_pixel(0, 0);
        assert_eq!(actual_pixel, expected_pixel);
    }
}