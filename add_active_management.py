#!/usr/bin/env python3
"""
脚本：为 src/app.rs 添加 Active 管理功能
包括：
1. 修改模态框布局为 900x600，36%/64% 分栏
2. 添加 Active 管理 UI（右侧区域）
3. 添加 Active 相关方法
4. 添加 Active 事件处理
"""

import re

# 读取当前文件
with open('src/app.rs', 'r', encoding='utf-8') as f:
    content = f.read()

# 1. 修改模态框尺寸和布局
# 找到 RoundedView 定义并修改
old_modal_start = '''                            <RoundedView> {
                                width: 600,
                                height: 500,'''

new_modal_start = '''                            <RoundedView> {
                                width: 900,
                                height: 600,'''

content = content.replace(old_modal_start, new_modal_start)

# 2. 在模态框标题后添加左右分栏结构
# 找到关闭按钮后的 ScrollYView
old_scroll_start = '''                                }

                                <ScrollYView> {
                                    width: Fill,
                                    height: Fill,
                                    scroll_bars: <ScrollBars> {
                                        show_scroll_y: true
                                    }

                                    <View> {
                                        width: Fill,
                                        height: Fit,
                                        flow: Down,
                                        spacing: 15,'''

new_scroll_start = '''                                }

                                // 主要内容区域 - 左右分栏
                                <View> {
                                    width: Fill,
                                    height: Fill,
                                    flow: Right,
                                    spacing: 20,

                                    // 左侧内容区域 (36%)
                                    <ScrollYView> {
                                        width: 324,  // 36% of 900
                                        height: Fill,
                                        scroll_bars: <ScrollBars> {
                                            show_scroll_y: true
                                        }

                                        <View> {
                                            width: Fill,
                                            height: Fit,
                                            flow: Down,
                                            spacing: 15,'''

content = content.replace(old_scroll_start, new_scroll_start)

# 3. 在活动记录 Label 后添加右侧 Active 管理区域
# 找到活动记录部分
active_label_section = '''                                        <View> {
                                            width: Fill,
                                            height: Fit,
                                            flow: Down,
                                            spacing: 5,

                                            <Label> {
                                                width: Fill,
                                                height: Fit,
                                                text: "活动记录"
                                                draw_text: {
                                                    color: #666666
                                                    text_style: {
                                                        font_size: 14.0
                                                    }
                                                }
                                            }

                                            card_active = <Label> {
                                                width: Fill,
                                                height: Fit,
                                                text: "暂无活动记录"
                                                draw_text: {
                                                    color: #333333
                                                    text_style: {
                                                        font_size: 14.0
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
}'''

# 新的结构：移除活动记录 Label，关闭左侧 ScrollYView，添加右侧 Active 管理区域
new_structure = '''                                        }
                                    }

                                    // 右侧 Active 管理区域 (64%)
                                    <ScrollYView> {
                                        width: 556,  // 64% of 900
                                        height: Fill,
                                        scroll_bars: <ScrollBars> {
                                            show_scroll_y: true
                                        }

                                        <View> {
                                            width: Fill,
                                            height: Fit,
                                            flow: Down,
                                            spacing: 15,
                                            padding: {left: 20}

                                            <View> {
                                                width: Fill,
                                                height: Fit,
                                                flow: Down,
                                                spacing: 5,

                                                <Label> {
                                                    width: Fill,
                                                    height: Fit,
                                                    text: "活动记录"
                                                    draw_text: {
                                                        color: #666666
                                                        text_style: {
                                                            font_size: 16.0
                                                            font_weight: 600.0
                                                        }
                                                    }
                                                }

                                                <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Right,
                                                    spacing: 10,
                                                    align: {y: 0.5}

                                                    card_active = <Label> {
                                                        width: Fill,
                                                        height: Fit,
                                                        text: "暂无活动记录"
                                                        draw_text: {
                                                            color: #333333
                                                            text_style: {
                                                                font_size: 14.0
                                                            }
                                                        }
                                                    }

                                                    add_active_button = <Button> {
                                                        width: 80,
                                                        height: 30,
                                                        text: "添加活动"
                                                        draw_bg: {
                                                            color: #28A745
                                                        }
                                                        draw_text: {
                                                            color: #FFFFFF
                                                            text_style: {
                                                                font_size: 12.0
                                                            }
                                                        }
                                                    }
                                                }

                                                active_dropdown = <View> {
                                                    width: Fill,
                                                    height: Fit,
                                                    flow: Down,
                                                    spacing: 5,
                                                    visible: false,

                                                    <Label> {
                                                        width: Fill,
                                                        height: Fit,
                                                        text: "活动记录管理:"
                                                        draw_text: {
                                                            color: #666666
                                                            text_style: {
                                                                font_size: 12.0
                                                            }
                                                        }
                                                    }

                                                    // 现有活动记录列表
                                                    existing_actives = <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Down,
                                                        spacing: 8,

                                                        active_item_1 = <View> {
                                                            width: Fill,
                                                            height: Fit,
                                                            flow: Down,
                                                            spacing: 5,
                                                            padding: 10,
                                                            visible: false,
                                                            draw_bg: {
                                                                color: #F8F9FA
                                                            }

                                                            <View> {
                                                                width: Fill,
                                                                height: Fit,
                                                                flow: Right,
                                                                align: {y: 0.5}

                                                                active_text_1 = <Label> {
                                                                    width: Fill,
                                                                    height: Fit,
                                                                    text: ""
                                                                    draw_text: {
                                                                        color: #333333
                                                                        text_style: {
                                                                            font_size: 14.0
                                                                            font_weight: 600.0
                                                                        }
                                                                    }
                                                                }

                                                                active_delete_1 = <Button> {
                                                                    width: 20,
                                                                    height: 20,
                                                                    text: "×"
                                                                    draw_bg: {
                                                                        color: #FF6B6B
                                                                    }
                                                                    draw_text: {
                                                                        color: #FFFFFF
                                                                        text_style: {
                                                                            font_size: 12.0
                                                                        }
                                                                    }
                                                                }
                                                            }

                                                            active_time_1 = <Label> {
                                                                width: Fill,
                                                                height: Fit,
                                                                text: ""
                                                                draw_text: {
                                                                    color: #6C757D
                                                                    text_style: {
                                                                        font_size: 10.0
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        active_item_2 = <View> {
                                                            width: Fill,
                                                            height: Fit,
                                                            flow: Down,
                                                            spacing: 5,
                                                            padding: 10,
                                                            visible: false,
                                                            draw_bg: {
                                                                color: #F8F9FA
                                                            }

                                                            <View> {
                                                                width: Fill,
                                                                height: Fit,
                                                                flow: Right,
                                                                align: {y: 0.5}

                                                                active_text_2 = <Label> {
                                                                    width: Fill,
                                                                    height: Fit,
                                                                    text: ""
                                                                    draw_text: {
                                                                        color: #333333
                                                                        text_style: {
                                                                            font_size: 14.0
                                                                            font_weight: 600.0
                                                                        }
                                                                    }
                                                                }

                                                                active_delete_2 = <Button> {
                                                                    width: 20,
                                                                    height: 20,
                                                                    text: "×"
                                                                    draw_bg: {
                                                                        color: #FF6B6B
                                                                    }
                                                                    draw_text: {
                                                                        color: #FFFFFF
                                                                        text_style: {
                                                                            font_size: 12.0
                                                                        }
                                                                    }
                                                                }
                                                            }

                                                            active_time_2 = <Label> {
                                                                width: Fill,
                                                                height: Fit,
                                                                text: ""
                                                                draw_text: {
                                                                    color: #6C757D
                                                                    text_style: {
                                                                        font_size: 10.0
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        active_item_3 = <View> {
                                                            width: Fill,
                                                            height: Fit,
                                                            flow: Down,
                                                            spacing: 5,
                                                            padding: 10,
                                                            visible: false,
                                                            draw_bg: {
                                                                color: #F8F9FA
                                                            }

                                                            <View> {
                                                                width: Fill,
                                                                height: Fit,
                                                                flow: Right,
                                                                align: {y: 0.5}

                                                                active_text_3 = <Label> {
                                                                    width: Fill,
                                                                    height: Fit,
                                                                    text: ""
                                                                    draw_text: {
                                                                        color: #333333
                                                                        text_style: {
                                                                            font_size: 14.0
                                                                            font_weight: 600.0
                                                                        }
                                                                    }
                                                                }

                                                                active_delete_3 = <Button> {
                                                                    width: 20,
                                                                    height: 20,
                                                                    text: "×"
                                                                    draw_bg: {
                                                                        color: #FF6B6B
                                                                    }
                                                                    draw_text: {
                                                                        color: #FFFFFF
                                                                        text_style: {
                                                                            font_size: 12.0
                                                                        }
                                                                    }
                                                                }
                                                            }

                                                            active_time_3 = <Label> {
                                                                width: Fill,
                                                                height: Fit,
                                                                text: ""
                                                                draw_text: {
                                                                    color: #6C757D
                                                                    text_style: {
                                                                        font_size: 10.0
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }

                                                    // 新增活动记录区域
                                                    <View> {
                                                        width: Fill,
                                                        height: Fit,
                                                        flow: Down,
                                                        spacing: 5,

                                                        <View> {
                                                            width: Fill,
                                                            height: Fit,
                                                            flow: Right,
                                                            spacing: 10,
                                                            align: {y: 0.5}

                                                            <Label> {
                                                                width: Fill,
                                                                height: Fit,
                                                                text: "新增活动:"
                                                                draw_text: {
                                                                    color: #666666
                                                                    text_style: {
                                                                        font_size: 12.0
                                                                    }
                                                                }
                                                            }

                                                            new_active_button = <Button> {
                                                                width: 60,
                                                                height: 25,
                                                                text: "新增"
                                                                draw_bg: {
                                                                    color: #28A745
                                                                }
                                                                draw_text: {
                                                                    color: #FFFFFF
                                                                    text_style: {
                                                                        font_size: 12.0
                                                                    }
                                                                }
                                                            }
                                                        }

                                                        new_active_input_container = <View> {
                                                            width: Fill,
                                                            height: Fit,
                                                            visible: false,

                                                            new_active_input = <TextInput> {
                                                                width: Fill,
                                                                height: 30,
                                                                text: "",
                                                                draw_text: {
                                                                    color: #333333
                                                                    text_style: {
                                                                        font_size: 12.0
                                                                    }
                                                                }
                                                                draw_bg: {
                                                                    color: #F8F9FA
                                                                }
                                                                draw_cursor: {
                                                                    color: #333333
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
            }
        }
    }
}'''

content = content.replace(active_label_section, new_structure)

# 保存修改后的文件
with open('src/app.rs', 'w', encoding='utf-8') as f:
    f.write(content)

print("✅ 已添加 Active 管理 UI")
print("📝 接下来需要手动添加 Active 相关方法和事件处理")
