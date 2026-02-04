use chrono;
use makepad_widgets::*;
use std::sync::mpsc::Receiver;

use crate::components::space::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::space::SpaceList;

    App = {{App}} {
        ui: <Root> {
            <Window> {
                window: {
                    title: "Betula - Kanban Board"
                },
                body = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    padding: 20,
                    spacing: 20,

                    <View> {
                        width: Fill,
                        height: Fit,
                        flow: Right,
                        spacing: 20,
                    }

                    <ScrollXYView> {
                        width: Fill,
                        height: Fill,
                        scroll_bars: <ScrollBars> {
                            show_scroll_x: true,
                            show_scroll_y: true,
                        }

                        <SpaceList> {}

                        create_button = <Button> {
                            text: "创建空间",
                            width: 120,
                            height: 40,
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl App {
    fn start_space_fetch(&mut self) {
        // 重置接收器以允许重复调用
        self.state.space_rx = None;

        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.space_signal.clone();
        self.state.space_rx = Some(rx);

        std::thread::spawn(move || {
            let request = reqwest::blocking::get("http://localhost:8911/api/v1/space/byUserId/1");
            match request {
                Ok(response) => match response.json::<Vec<SpaceDto>>() {
                    Ok(spaces) => {
                        let _ = tx.send(spaces);
                        signal.set();
                    }
                    Err(e) => {
                        println!("JSON 解析失败: {}", e);
                    }
                },
                Err(e) => {
                    println!("API 请求失败: {}", e);
                }
            }
        });
    }

    fn handle_space_signal(&mut self, cx: &mut Cx) {
        if !self.state.space_signal.check_and_clear() {
            return;
        }

        // 收集所有接收到的数据
        let mut received_spaces = Vec::new();
        if let Some(rx) = &self.state.space_rx {
            while let Ok(spaces) = rx.try_recv() {
                received_spaces.push(spaces);
            }
        }

        // 处理接收到的数据
        for spaces in received_spaces {
            self.state.spaces_data = spaces;
            
            // 缓存所有卡片的原始文本
            self.state.card_original_texts.clear();
            let mut total_cards = 0;
            for space in &self.state.spaces_data {
                for card in &space.cards {
                    self.state.card_original_texts.insert(card.id, card.title.clone());
                    total_cards += 1;
                }
            }
            
            println!(
                "收到 {} 个空间的数据，共 {} 张卡片，通过 PortalList 实现真正无限制渲染",
                self.state.spaces_data.len(),
                total_cards
            );
            
            // 打印第一个空间的卡片信息用于调试
            if !self.state.spaces_data.is_empty() {
                let first_space = &self.state.spaces_data[0];
                println!("第一个空间 '{}' 有 {} 张卡片", first_space.title, first_space.cards.len());
                for card in &first_space.cards {
                    println!("  - 卡片: {}", card.title);
                }
            }
            
            cx.redraw_all();
        }
    }

    fn start_create_space(&mut self) {
        // 重置接收器以允许重复调用
        self.state.create_space_rx = None;

        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.create_space_signal.clone();
        self.state.create_space_rx = Some(rx);

        std::thread::spawn(move || {
            // 创建新空间的请求数据
            let new_space = CreateSpaceRequest {
                title: format!("新空间 {}", chrono::Utc::now().format("%H:%M:%S")),
                user_id: "1".to_string(),
                canceled: Some(false),
                sort: Some(1),
                color: Some("#FFFFFF".to_string()),
                sort_by: Some("created_at".to_string()),
            };

            // 发送 POST 请求创建空间
            let client = reqwest::blocking::Client::new();
            let request = client
                .post("http://localhost:8911/api/v1/space")
                .header("Content-Type", "application/json")
                .json(&new_space)
                .send();

            match request {
                Ok(response) => {
                    if response.status().is_success() {
                        let _ = tx.send(true);
                    } else {
                        let _ = tx.send(false);
                    }
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    fn handle_create_space_signal(&mut self, _cx: &mut Cx) {
        if !self.state.create_space_signal.check_and_clear() {
            return;
        }

        // 收集所有接收到的结果
        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.create_space_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        // 处理接收到的结果
        for success in received_results {
            if success {
                // 创建成功后自动刷新空间列表
                self.start_space_fetch();
            }
        }
    }

    // 卡片 CRUD 方法
    fn show_card_modal(&mut self, cx: &mut Cx, space_id: i64, card_id: Option<i64>) {
        self.state.current_space_id = Some(space_id);
        self.state.current_card_id = card_id;
        self.state.is_editing_card = card_id.is_some();
        
        if let Some(card_id) = card_id {
            // 编辑模式：填充现有卡片数据
            let card_data = self.find_card_by_id(card_id).map(|card| {
                (card.title.clone(), card.description.clone().unwrap_or_default())
            });
            
            if let Some((title, description)) = card_data {
                self.state.card_title = title;
                self.state.card_description = description;
            }
        } else {
            // 创建模式：清空表单
            self.state.card_title.clear();
            self.state.card_description.clear();
        }
        
        self.state.card_modal_visible = true;
        cx.redraw_all();
    }

    fn hide_card_modal(&mut self, cx: &mut Cx) {
        self.state.card_modal_visible = false;
        self.state.current_space_id = None;
        self.state.current_card_id = None;
        self.state.card_title.clear();
        self.state.card_description.clear();
        cx.redraw_all();
    }

    fn find_card_by_id(&self, card_id: i64) -> Option<&CardDto> {
        for space in &self.state.spaces_data {
            for card in &space.cards {
                if card.id == card_id {
                    return Some(card);
                }
            }
        }
        None
    }

    fn find_space_id_by_card_id(&self, card_id: i64) -> Option<i64> {
        for space in &self.state.spaces_data {
            for card in &space.cards {
                if card.id == card_id {
                    return Some(space.id);
                }
            }
        }
        None
    }

    fn create_card(&mut self) {
        if let Some(space_id) = self.state.current_space_id {
            let (tx, rx) = std::sync::mpsc::channel();
            let signal = self.state.card_signal.clone();
            self.state.card_rx = Some(rx);

            let title = self.state.card_title.clone();
            let description = self.state.card_description.clone();

            std::thread::spawn(move || {
                let new_card = CreateCardRequest {
                    title,
                    description: if description.is_empty() { None } else { Some(description) },
                    status: Some(false),
                    space: SpaceReference { id: space_id },
                };

                let client = reqwest::blocking::Client::new();
                let request = client
                    .post("http://localhost:8911/api/v1/card")
                    .header("Content-Type", "application/json")
                    .json(&new_card)
                    .send();

                match request {
                    Ok(response) => {
                        let success = response.status().is_success();
                        let _ = tx.send(success);
                    }
                    Err(_) => {
                        let _ = tx.send(false);
                    }
                }

                signal.set();
            });
        }
    }

    fn update_card(&mut self) {
        if let (Some(card_id), Some(_space_id)) = (self.state.current_card_id, self.state.current_space_id) {
            let (tx, rx) = std::sync::mpsc::channel();
            let signal = self.state.card_signal.clone();
            self.state.card_rx = Some(rx);

            let title = self.state.card_title.clone();
            let description = self.state.card_description.clone();

            std::thread::spawn(move || {
                let update_card = UpdateCardRequest {
                    title,
                    description: if description.is_empty() { None } else { Some(description) },
                    status: Some(false),
                };

                let client = reqwest::blocking::Client::new();
                let request = client
                    .put(&format!("http://localhost:8911/api/v1/card/{}", card_id))
                    .header("Content-Type", "application/json")
                    .json(&update_card)
                    .send();

                match request {
                    Ok(response) => {
                        let success = response.status().is_success();
                        let _ = tx.send(success);
                    }
                    Err(_) => {
                        let _ = tx.send(false);
                    }
                }

                signal.set();
            });
        }
    }

    fn delete_card(&mut self, card_id: i64) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_signal.clone();
        self.state.card_rx = Some(rx);

        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            let request = client
                .delete(&format!("http://localhost:8911/api/v1/card/{}", card_id))
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    fn create_card_from_input(&mut self, space_id: i64, title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_signal.clone();
        self.state.card_rx = Some(rx);

        println!("create_card_from_input: 开始创建卡片 '{}' 到空间 {}", title, space_id);

        std::thread::spawn(move || {
            let new_card = CreateCardRequest {
                title: title.clone(),
                description: None,
                status: Some(false),
                space: SpaceReference { id: space_id },
            };

            println!("create_card_from_input: 发送API请求，数据: {:?}", new_card);

            let client = reqwest::blocking::Client::new();
            let request = client
                .post("http://localhost:8911/api/v1/card")
                .header("Content-Type", "application/json")
                .json(&new_card)
                .send();

            match request {
                Ok(response) => {
                    let status = response.status();
                    println!("create_card_from_input: API响应状态: {}", status);
                    
                    if status.is_success() {
                        if let Ok(response_text) = response.text() {
                            println!("create_card_from_input: API响应内容: {}", response_text);
                        }
                        let _ = tx.send(true);
                    } else {
                        println!("create_card_from_input: API请求失败，状态码: {}", status);
                        let _ = tx.send(false);
                    }
                }
                Err(e) => {
                    println!("create_card_from_input: API请求错误: {}", e);
                    let _ = tx.send(false);
                }
            }

            println!("create_card_from_input: 设置信号");
            signal.set();
        });
    }

    fn handle_card_signal(&mut self, _cx: &mut Cx) {
        if !self.state.card_signal.check_and_clear() {
            return;
        }

        println!("handle_card_signal: 收到卡片信号");

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.card_rx {
            while let Ok(success) = rx.try_recv() {
                println!("handle_card_signal: 收到结果: {}", success);
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                println!("handle_card_signal: 卡片创建成功，刷新数据");
                self.start_space_fetch(); // 刷新数据
            } else {
                println!("handle_card_signal: 卡片创建失败");
            }
        }
    }

    fn update_space_title(&mut self, space_id: i64, new_title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.space_update_signal.clone();
        self.state.space_update_rx = Some(rx);

        std::thread::spawn(move || {
            let update_space = UpdateSpaceRequest {
                title: new_title,
            };

            let client = reqwest::blocking::Client::new();
            let request = client
                .put(&format!("http://localhost:8911/api/v1/space/{}", space_id))
                .header("Content-Type", "application/json")
                .json(&update_space)
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    fn update_card_title(&mut self, card_id: i64, new_title: String) {
        let (tx, rx) = std::sync::mpsc::channel();
        let signal = self.state.card_update_signal.clone();
        self.state.card_update_rx = Some(rx);

        std::thread::spawn(move || {
            let update_card = UpdateCardRequest {
                title: new_title,
                description: None,
                status: Some(false),
            };

            let client = reqwest::blocking::Client::new();
            let request = client
                .put(&format!("http://localhost:8911/api/v1/card/{}", card_id))
                .header("Content-Type", "application/json")
                .json(&update_card)
                .send();

            match request {
                Ok(response) => {
                    let success = response.status().is_success();
                    let _ = tx.send(success);
                }
                Err(_) => {
                    let _ = tx.send(false);
                }
            }

            signal.set();
        });
    }

    fn handle_space_update_signal(&mut self, _cx: &mut Cx) {
        if !self.state.space_update_signal.check_and_clear() {
            return;
        }

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.space_update_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                self.start_space_fetch(); // 刷新数据
            }
        }
    }

    fn handle_card_update_signal(&mut self, _cx: &mut Cx) {
        if !self.state.card_update_signal.check_and_clear() {
            return;
        }

        let mut received_results = Vec::new();
        if let Some(rx) = &self.state.card_update_rx {
            while let Ok(success) = rx.try_recv() {
                received_results.push(success);
            }
        }

        for success in received_results {
            if success {
                self.start_space_fetch(); // 刷新数据
            }
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.start_space_fetch();
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        match event {
            Event::Startup => {
                self.start_space_fetch();
            }
            Event::Signal => {
                self.handle_space_signal(cx);
                self.handle_create_space_signal(cx);
                self.handle_card_signal(cx);
                self.handle_space_update_signal(cx);
                self.handle_card_update_signal(cx);
            }
            Event::KeyDown(key_event) => {
                // 添加键盘快捷键测试
                if key_event.key_code == makepad_widgets::KeyCode::KeyA {
                    println!("键盘快捷键A被按下，测试添加输入框");
                    if !self.state.spaces_data.is_empty() {
                        let space_id = self.state.spaces_data[0].id;
                        self.state.new_card_inputs.insert(space_id, String::new());
                        println!("通过键盘快捷键添加输入框到空间: {}", space_id);
                        cx.redraw_all();
                    }
                }
            }
            _ => {}
        }

        self.match_event(cx, event);
        let mut scope = Scope::with_data(&mut self.state);
        self.ui.handle_event(cx, event, &mut scope);

        // 处理待处理的按钮点击 - 移除pending_add_card_space_id处理，现在直接在SpaceColumn中处理
        
        if let Some(card_id) = self.state.pending_edit_card_id.take() {
            println!("编辑卡片: {}", card_id);
        }
        
        if let Some(card_id) = self.state.pending_delete_card_id.take() {
            println!("删除卡片: {}", card_id);
            self.delete_card(card_id);
        }
        
        // 处理待处理的更新
        if let Some((space_id, new_title)) = self.state.pending_space_update.take() {
            println!("更新空间标题: {} -> {}", space_id, new_title);
            self.update_space_title(space_id, new_title);
        }
        
        if let Some((card_id, new_title)) = self.state.pending_card_update.take() {
            println!("更新卡片标题: {} -> {}", card_id, new_title);
            self.update_card_title(card_id, new_title);
        }
        
        // 处理新卡片创建
        if let Some((space_id, title)) = self.state.pending_create_card.take() {
            if !title.trim().is_empty() {
                println!("创建新卡片: {} 到空间: {}", title, space_id);
                self.create_card_from_input(space_id, title.trim().to_string());
                // 清除输入框状态
                self.state.new_card_inputs.remove(&space_id);
                cx.redraw_all();
            }
        }
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 处理刷新按钮点击
        if self.ui.button(id!(refresh_button)).clicked(&actions) {
            self.start_space_fetch();
        }

        // 处理创建空间按钮点击
        if self.ui.button(id!(create_button)).clicked(&actions) {
            self.start_create_space();
        }
        
        // 处理创建卡片按钮点击 - 尝试直接访问
        if self.ui.button(id!(create_button)).clicked(&actions) {
            println!("App: 检测到创建卡片按钮点击");
            if !self.state.spaces_data.is_empty() {
                let space_id = self.state.spaces_data[0].id; // 暂时使用第一个空间
                self.state.new_card_inputs.insert(space_id, String::new());
                println!("App: 添加新卡片输入框到空间ID: {}", space_id);
                cx.redraw_all();
            }
        }
        
        // 处理测试添加卡片按钮点击
        if self.ui.button(id!(test_add_card_button)).clicked(&actions) {
            if !self.state.spaces_data.is_empty() {
                let space_id = self.state.spaces_data[0].id;
                println!("测试：添加新卡片输入框到空间 {}", space_id);
                self.state.new_card_inputs.insert(space_id, String::new());
            }
        }

        // 处理卡片相关的按钮点击
        for space in &self.state.spaces_data {
            for card in &space.cards {
                // 处理删除卡片按钮点击
                if self.ui.button(id!(delete_card_btn)).clicked(&actions) {
                    self.state.pending_delete_card_id = Some(card.id);
                    return; // 只处理第一个匹配的按钮
                }
            }
        }

        // 处理新卡片输入框的文本变化和失去焦点事件
        // 简化版本：暂时通过其他方式处理输入框事件
        // TODO: 实现完整的 TextInput 事件处理
    }
}


pub struct State {
    pub spaces_data: Vec<SpaceDto>,
    pub space_signal: SignalToUI,
    pub space_rx: Option<Receiver<Vec<SpaceDto>>>,
    pub create_space_signal: SignalToUI,
    pub create_space_rx: Option<Receiver<bool>>,
    
    // 卡片 CRUD 相关
    pub card_signal: SignalToUI,
    pub card_rx: Option<Receiver<bool>>,
    pub current_space_id: Option<i64>,
    pub current_card_id: Option<i64>,
    pub card_modal_visible: bool,
    pub card_title: String,
    pub card_description: String,
    pub is_editing_card: bool,
    
    // 按钮点击状态
    pub pending_add_card_space_id: Option<i64>,
    pub pending_edit_card_id: Option<i64>,
    pub pending_delete_card_id: Option<i64>,
    
    // 内联编辑状态
    pub editing_space_id: Option<i64>,
    pub editing_card_id: Option<i64>,
    pub space_update_signal: SignalToUI,
    pub space_update_rx: Option<Receiver<bool>>,
    pub card_update_signal: SignalToUI,
    pub card_update_rx: Option<Receiver<bool>>,
    
    // 待处理的更新
    pub pending_space_update: Option<(i64, String)>,
    pub pending_card_update: Option<(i64, String)>,
    
    // 新卡片输入框状态
    pub new_card_inputs: std::collections::HashMap<i64, String>, // space_id -> input_text
    pub pending_create_card: Option<(i64, String)>, // space_id, title
    
    // 卡片原始文本缓存（用于对比是否有变化）
    pub card_original_texts: std::collections::HashMap<i64, String>, // card_id -> original_text
}

impl Default for State {
    fn default() -> Self {
        Self {
            spaces_data: Vec::new(),
            space_signal: SignalToUI::default(),
            space_rx: None,
            create_space_signal: SignalToUI::default(),
            create_space_rx: None,
            
            // 卡片 CRUD 相关
            card_signal: SignalToUI::default(),
            card_rx: None,
            current_space_id: None,
            current_card_id: None,
            card_modal_visible: false,
            card_title: String::new(),
            card_description: String::new(),
            is_editing_card: false,
            
            // 按钮点击状态
            pending_add_card_space_id: None,
            pending_edit_card_id: None,
            pending_delete_card_id: None,
            
            // 内联编辑状态
            editing_space_id: None,
            editing_card_id: None,
            space_update_signal: SignalToUI::default(),
            space_update_rx: None,
            card_update_signal: SignalToUI::default(),
            card_update_rx: None,
            
            // 待处理的更新
            pending_space_update: None,
            pending_card_update: None,
            
            // 新卡片输入框状态
            new_card_inputs: std::collections::HashMap::new(),
            pending_create_card: None,
            
            // 卡片原始文本缓存
            card_original_texts: std::collections::HashMap::new(),
        }
    }
}

// 辅助函数：格式化卡片信息
fn format_cards_info(cards: &[CardDto]) -> String {
    if cards.is_empty() {
        "暂无卡片".to_string()
    } else {
        let mut info = format!("{} 张卡片:\n", cards.len());
        for (idx, card) in cards.iter().enumerate() {
            if idx < 3 {
                // 最多显示3张卡片
                info.push_str(&format!("• {}\n", card.title));
            } else if idx == 3 {
                info.push_str(&format!("... 还有 {} 张卡片", cards.len() - 3));
                break;
            }
        }
        info
    }
}

app_main!(App);

