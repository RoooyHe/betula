use makepad_widgets::*;

pub mod space;
pub mod card_list;
pub mod card_item;
pub mod card_tag;
pub mod card_modal;

pub use space::*;
pub use card_list::*;

// 导出组件的 live_design
pub fn live_design(cx: &mut Cx) {
    space::live_design(cx);
    card_list::live_design(cx);
    card_item::live_design(cx);
    card_tag::live_design(cx);
    card_modal::live_design(cx);
}