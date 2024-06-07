use indicatif::{ProgressBar, ProgressStyle};
use raytrace::{
    vec3::Vec3,
    color::write_color,
};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let progress = ProgressBar::new(image_height);
    progress.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap(),
    );

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (f64::from(image_width) - 1.0);
            let g = (j as f64) / (f64::from(image_width) - 1.0);
            let b = 0.0f64;

            let color = Vec3::new(r, g, b);
            write_color(color)
        }
        progress.inc(1);
    }
    progress.finish();
}
