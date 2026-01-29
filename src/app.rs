use makepad_widgets::*;

live_design! {
    use link::widgets::*;

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
                    flow: Right,
                    spacing: 10,
                    margin: 10,

                    <RoundedView> {
                        width: Fill,
                        height: Fit,
                        margin: 10,
                        padding: 5,
                        draw_bg: {
                            color: #fff
                            border_radius: 5.0
                        }
                        flow: Down,
                        title = <H3> {
                            text: "Trello新手指南"
                        }

                        card_button = <Button> {
                            text: "TestButton"
                            width: Fill,

                        }
                          <Button> { text: "TestButton"}
                          <Button> { text: "TestButton"}
                    }
                    <RoundedView> {
                        width: Fill, height: Fit,
                        flow: Right,
                        <P> { text: "- Shader-based: what does that mean for how things work." }
                    }

                    <RoundedView> {
                        width: Fill, height: Fit,
                        flow: Right,
                        <P> { text: "- Shader-based: what does that mean for how things work." }
                    }
                }
                card_modal = <Modal> {
                    content: {
                        full_view = <RoundedView> {
                            width: 400,
                            height: 800,
                            flow: Down,
                            draw_bg: {
                                color: #CCC
                            }
                            header_view = <View> {
                                width: 400,
                                height: 200,
                                flow: Right,
                                show_bg: true
                                button_view = <View> {
                                    width: 200,
                                    height: Fill,
                                    button = <Button> {
                                        align: {x: 0.5, y: 0.5}
                                        width: 50, height: Fill
                                        text: "Cancel"
                                    }
                                }
                            }
                            content_view = <View> {
                                width: 400,
                                height: 600,
                                flow: Right,
                                <View> {
                                    flow: Down,
                                    <P> { text: "- Shader-based: what does that mean for how things work." }
                                    <View> {
                                        flow: Right,
                                        <Button> { text: "添加" }
                                        <Button> { text: "标签" }
                                        <Button> { text: "日期" }
                                        <Button> { text: "清单" }
                                        <Button> { text: "成员" }
                                    }
                                    <View> {

                                    }
                                }
                                <View> {<P> { text: "- Shader-based: what does that mean for how things work." }}
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
