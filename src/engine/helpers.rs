use egui::Vec2;


pub fn fit_aspect(original: [usize; 2], max: Vec2) -> Vec2 {
    let [w, h] = original;
    let aspect = w as f32 / h as f32;

    let max_w = max.x;
    let max_h = max.y;

    let (new_w, new_h) = if max_w / aspect <= max_h {
        (max_w, max_w / aspect)
    } else {
        (max_h * aspect, max_h)
    };

    Vec2::new(new_w, new_h)
}