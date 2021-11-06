use mandelbrot::canvas::Canvas;
use mandelbrot::canvas::Pixel;

fn main() {
    let mut canvas = Canvas::new(3, 1);
    canvas.set_pixel(0, 0, Pixel::new(255, 0, 0));
    canvas.set_pixel(1, 0, Pixel::new(0, 255, 0));
    canvas.set_pixel(2, 0, Pixel::new(0, 0, 255));

    print!("{}", canvas.to_ppm());
}
