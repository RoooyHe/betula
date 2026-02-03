use makepad_widgets::Cx;

mod card_item;
mod card_list;
mod card_modal;
mod card_tag;
pub mod space;

pub use space::SpaceDto;

pub fn live_design(cx: &mut Cx) {
    card_item::live_design(cx);
    card_list::live_design(cx);
    card_modal::live_design(cx);
    card_tag::live_design(cx);
    space::live_design(cx);
}
