use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::markdown::*;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                window: {
                    title: "Betula"
                },
                caption_bar = {
                    visible: true,
                    margin: {
                        left: -500
                    },
                    caption_label = {
                        label = {
                            text: "Betula"
                        }
                    },
                },
                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Overlay,
                    draw_bg: {
                        color: #F8F6FF
                    }
                    header_view = <View> {
                        flow: Right
                        <Button> { text: "View" }
                        <Button> { text: "Betula" }
                        <TextInput> {}
                        <Button> { text: "创建" }
                        <Button> { text: "意见" }
                        <Button> { text: "通知" }
                        <Button> { text: "信息" }
                        <Button> { text: "账户" }
                    }


                        panel_view = <View> {
                            <Button> { text: "我的面板" }
                            <Button> { text: "视图切换" }
                            <Button> { text: "账户" }
                            <Button> { text: "自动化" }
                            <Button> { text: "筛选" }
                            <Button> { text: "收藏" }
                            <Button> { text: "变更可见性" }
                            <Button> { text: "分享" }
                            <Button> { text: "。。。" }
                        }

                    content_view = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Right,
                        spacing: 10,
                        margin: 10,


                        <RoundedView> {
                            width: Fill,
                            height: Fit,
                            margin: 10,
                            padding: 5,
                            draw_bg: {
                                color: #FFFFFF
                                border_radius: 8.0
                            }
                            flow: Down,
                        title = <H3> {
                            text: "Trello新手指南"
                            draw_text: { color: #4A3E4E }

                            }

                            card_button = <Button> {
                                text: "TestButton"
                                width: Fill,

                            }
                            <Button> { text: "TestButton" draw_text: { color: #4A3E4E } }
                            <Button> { text: "TestButton" draw_text: { color: #4A3E4E } }
                        }
                        <RoundedView> {
                            width: Fill, height: Fit,
                            flow: Right,
                            draw_bg: { color: #FFE9F2, border_radius: 8.0 }
                            <P> { text: "- Shader-based: what does that mean for how things work." draw_text: { color: #5A4660 } }
                        }

                        <RoundedView> {
                            width: Fill, height: Fit,
                            flow: Right,
                            draw_bg: { color: #E9FFF6, border_radius: 8.0 }
                            <P> { text: "- Shader-based: what does that mean for how things work." draw_text: { color: #3F5C55 } }
                        }
                    }
                    card_modal = <Modal> {
                        content: <View> {
                            width: Fill,
                            height: Fill,
                            flow: Overlay,
                            align: {x: 0.5, y: 0.5}
                            scrim = <View> {
                                width: Fill,
                                height: Fill,
                                draw_bg: { color: #F3F4FFCC }
                            }
                            card_view = <RoundedView> {
                                width: 560,
                                height: 520,
                                flow: Down,
                                padding: {top: 16, right: 16, bottom: 16, left: 16}
                                spacing: 12,
                                draw_bg: {
                                    color: #FFFFFFE6
                                    border_radius: 12.0
                                }
                                header_view = <View> {
                                    width: Fill,
                                    height: Fit,
                                    flow: Right,
                                    spacing: 12,
                                    align: {x: 0.0, y: 0.5}
                                title = <H3> { text: "卡片详情" draw_text: { color: #4A3E4E } }
                                    header_spacer = <View> { width: Fill, height: Fit }
                                    button_view = <View> {
                                        width: Fit,
                                        height: Fit,
                                        flow: Right,
                                        spacing: 8,
                                        cancel_button = <Button> {
                                            width: 90,
                                            height: 32,
                                            text: "Cancel"
                                        }
                                    }
                                }
                                content_view = <View> {
                                    width: Fill,
                                    height: Fill,
                                    flow: Right,
                                    spacing: 12,
                                    body_view = <RoundedView> {
                                        width: Fill,
                                        height: Fill,
                                        flow: Down,
                                        padding: {top: 12, right: 12, bottom: 12, left: 12}
                                        spacing: 10,
                                        draw_bg: {
                                            color: #FFFDF8
                                            border_radius: 10.0
                                        }
                                        <P> { text: "- Shader-based: what does that mean for how things work." draw_text: { color: #5A4660 } }
                                        action_row = <View> {
                                            flow: Right,
                                            spacing: 6,
                                            <Button> { text: "添加" draw_text: { color: #4A3E4E } }
                                            <Button> { text: "标签" draw_text: { color: #4A3E4E } }
                                            <Button> { text: "日期" draw_text: { color: #4A3E4E } }
                                            <Button> { text: "清单" draw_text: { color: #4A3E4E } }
                                            <Button> { text: "成员" draw_text: { color: #4A3E4E } }
                                        }
                                        MarkdownEditorUI = <View> {
                                            width: Fill, height: Fill,
                                            flow: Right,
                                            editor = <View> {
                                                width: Fill, height: Fill,
                                                draw_bg: { color: #f5f5f5 }

                                                editor_input = <TextInput> {
                                                    width: Fill, height: Fill,
                                                    draw_text: {
                                                        text_style: <THEME_FONT_CODE> {}
                                                    }
                                                }
                                            }
                                            preview = <Markdown> {
                                                width: Fill, height: Fit,
                                                draw_bg: { color: #fff }
                                            }
                                        }
                                    }
                                    side_view = <RoundedView> {
                                        width: 180,
                                        height: Fill,
                                        flow: Down,
                                        padding: {top: 12, right: 12, bottom: 12, left: 12}
                                        spacing: 8,
                                        draw_bg: {
                                            color: #F5F9FF
                                            border_radius: 10.0
                                        }
                                        <P> { text: "- Shader-based: what does that mean for how things work." draw_text: { color: #3F4F6A } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        let actions = cx.capture_actions(|cx| {
            self.ui.handle_event(cx, event, &mut Scope::empty());
        });
        self.handle_actions(cx, &actions);
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(card_button)).clicked(actions) {
            self.ui.modal(id!(card_modal)).open(cx);
        }
    }
}

app_main!(App);
