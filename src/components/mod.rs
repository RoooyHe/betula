use makepad_widgets::Cx;

mod image_grid;
pub(crate) mod image_item;
mod image_row;

pub fn live_design(cx: &mut Cx) {
    image_item::live_design(cx);
    image_row::live_design(cx);
    image_grid::live_design(cx);
}
