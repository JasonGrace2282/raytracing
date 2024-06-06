use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let progress = ProgressBar::new(image_height);
    progress.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
    );

    // Render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        for i in 0..image_width {
            let r = (i as f64) / (image_width as f64 -1.0);
            let g = (j as f64) / (image_width as f64 - 1.0);
            let b = 0.0;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.99 * b) as u32;

            println!("{ir} {ig} {ib}");
        }
        progress.inc(1);
    }
    progress.finish();
}
