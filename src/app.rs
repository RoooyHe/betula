use makepad_widgets::*;
use serde::Deserialize;
use std::sync::mpsc::Receiver;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::markdown::*;
    use crate::design::*;

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

                    // 顶部导航栏
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

                            // 面板工具栏（包含 Space 标签）
                            panel_view = <View> {
                                width: Fill,
                                height: Fit,
                                flow: Right,
                                spacing: 6,
                                align: {x: 0.0, y: 0.5}

                                space_tabs = <View> {
                                    width: Fit,
                                    height: Fit,
                                    flow: Right,
                                    spacing: 4,
                                    space_item_1 = <SpaceItem> { text: "加载中..." }
                                }

                                <View> { width: 20, height: Fit }

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

                            // 卡片列表区域
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

                        // 卡片详情弹窗
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

                                    // 头部区域
                                    header_section = <View> {
                                        width: Fill,
                                        height: Fit,
                                        padding: {top: 12, right: 16, bottom: 12, left: 52}
                                        flow: Right,
                                        align: {x: 0.0, y: 0.5}
                                        draw_bg: { color: #F4F5F7FF }

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
                                                            text: "实现 API 接口"
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
                                                            text: "测试部署"
                                                            draw_text: {
                                                                color: #172B4DFF
                                                                text_style: { font_size: 14 }
                                                            }
                                                        }
                                                    }
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

#[derive(Clone, Deserialize)]
struct SpaceDto {
    id: i64,
    title: String,
    user_id: String,
    canceled: Option<bool>,
    sort: Option<i32>,
    color: Option<String>,
    sort_by: Option<String>,
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    card_signal: SignalToUI,
    #[rust]
    space_signal: SignalToUI,
    #[rust]
    space_rx: Option<Receiver<Vec<SpaceDto>>>,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Startup = event {
            self.start_space_fetch();
        }
        if let Event::Signal = event {
            self.handle_space_signal(cx);
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
        if self
            .ui
            .button(id!(card_list_1.card_item_1))
            .clicked(actions)
            || self
                .ui
                .button(id!(card_list_1.card_item_2))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_1.card_item_3))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_2.card_item_1))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_2.card_item_2))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_2.card_item_3))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_3.card_item_1))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_3.card_item_2))
                .clicked(actions)
            || self
                .ui
                .button(id!(card_list_3.card_item_3))
                .clicked(actions)
        {
            println!("Card clicked! Opening modal...");
            self.ui.modal(id!(card_modal)).open(cx);
        }

        // 关闭按钮和归档按钮都可以关闭弹窗
        if self.ui.button(id!(close_button)).clicked(actions)
            || self.ui.button(id!(cancel_button)).clicked(actions)
        {
            self.ui.modal(id!(card_modal)).close(cx);
        }
    }
}

impl App {
    fn start_space_fetch(&mut self) {
        if self.space_rx.is_some() {
            return;
        }
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.space_signal.clone();
        self.space_rx = Some(rx);
        std::thread::spawn(move || {
            let request = reqwest::blocking::get("http://localhost:8911/api/v1/space/byUserId/1");
            if let Ok(response) = request {
                if let Ok(spaces) = response.json::<Vec<SpaceDto>>() {
                    let _ = tx.send(spaces);
                    signal.set();
                }
            }
        });
    }

    fn handle_space_signal(&mut self, cx: &mut Cx) {
        if !self.space_signal.check_and_clear() {
            return;
        }
        let mut all_spaces = Vec::new();
        if let Some(rx) = &self.space_rx {
            while let Ok(spaces) = rx.try_recv() {
                all_spaces.push(spaces);
            }
        }
        for spaces in all_spaces {
            self.apply_spaces(cx, &spaces);
        }
    }

    fn apply_spaces(&mut self, cx: &mut Cx, spaces: &[SpaceDto]) {
        for (index, space) in spaces.iter().enumerate() {
            if index == 0 {
                self.ui
                    .button(id!(space_tabs.space_item_1))
                    .set_text(cx, &space.title);
                println!("Loaded space: {} (id={})", space.title, space.id);
            }
        }
        if spaces.is_empty() {
            self.ui
                .button(id!(space_tabs.space_item_1))
                .set_text(cx, "暂无空间");
        }
    }
}

app_main!(App);
