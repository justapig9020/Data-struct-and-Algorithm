mod hash;

use hash::*;
use rand;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "chart/complexity.png";

use std::cmp::{max, min};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = 10;
    let (start, end) = (100, 50000);
    let step = 100;
    let mut search_times = vec![];
    let mut min_time = None;
    let mut max_time = None;
    for times in (start..=end).step_by(step) {
        let mut h = Hash::new(size);
        let search_time = hash_test(&mut h, times);
        search_times.push(search_time);
        min_time = Some(min(min_time.unwrap_or(search_time), search_time));
        max_time = Some(max(max_time.unwrap_or(search_time), search_time));
    }

    let min_time = min_time.unwrap_or(0);
    let max_time = max_time.unwrap_or(1000);

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Size-Compare chart", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(100)
        .y_label_area_size(100)
        //.build_cartesian_2d(100f32..=1000f32, 100f32..=1000f32)?;
        //.build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)?;
        .build_cartesian_2d(start as f32..end as f32, min_time as f32..max_time as f32)?;

    chart.configure_mesh().draw()?;


    chart
        .draw_series(LineSeries::new(
            search_times.iter().enumerate().map(|(i, v)| ((start + i * step) as f32, *v as f32)),
            &RED,
        ))?
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}

fn hash_test(table: &mut Hash, times: usize) -> usize {
    let mut case = Vec::with_capacity(times);
    let mut i = 0;
    while i < times {
        let key = rand::random();
        if let Ok(_) = table.insert(key) {
            i += 1;
            case.push(key);
        }
    }

    let mut search_times = 0;
    for k in case.iter() {
        if let Ok((_key, lv)) = table.search(*k) {
           search_times += lv; 
        }
    }
    search_times
}
