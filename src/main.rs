use mandelbrot::canvas::Canvas;
use mandelbrot::canvas::Pixel;
use mandelbrot::complex::Complex;

fn main() {
    let width = 1920;
    let height = 1920;
    let mut canvas = Canvas::new(width, height);

    let real_lower = -2.0;
    let real_upper = 0.5;
    let im_lower = -1.25;
    let im_upper = 1.25;

    for row in 0..height{
        for col in 0..width{
            let c = map_pixel_to_complex(col, row, width, height, real_lower, real_upper, im_lower, im_upper);
            let result = iterate(c, 1000);
            if result.diverge{
                canvas.set_pixel(col, row, Pixel::new(255,255,255));
            }
        }
    }

    print!("{}",canvas.to_ppm());
}

fn map_pixel_to_complex(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    real_lower: f64,
    real_upper: f64,
    im_lower: f64,
    im_upper: f64,
) -> Complex {
    let dx = (real_upper - real_lower) / (width as f64);
    let dy = (im_upper - im_lower) / (height as f64);
    let real = real_lower + (x as f64) * dx + dx / 2.0;
    let imaginary = im_upper - (y as f64) * dy - dy / 2.0;
    return Complex::new(real, imaginary);
}

pub struct IterationResult {
    diverge: bool,
    iterations: usize,
}

pub fn iterate(c: Complex, max_iterations: usize) -> IterationResult {
    let mut diverge = false;
    let mut iterations = 0;
    let mut z = Complex::new(0.0, 0.0);
    while iterations < max_iterations {
        if z.modulus_squared() > 4.0 {
            diverge = true;
            break;
        }
        z = z * z + c;
        iterations += 1;
    }
    return IterationResult {
        diverge,
        iterations,
    };
}
