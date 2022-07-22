use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreStage::Update` stage,
        // or after the `EguiSystem::BeginFrame` system (which belongs to the `CoreStage::PreUpdate` stage).
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui|{
        use egui::plot::{Line, Plot, Value, Values};

        let true_signal = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            Value::new(x, (x+ui.input().time).sin())
        });

        let sampled_signal = true_signal.to_owned().map(|i|{
            let new_y = (i.y * 5.0).round()/5.0;
            Value::new(i.x, new_y)
        });

        let true_line = Line::new(Values::from_values_iter(true_signal)).width(10.0);
        let sampled_line = Line::new(Values::from_values_iter(sampled_signal)).width(5.0);

        Plot::new("my_plot")
        .view_aspect(2.0)
        .allow_scroll(false)
        .allow_drag(false)
        .set_margin_fraction(egui::Vec2::new(0.0,0.1))
        .show(ui, |plot_ui|{
            plot_ui.line(true_line);
            plot_ui.line(sampled_line);
        });
    });

}