use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;

    pub CardModal = <View> {
        width: Fill,
        height: Fill,
        flow: Overlay,
        visible: false,

        // 背景遮罩
        <View> {
            width: Fill,
            height: Fill,
            draw_bg: {
                color: #00000080
            }
        }

        // 模态框内容
        <View> {
            width: 400,
            height: 500,
            align: {x: 0.5, y: 0.5},
            flow: Down,
            padding: 20,
            spacing: 15,
            draw_bg: {
                color: #FFFFFFFF
            }

            modal_title = <Label> {
                text: "创建卡片",
                draw_text: {
                    color: #333333FF,
                    text_style: {
                        font_size: 18.0,
                    }
                }
            }

            <Label> {
                text: "标题:",
                draw_text: {
                    color: #666666FF,
                    text_style: {
                        font_size: 14.0,
                    }
                }
            }

            card_title_input = <TextInput> {
                width: Fill,
                height: 35,
                text: "",
            }

            <Label> {
                text: "描述:",
                draw_text: {
                    color: #666666FF,
                    text_style: {
                        font_size: 14.0,
                    }
                }
            }

            card_description_input = <TextInput> {
                width: Fill,
                height: 80,
                text: "",
            }

            <View> {
                width: Fill,
                height: Fit,
                flow: Right,
                spacing: 10,
                align: {x: 1.0},

                cancel_btn = <Button> {
                    width: 80,
                    height: 35,
                    text: "取消",
                }

                save_card_btn = <Button> {
                    width: 80,
                    height: 35,
                    text: "保存",
                    draw_bg: {
                        color: #00CED1FF
                    }
                }
            }
        }
    }
}
