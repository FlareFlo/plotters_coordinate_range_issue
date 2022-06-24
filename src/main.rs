use std::ops::Range;
use std::time::Instant;
use plotters::prelude::*;


const WIDTH: u32 = 3840;
const HEIGHT: u32 = 2160;

const TIMESTEP: f64 = 0.1;

// Scaling settings
const FONT_AXIS: u32 = ((WIDTH + HEIGHT) / 2) as u32;

fn main() {
    let start = Instant::now();

    let backend = BitMapBackend::new("out.png", (WIDTH, HEIGHT));

    let root = backend.into_drawing_area();

    let x_dim: Range<f64> = 0.0..440.0;

    // These lines below are the specific problem that i encountered
    // Using the normally generated dimensions below which exceed 1000 the program will never return and leak memory past the 10 gigabyte mark
    let y_dim: Range<f64> = -200.0..801.0;

    // Using this range which specifically sits within the bounds of 1000 works and returns as expected within 10 seconds in a debug build
    // let y_dim: Range<f64> = -200.0..800.0;
    dbg!(&y_dim);

    root.fill(&WHITE).unwrap();
    let root = root.margin(10, 10, 10, 10);


    // Catches weird edge case
    // https://github.com/plotters-rs/plotters/issues/358
    if y_dim.start.abs() + y_dim.end.abs() > 1000.0 {
        // Enable this line to prevent crash but will not produce expected result with bad Y-bounds
        // y_dim = 0.0..1000.0;
    }

    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the size of the label region
        .x_label_area_size(20)
        .y_label_area_size(40)
        .set_label_area_size(LabelAreaPosition::Bottom, FONT_AXIS / 50)
        .set_label_area_size(LabelAreaPosition::Left, FONT_AXIS / 50)
        .caption("caption", ("sans-serif", FONT_AXIS / 20))
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_cartesian_2d(x_dim, y_dim).unwrap(); // Any y range >= 1000 breaks the tool


    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(50)
        .y_labels(50)
        .x_desc("time in s")
        .x_label_style(("sans-serif", FONT_AXIS / 100))
        .y_label_style(("sans-serif", FONT_AXIS / 100))
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.0}", x))
        .x_label_formatter(&|x| format!("{}", x / TIMESTEP.powi(-1) as f64))
        .draw().unwrap();

    eprintln!("took: {:?}", start.elapsed());
}