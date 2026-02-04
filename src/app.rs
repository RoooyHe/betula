use makepad_widgets::*;
use std::sync::mpsc::Receiver;
use crate::components::space::{SpaceDto, get_spaces_data};

live_design! {
    use link::theme::*;
    use link::widgets::*;

    use crate::components::card_list::*;
    use crate::components::card_modal::*;
    use crate::components::space::*;

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
                        flow: Down,
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
                                align: {x: 0.0, y: 0.5}

                                space_tabs = <View> {
                                    width: Fit,
                                    height: Fit,
                                    flow: Right,
                                    spacing: 4,
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

                            space = <Space> {}
                        }
                        card_modal = <CardModal> {}
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
    space_signal: SignalToUI,

    #[rust]
    space_rx: Option<Receiver<Vec<SpaceDto>>>,
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
            match request {
                Ok(response) => {
                    match response.json::<Vec<SpaceDto>>() {
                        Ok(spaces) => {
                            let _ = tx.send(spaces);
                            signal.set();
                        }
                        Err(e) => {
                            println!("Failed to parse JSON: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to fetch from API: {}", e);
                }
            }
        });
    }

    fn handle_space_signal(&mut self, cx: &mut Cx) {
        if !self.space_signal.check_and_clear() {
            return;
        }
        
        // 收集所有待处理的空间数据
        let mut spaces_list = Vec::new();
        if let Some(rx) = &self.space_rx {
            while let Ok(spaces) = rx.try_recv() {
                spaces_list.push(spaces);
            }
        }
        
        // 写入共享状态
        let shared_data = get_spaces_data();
        for spaces in spaces_list {
            *shared_data.lock().unwrap() = spaces.clone();
            self.apply_spaces(cx, &spaces);
        }
    }

    fn apply_spaces(&mut self, cx: &mut Cx, spaces: &[SpaceDto]) {
        // 触发重新渲染
        cx.redraw_all();
        
        println!("=== APPLY SPACES DEBUG ===");
        println!("apply_spaces: received {} spaces", spaces.len());
        for (i, space) in spaces.iter().enumerate() {
            println!("  Space {}: '{}' with {} cards", i, space.title, space.cards.len());
            for (j, card) in space.cards.iter().enumerate() {
                println!("    Card {}: '{}'", j, card.title);
            }
        }
        println!("=== END DEBUG ===");
        
        if spaces.is_empty() {
            // 没有空间时更新第一个 CardList 的标题
            self.ui
                .view(id!(space))
                .label(id!(header_row.title))
                .set_text(cx, "暂无空间");
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        println!("=== APP EVENT: {:?} ===", event);
        
        if let Event::Startup = event {
            println!("App started! Starting space fetch...");
            self.start_space_fetch();
        }
        if let Event::Signal = event {
            println!("Received signal!");
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
        crate::components::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 处理卡片模态框的关闭按钮
        if self.ui.button(id!(card_modal.close_button)).clicked(actions)
            || self.ui.button(id!(card_modal.cancel_button)).clicked(actions)
        {
            self.ui.modal(id!(card_modal)).close(cx);
        }
        
        // 简化的事件处理 - 检查任何按钮点击
        for action in actions.iter() {
            if let Some(button_action) = action.as_widget_action() {
                if button_action.action.is::<ButtonAction>() {
                    // 简单检查：如果有按钮被点击，就打开模态框
                    // 这是一个临时解决方案，后续可以通过更精确的事件处理来优化
                    println!("Button clicked! Opening modal...");
                    self.ui.modal(id!(card_modal)).open(cx);
                    break;
                }
            }
        }
    }
}

impl App {}

app_main!(App);
