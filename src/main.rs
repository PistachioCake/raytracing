use std::io::{BufWriter, Write};

use raytracing::{write_color, Color};

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");

    let stdout = std::io::stdout().lock();
    let mut stdout = BufWriter::new(stdout);

    let mut stderr = std::io::stderr().lock();
    for x in 0..image_height {
        write!(stderr, "\rScanlines remaining: {:3}", image_height - x).unwrap();
        stderr.flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));

        for y in 0..image_width {
            let r: f32 = x as f32 / (image_height as f32 - 1.);
            let g: f32 = y as f32 / (image_height as f32 - 1.);
            let b: f32 = 0.;

            let color = Color::new(r, g, b);
            write_color(color, &mut stdout);
        }
    }
    write!(stderr, "\r{:30}\n", "Done.").unwrap();
}
