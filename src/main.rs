fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3\n{image_width} {image_height}\n255");

    for x in 0..image_height {
        for y in 0..image_width {
            let r: f32 = x as f32 / (image_height as f32 - 1.);
            let g: f32 = y as f32 / (image_height as f32 - 1.);
            let b: f32 = 0.;

            let ir = (r * 255.999).floor() as u32;
            let ig = (g * 255.999).floor() as u32;
            let ib = (b * 255.999).floor() as u32;

            println!("{ir} {ig} {ib}");
        }
    }
}
