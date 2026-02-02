use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::markdown::*;
    

    // ============ 卡片列表组件 ============
    pub CardList = <RoundedView> {
        width: 260,
        height: Fit,
        margin: 10,
        padding: {top: 10, right: 10, bottom: 10, left: 10}
        draw_bg: {
            color: #F6E88BFF
            border_radius: 12.0
        }
        flow: Down,
        spacing: 8,
        header_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            align: {x: 0.0, y: 0.5}
            title = <H3> {
                text: "今天"
                draw_text: { color: #4A3E4EFF }
            }
            header_spacer = <View> { width: Fill, height: Fit }
            <Button> { text: "->" draw_text: { color: #6A5A72FF } }
            <Button> { text: "..." draw_text: { color: #6A5A72FF } }
        }
        card_item_1 = <CardItem> {}
        card_item_2 = <CardItem> {}
        card_item_3 = <CardItem> {}
        divider = <View> {
            width: Fill,
            height: 1,
            draw_bg: { color: #E6D98AFF }
        }
        add_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            <Button> { text: "+" draw_text: { color: #6A5A72FF } }
            <Button> { text: "添加卡片" draw_text: { color: #6A5A72FF } }
            add_spacer = <View> { width: Fill, height: Fit }
            <Button> { text: "=" draw_text: { color: #6A5A72FF } }
        }
    }
}
