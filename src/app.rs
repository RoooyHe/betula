use makepad_widgets::*;
use serde::Deserialize;
use std::sync::mpsc::Receiver;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::markdown::*;

    CardButton = <Button> {
        text: "TestButton"
        width: Fill,
        draw_text: { color: #4A3E4EFF }
    }

    CardItem = <Button> {
        width: Fill,
        height: Fit,
        padding: {top: 8, right: 8, bottom: 8, left: 8}
        draw_bg: {
            color: #FFFFFFFF
            border_radius: 10.0
        }
        draw_text: { text_style: { font_size: 0.0 } } // 隐藏按钮文字

        layout: {
            flow: Down,
            spacing: 6,
        }

        tag_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            tag = <RoundedView> {
                width: Fit,
                height: Fit,
                padding: {top: 2, right: 6, bottom: 2, left: 6}
                draw_bg: {
                    color: #9FE7B4FF
                    border_radius: 6.0
                }
                tag_text = <Label> { text: "非紧急" draw_text: { color: #2E5A45FF } }
            }
        }
        title_text = <Label> { text: "完成代码编辑" draw_text: { color: #3E3342FF } }
        meta_row = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            spacing: 8,
            date_text = <Label> { text: "2月7日" draw_text: { color: #6A5A72FF } }
            progress_text = <Label> { text: "0/2" draw_text: { color: #6A5A72FF } }
        }
    }

    CardList = <RoundedView> {
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
                    flow: Down,
                    draw_bg: {
                        color: #F8F6FFFF
                    }
                    header_view = <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right
                        spacing: 8,
                        padding: {top: 8, right: 12, bottom: 8, left: 12}
                        <Button> { text: "View" }
                        <Button> { text: "Betula" }
                        <TextInput> { width: 220 }
                        <Button> { text: "创建" }
                        <Button> { text: "意见" }
                        <Button> { text: "通知" }
                        <Button> { text: "信息" }
                        <Button> { text: "账户" }
                    }

                    main_view = <View> {
                        width: Fill,
                        height: Fill,
                        flow: Overlay,
                        content_row = <View> {
                            width: Fill,
                            height: Fill,
                            flow: Down,
                            spacing: 10,
                            margin: 10,

                            panel_view = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Right,
                                spacing: 6,
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

                            card_list_1 = <CardList> {}
                            card_list_2 = <CardList> {}
                            card_list_3 = <CardList> {}
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
                                    draw_bg: { color: #000000AA }
                                }
                                card_view = <RoundedView> {
                                    width: 768,
                                    height: 600,
                                    flow: Down,
                                    padding: 0,
                                    draw_bg: {
                                        color: #F4F5F7FF
                                        border_radius: 8.0
                                    }

                                    // 头部区域 - 包含标题和关闭按钮
                                    header_section = <View> {
                                        width: Fill,
                                        height: Fit,
                                        padding: {top: 12, right: 16, bottom: 12, left: 52}
                                        flow: Right,
                                        align: {x: 0.0, y: 0.5}
                                        draw_bg: { color: #F4F5F7FF }

                                        // 标题图标
                                        icon_area = <View> {
                                            width: Fit,
                                            height: Fit,
                                            align: {x: 0.0, y: 0.5}
                                            <Label> {
                                                text: "▢"
                                                draw_text: {
                                                    color: #5E6C84FF
                                                    text_style: { font_size: 24 }
                                                }
                                            }
                                        }

                                        title_area = <View> {
                                            width: Fill,
                                            height: Fit,
                                            flow: Down,
                                            spacing: 4,
                                            margin: {left: 8}

                                            card_title = <TextInput> {
                                                width: Fill,
                                                height: Fit,
                                                text: "完成代码编辑"
                                                draw_text: {
                                                    text_style: {
                                                        font_size: 20,
                                                        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                    }
                                                    color: #172B4DFF
                                                }
                                                draw_bg: {
                                                    color: #00000000
                                                }
                                            }

                                            list_info = <Label> {
                                                text: "在列表 今天 中"
                                                draw_text: {
                                                    text_style: { font_size: 12 }
                                                    color: #5E6C84FF
                                                }
                                            }
                                        }

                                        header_spacer = <View> { width: Fill, height: Fit }

                                        close_button = <Button> {
                                            width: 32,
                                            height: 32,
                                            text: "✕"
                                            draw_text: {
                                                color: #5E6C84FF
                                                text_style: { font_size: 18 }
                                            }
                                            draw_bg: {
                                                color: #00000000
                                                border_radius: 4.0
                                            }
                                        }
                                    }

                                    // 主要内容区域
                                    content_scroll = <View> {
                                        width: Fill,
                                        height: Fill,
                                        flow: Right,
                                        padding: {top: 0, right: 16, bottom: 16, left: 16}
                                        spacing: 16,

                                        // 左侧主内容区
                                        main_content = <View> {
                                            width: Fill,
                                            height: Fill,
                                            flow: Down,
                                            spacing: 24,

                                            // 成员和标签
                                            meta_section = <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: 16,

                                                // 成员
                                                members_row = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Down,
                                                    spacing: 8,

                                                    <Label> {
                                                        text: "成员"
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: {
                                                                font_size: 12
                                                                font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                            }
                                                        }
                                                    }

                                                    members_list = <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Right,
                                                        spacing: 4,

                                                        <RoundedView> {
                                                            width: 32,
                                                            height: 32,
                                                            draw_bg: {
                                                                color: #DFE1E6FF
                                                                border_radius: 16.0
                                                            }
                                                            align: {x: 0.5, y: 0.5}
                                                            <Label> {
                                                                text: "+"
                                                                draw_text: { color: #5E6C84FF}
                                                            }
                                                        }
                                                    }
                                                }

                                                // 标签
                                                labels_row = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Down,
                                                    spacing: 8,

                                                    <Label> {
                                                        text: "标签"
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: {
                                                                font_size: 12
                                                                font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                            }
                                                        }
                                                    }

                                                    labels_list = <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Right,
                                                        spacing: 4,
                                                        align: {x: 0.0, y: 0.5}

                                                        <RoundedView> {
                                                            width: Fit,
                                                            height: Fit,
                                                            padding: {top: 6, right: 12, bottom: 6, left: 12}
                                                            draw_bg: {
                                                                color: #61BD4FFF
                                                                border_radius: 3.0
                                                            }
                                                            <Label> {
                                                                text: "进行中"
                                                                draw_text: {
                                                                    color: #FFFFFFFF
                                                                    text_style: {
                                                                        font_size: 12
                                                                        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        <RoundedView> {
                                                            width: 32,
                                                            height: 32,
                                                            draw_bg: {
                                                                color: #DFE1E6FF
                                                                border_radius: 3.0
                                                            }
                                                            align: {x: 0.5, y: 0.5}
                                                            <Label> {
                                                                text: "+"
                                                                draw_text: { color: #5E6C84FF}
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // 描述部分
                                            description_section = <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: 8,

                                                desc_header = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Right,
                                                    align: {x: 0.0, y: 0.5}
                                                    spacing: 8,

                                                    <Label> {
                                                        text: "☰"
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: { font_size: 20 }
                                                        }
                                                    }

                                                    <Label> {
                                                        text: "描述"
                                                        draw_text: {
                                                            color: #172B4DFF
                                                            text_style: {
                                                                font_size: 16
                                                                font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                            }
                                                        }
                                                    }

                                                    desc_spacer = <View> { width: Fill, height: Fit }

                                                    edit_button = <Button> {
                                                        text: "编辑"
                                                        padding: {top: 6, right: 12, bottom: 6, left: 12}
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: { font_size: 12 }
                                                        }
                                                        draw_bg: {
                                                            color: #091E4208
                                                            border_radius: 3.0
                                                        }
                                                    }
                                                }

                                                description_content = <RoundedView> {
                                                    width: Fill,
                                                    height: Fit,
                                                    padding: {top: 8, right: 12, bottom: 8, left: 12}
                                                    margin: {left: 32}
                                                    draw_bg: {
                                                        color: #FFFFFFFF
                                                        border_radius: 3.0
                                                    }

                                                    <TextInput> {
                                                        width: Fill,
                                                        height: 100,
                                                        text: "添加更详细的描述..."
                                                        draw_text: {
                                                            text_style: { font_size: 14 }
                                                            color: #5E6C84FF
                                                        }
                                                        draw_bg: { color: #00000000 }
                                                    }
                                                }
                                            }

                                            // 清单部分
                                            checklist_section = <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: 12,

                                                checklist_header = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Right,
                                                    align: {x: 0.0, y: 0.5}
                                                    spacing: 8,

                                                    <Label> {
                                                        text: "☑"
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: { font_size: 20 }
                                                        }
                                                    }

                                                    <Label> {
                                                        text: "清单"
                                                        draw_text: {
                                                            color: #172B4DFF
                                                            text_style: {
                                                                font_size: 16
                                                                font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                            }
                                                        }
                                                    }
                                                }

                                                // 进度条
                                                progress_bar = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Down,
                                                    spacing: 4,
                                                    margin: {left: 32}

                                                    progress_text = <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Right,

                                                        <Label> {
                                                            text: "0%"
                                                            draw_text: {
                                                                color: #5E6C84FF
                                                                text_style: { font_size: 11 }
                                                            }
                                                        }
                                                    }

                                                    progress_track = <RoundedView> {
                                                        width: Fill,
                                                        height: 8,
                                                        draw_bg: {
                                                            color: #DFE1E6FF
                                                            border_radius: 4.0
                                                        }
                                                    }
                                                }

                                                // 清单项目
                                                checklist_items = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Down,
                                                    spacing: 8,
                                                    margin: {left: 32}

                                                    <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Right,
                                                        spacing: 8,
                                                        align: {x: 0.0, y: 0.5}

                                                        <RoundedView> {
                                                            width: 16,
                                                            height: 16,
                                                            draw_bg: {
                                                                color: #FFFFFFFF
                                                                border_color: #DFE1E6FF
                                                                border_width: 1.5
                                                                border_radius: 2.0
                                                            }
                                                        }

                                                        <Label> {
                                                            text: "完成 UI 设计"
                                                            draw_text: {
                                                                color: #172B4DFF
                                                                text_style: { font_size: 14 }
                                                            }
                                                        }
                                                    }

                                                    <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Right,
                                                        spacing: 8,
                                                        align: {x: 0.0, y: 0.5}

                                                        <RoundedView> {
                                                            width: 16,
                                                            height: 16,
                                                            draw_bg: {
                                                                color: #FFFFFFFF
                                                                border_color: #DFE1E6FF
                                                                border_width: 1.5
                                                                border_radius: 2.0
                                                            }
                                                        }

                                                        <Label> {
                                                            text: "实现后端接口"
                                                            draw_text: {
                                                                color: #172B4DFF
                                                                text_style: { font_size: 14 }
                                                            }
                                                        }
                                                    }

                                                    add_item_button = <Button> {
                                                        text: "添加项目"
                                                        width: Fit,
                                                        padding: {top: 6, right: 12, bottom: 6, left: 12}
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: { font_size: 12 }
                                                        }
                                                        draw_bg: {
                                                            color: #091E4208
                                                            border_radius: 3.0
                                                        }
                                                    }
                                                }
                                            }

                                            // 活动部分
                                            activity_section = <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: 12,

                                                activity_header = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Right,
                                                    align: {x: 0.0, y: 0.5}
                                                    spacing: 8,

                                                    <Label> {
                                                        text: "◐"
                                                        draw_text: {
                                                            color: #5E6C84FF
                                                            text_style: { font_size: 20 }
                                                        }
                                                    }

                                                    <Label> {
                                                        text: "活动"
                                                        draw_text: {
                                                            color: #172B4DFF
                                                            text_style: {
                                                                font_size: 16
                                                                font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                            }
                                                        }
                                                    }
                                                }

                                                comment_input = <RoundedView> {
                                                    width: Fill,
                                                    height: Fit,
                                                    padding: {top: 8, right: 12, bottom: 8, left: 12}
                                                    margin: {left: 32}
                                                    draw_bg: {
                                                        color: #FFFFFFFF
                                                        border_radius: 3.0
                                                        border_color: #DFE1E6FF
                                                        border_width: 1.0
                                                    }

                                                    <TextInput> {
                                                        width: Fill,
                                                        height: 60,
                                                        text: "编写评论..."
                                                        draw_text: {
                                                            text_style: { font_size: 14 }
                                                            color: #5E6C84FF
                                                        }
                                                        draw_bg: { color: #00000000 }
                                                    }
                                                }
                                            }
                                        }

                                        // 右侧操作栏
                                        sidebar = <View> {
                                            width: 168,
                                            height: Fit,
                                            flow: Down,
                                            spacing: 8,

                                            <Label> {
                                                text: "添加到卡片"
                                                draw_text: {
                                                    color: #5E6C84FF
                                                    text_style: {
                                                        font_size: 12
                                                        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                    }
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "成员"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "标签"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "清单"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "日期"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "附件"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <View> { height: 16 }

                                            <Label> {
                                                text: "操作"
                                                draw_text: {
                                                    color: #5E6C84FF
                                                    text_style: {
                                                        font_size: 12
                                                        font: {path: dep("crate://makepad-widgets/resources/IBMPlexSans-SemiBold.ttf")}
                                                    }
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "移动"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "复制"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "分享"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
                                                }
                                            }

                                            <View> { height: 8 }

                                            cancel_button = <Button> {
                                                width: Fill,
                                                height: 32,
                                                text: "归档"
                                                draw_text: {
                                                    color: #172B4DFF
                                                    text_style: { font_size: 14 }
                                                }
                                                draw_bg: {
                                                    color: #091E4214
                                                    border_radius: 3.0
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
        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    card_signal: SignalToUI,
    #[rust]
    card_rx: Option<Receiver<Vec<CardDto>>>,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.start_card_fetch();
        }
        if let Event::Signal = event {
            self.handle_card_signal(cx);
        }
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
        // 处理三个列表中的卡片点击事件
        // 直接使用 card_item 作为按钮
        if self.ui.button(id!(card_list_1.card_item_1)).clicked(actions)
            || self.ui.button(id!(card_list_1.card_item_2)).clicked(actions)
            || self.ui.button(id!(card_list_1.card_item_3)).clicked(actions)
            || self.ui.button(id!(card_list_2.card_item_1)).clicked(actions)
            || self.ui.button(id!(card_list_2.card_item_2)).clicked(actions)
            || self.ui.button(id!(card_list_2.card_item_3)).clicked(actions)
            || self.ui.button(id!(card_list_3.card_item_1)).clicked(actions)
            || self.ui.button(id!(card_list_3.card_item_2)).clicked(actions)
            || self.ui.button(id!(card_list_3.card_item_3)).clicked(actions)
        {
            println!("Card clicked! Opening modal...");
            self.ui.modal(id!(card_modal)).open(cx);
        }

        // 关闭按钮和归档按钮都可以关闭弹窗
        if self.ui.button(id!(close_button)).clicked(actions)
            || self.ui.button(id!(cancel_button)).clicked(actions) {
            println!("Closing modal...");
            self.ui.modal(id!(card_modal)).close(cx);
        }
    }
}

#[derive(Clone, Deserialize)]
struct CardDto {
    id: String,
    title: String,
    status: bool,
    #[serde(rename = "endTime")]
    end_time: Option<String>,
    description: Option<String>,
    #[serde(rename = "todoList")]
    todo_list: Vec<String>,
    tags: Vec<serde_json::Value>,
    active: Vec<String>,
}

impl App {
    fn start_card_fetch(&mut self) {
        if self.card_rx.is_some() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.card_signal.clone();
        self.card_rx = Some(rx);
        std::thread::spawn(move || {
            let request = reqwest::blocking::get("http://localhost:8080/api/v1/cards");
            if let Ok(response) = request {
                if let Ok(cards) = response.json::<Vec<CardDto>>() {
                    let _ = tx.send(cards);
                    signal.set();
                }
            }
        });
    }

    fn handle_card_signal(&mut self, cx: &mut Cx) {
        if !self.card_signal.check_and_clear() {
            return;
        }
        // 先收集所有数据，避免借用冲突
        let mut all_cards = Vec::new();
        if let Some(rx) = &self.card_rx {
            while let Ok(cards) = rx.try_recv() {
                all_cards.push(cards);
            }
        }
        // 然后处理收集到的数据
        for cards in all_cards {
            self.apply_cards(cx, &cards);
        }
    }

    fn apply_cards(&mut self, cx: &mut Cx, cards: &[CardDto]) {
        // 可以根据实际需求分配卡片到不同列表
        // 这里简单地将所有卡片应用到每个列表
        self.apply_cards_to_list(cx, id!(card_list_1), cards);
        self.apply_cards_to_list(cx, id!(card_list_2), cards);
        self.apply_cards_to_list(cx, id!(card_list_3), cards);
    }

    fn apply_cards_to_list(&mut self, cx: &mut Cx, list_id: &[LiveId; 1], cards: &[CardDto]) {
        let items = [id!(card_item_1), id!(card_item_2), id!(card_item_3)];

        for (index, item_id) in items.iter().enumerate() {
            if let Some(card) = cards.get(index) {
                // 构建完整的 ID 路径来设置每个卡片
                self.set_card_item(cx, list_id[0], item_id[0], card);
            }
        }
    }

    fn set_card_item(&mut self, cx: &mut Cx, list_id: LiveId, item_id: LiveId, card: &CardDto) {
        let tag_text = if card.status {
            "已完成"
        } else {
            "进行中"
        };

        let date_text = card
            .end_time
            .as_deref()
            .and_then(|value| value.split('T').next())
            .unwrap_or("-");

        let total = card.todo_list.len();
        let progress_text = if card.status {
            format!("{}/{}", total, total)
        } else {
            format!("0/{}", total)
        };

        // 使用完整的 ID 路径访问嵌套组件
        // CardItem 现在是 Button，所以路径是: card_list_X.card_item_X.tag_row.tag.tag_text
        self.ui.label(&[list_id, item_id, id!(tag_row)[0], id!(tag)[0], id!(tag_text)[0]])
            .set_text(cx,tag_text);

        self.ui.label(&[list_id, item_id, id!(title_text)[0]])
            .set_text(cx,&card.title);

        self.ui.label(&[list_id, item_id, id!(meta_row)[0], id!(date_text)[0]])
            .set_text(cx,date_text);

        self.ui.label(&[list_id, item_id, id!(meta_row)[0], id!(progress_text)[0]])
            .set_text(cx,&progress_text);
    }
}

app_main!(App);