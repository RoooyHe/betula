use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    pub CardModal = <Modal> {
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
                }

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
                                        }
                                    }
                                }
                            }

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
