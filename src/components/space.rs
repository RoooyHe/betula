use makepad_widgets::*;
use serde::{Deserialize, Serialize};

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use crate::components::card_list::CardList;

    pub SpaceColumn = <RoundedView> {
        width: 300,
        height: 600,
        padding: 15,
        flow: Down,
        spacing: 10,
        draw_bg: {
            color: #E8F4FDFF
        }

        <RoundedView> {
            width: Fill,
            height: Fit,
            flow: Right,
            align: {y: 0.5},

            space_title_input = <TextInput> {
                width: Fill,
                height: 35,
                text: "空间标题",
                draw_text: {
                    color: #333333FF,
                    text_style: {
                        font_size: 18.0,
                    }
                }
                draw_bg: {
                    color: #F8F9FAFF
                }
            }

            add_card_btn = <Button> {
                width: 80,
                height: 30,
                text: "添加卡片",
                draw_bg: {
                    color: #00CED1FF
                }
            }
        }

        <ScrollXYView> {
            width: Fill,
            height: Fill,
            scroll_bars: <ScrollBars> {
                show_scroll_x: false,
                show_scroll_y: true,
            }

            <CardList> {}
        }
    }

    pub SpaceList = {{SpaceList}} {
        spaces = <PortalList> {
            flow: Right,
            spacing: 20,

            Space = <SpaceColumn> {}
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SpaceList {
    #[deref]
    view: View,
}

impl Widget for SpaceList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // 处理添加卡片按钮点击事件
        if let Event::Actions(actions) = event {
            if self.view.button(id!(add_card_btn)).clicked(actions) {
                // 找到被点击的空间
                let state = scope.data.get_mut::<crate::app::State>().unwrap();
                
                // 遍历所有空间，找到被点击的按钮对应的空间
                for space_idx in 0..state.spaces_data.len() {
                    let space_id = state.spaces_data[space_idx].id;
                    
                    // 设置待处理的添加卡片请求
                    state.pending_add_card_space_id = Some(space_id);
                    println!("SpaceList: 检测到添加卡片按钮点击，空间ID: {}", space_id);
                    break; // 只处理第一个匹配的按钮
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            let state = scope.data.get_mut::<crate::app::State>().unwrap();

            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, state.spaces_data.len());

                while let Some(space_idx) = list.next_visible_item(cx) {
                    if space_idx >= state.spaces_data.len() {
                        continue;
                    }

                    let space_item = list.item(cx, space_idx, live_id!(Space));
                    let space = &state.spaces_data[space_idx];

                    // 设置空间标题输入框
                    space_item
                        .text_input(id!(space_title_input))
                        .set_text(cx, &space.title);

                    // 设置背景颜色
                    let colors = [
                        0xE8F4FDFFu32, // 浅蓝色
                        0xF0FDF4FFu32, // 浅绿色
                        0xFEF3C7FFu32, // 浅黄色
                        0xFDF2F8FFu32, // 浅粉色
                        0xF3E8FFFFu32, // 浅紫色
                        0xFFF1F2FFu32, // 浅红色
                        0xE0F2FEFFu32, // 浅青色
                        0xF0FFF4FFu32, // 浅薄荷绿
                    ];
                    let color_index = space_idx % colors.len();
                    let bg_color = colors[color_index];

                    space_item.apply_over(
                        cx,
                        live! {
                            draw_bg: {
                                color: (bg_color)
                            }
                        },
                    );

                    // 为 CardList 传递 space_idx
                    let mut scope = Scope::with_data_props(state, &space_idx);
                    space_item.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }
}

// DTO 定义
#[derive(Clone, Deserialize, Debug)]
pub struct TagDto {
    pub id: i64,
    pub title: String,
    pub color: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CardDto {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SpaceDto {
    pub id: i64,
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    pub cards: Vec<CardDto>,
}

// 创建 Space 的请求数据结构
#[derive(Serialize, Debug)]
pub struct CreateSpaceRequest {
    pub title: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    pub canceled: Option<bool>,
    pub sort: Option<i32>,
    pub color: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
}

// 创建卡片的请求数据结构
#[derive(Serialize, Debug)]
pub struct CreateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
    #[serde(rename = "spaceId")]
    pub space_id: i64,
}

// 更新卡片的请求数据结构
#[derive(Serialize, Debug)]
pub struct UpdateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub status: Option<bool>,
}

// 更新空间的请求数据结构
#[derive(Serialize, Debug)]
pub struct UpdateSpaceRequest {
    pub title: String,
}
