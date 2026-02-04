use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub CardTag = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {left: 8, right: 8, top: 4, bottom: 4},
        margin: {right: 5, bottom: 3},
        draw_bg: {
            color: #E0E7FFFF
        }

        tag_label = <Label> {
            text: "标签",
            draw_text: {
                color: #4338CAFF,
                text_style: {
                    font_size: 11.0,
                }
            }
        }
    }
}
