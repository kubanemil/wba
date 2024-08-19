use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing backend and save the result as an image
    let root_area = BitMapBackend::new("AMM_graph.png", (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("AMM constant curve", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

    chart.configure_mesh().draw()?;

    let range = 0..1_000_000;
    let precision = 1000.0;
    let function = |x| (x, (1.0 / x));

    chart
        .draw_series(LineSeries::new(
            range.map(|x| x as f32 / precision).map(function),
            RED,
        ))?
        .label("k = 1000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    chart
        .configure_series_labels()
        .background_style(WHITE.mix(0.8))
        .border_style(BLACK)
        .draw()?;

    Ok(())
}
