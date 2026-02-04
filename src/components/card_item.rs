use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub CardItem = <RoundedView> {
        width: Fill,
        height: 100,  // 增加高度到100px
        margin: {bottom: 10},  // 增加底部间距
        padding: 15,
        flow: Down,
        spacing: 8,
        draw_bg: {
            color: #FFFFFFFF  // 白色背景
        }

        <RoundedView> {
            width: Fill,
            height: Fit,
            flow: Right,
            align: {y: 0.5},

            card_title_input = <TextInput> {
                width: Fill,
                height: 35,
                text: "卡片标题",
                draw_text: {
                    color: #333333FF,
                    text_style: {
                        font_size: 16.0,  // 增大字体
                    }
                }
                draw_bg: {
                    color: #F8F9FAFF  // 浅灰色背景
                }
            }

            delete_card_btn = <Button> {
                width: 60,
                height: 30,
                text: "删除",
                margin: {left: 8}
                draw_bg: {
                    color: #FF6B6BFF
                }
                draw_text: {
                    color: #FFFFFFFF
                }
            }
        }

        card_tags = <Label> {
            width: Fill,
            height: 20,
            text: "标签",
            draw_text: {
                color: #666666FF,
                text_style: {
                    font_size: 12.0,
                }
            }
        }
    }
}