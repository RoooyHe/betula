use crate::components::image_item::ImageClickedAction;
use makepad_widgets::*;
use std::path::{Path, PathBuf};

live_design! {
    use link::widgets::*;
    use link::theme::*;
    use crate::components::image_grid::ImageGrid;

    PLACEHOLDER = dep("crate://self/resources/placeholder.png");
    LEFT_ARROW = dep("crate://self/resources/left_arrow.svg");
    RIGHT_ARROW = dep("crate://self/resources/right_arrow.svg");
    LOOKING_GLASS = dep("crate://self/resources/looking_glass.svg");

    AlertDialog = <Modal> {
        width: Fill
        height: Fill

        bg_view: <View> {
            width: Fill
            height: Fill
            show_bg: true
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return #00000080;
                }
            }
        }

        content: <View> {
            width: Fit
            height: Fit
            padding: 20
            flow: Down
            align: {x: 0.5, y: 0.5}

            draw_bg: {
                color: #333
            }

            dialog = <RoundedView> {
                width: 300
                height: 150
                align: {x: 0.5, y: 0.5}
                draw_bg: {
                    color: #333
                    border_color: #555
                    border_size: 1.0
                    border_radius: 4.0
                }
                padding: 20


                message = <Label> {
                    width: Fill
                    height: Fit
                    align: {x: 0.5}
                    margin: {bottom: 20}
                    draw_text: {
                        text_style: { font_size: 12.0 }
                        color: #fff
                    }
                    text: "默认消息"
                }
            }
        }
    }

    SearchBox = <View> {
        width: 150,
        height: Fit,
        align: { y: 0.5 }
        margin: { left: 75 }

        <Icon> {
            icon_walk: { width: 12.0 }
            draw_icon: {
                color: #8,
                svg_file: (LOOKING_GLASS)
            }
        }

        query = <TextInput> {
            empty_text: "Search",
            draw_text: {
                text_style: { font_size: 10 },
                color: #8
            }
        }
    }

    MenuBar = <View> {
        width: Fill,
        height: Fit,

        <SearchBox> {}
        <Filler> {}
        slideshow_button = <Button> {
            text: "Slideshow"
        }
    }

    ImageBrowser = <View> {
        flow: Down,

        <MenuBar> {}
        <ImageGrid> {}
    }

    SlideshowNavigateButton = <Button> {
        width: 50,
        height: Fill,
        draw_bg: {
            color: #fff0,
            color_down: #fff2,
        }
        icon_walk: { width: 9 },
        text: "",
        grab_key_focus: false,
    }

    SlideshowOverlay = <View> {
        height: Fill,
        width: Fill,
        cursor: Arrow,
        capture_overload: true,

        navigate_left = <SlideshowNavigateButton> {
            draw_icon: { svg_file: (LEFT_ARROW) }
        }
        <Filler> {}
        navigate_right = <SlideshowNavigateButton> {
            draw_icon: { svg_file: (RIGHT_ARROW) }
        }
    }

    Slideshow = <View> {
        flow: Overlay,

        image = <Image> {
            width: Fill,
            height: Fill,
            fit: Biggest,
            source: (PLACEHOLDER)
        }
        overlay = <SlideshowOverlay> {}
    }

    App = {{App}} {
        placeholder: (PLACEHOLDER),

        ui: <Root> {
            <Window> {
                body = <View> {
                    flow: Overlay,

                    page_flip = <PageFlip> {
                        active_page: image_browser,

                        image_browser = <ImageBrowser> {}
                        slideshow = <Slideshow> {}
                    }

                    alert_dialog = <AlertDialog> {}
                }
            }
        }
    }
}

#[derive(Live)]
pub struct App {
    #[live]
    placeholder: LiveDependency,
    #[live]
    ui: WidgetRef,
    #[rust]
    state: State,
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui
            .handle_event(cx, event, &mut Scope::with_data(&mut self.state));
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::components::live_design(cx);
    }
}

impl App {
    fn load_image_paths(&mut self, cx: &mut Cx, path: &Path) {
        self.state.image_paths.clear();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            self.state.image_paths.push(path);
        }

        let query = self.ui.text_input(id!(query)).text();
        self.filter_image_paths(cx, &query);
    }

    fn set_current_image(&mut self, cx: &mut Cx, image_idx: Option<usize>) {
        self.state.current_image_idx = image_idx;

        let image = self.ui.image(id!(slideshow.image));
        if let Some(image_idx) = self.state.current_image_idx {
            let filtered_image_idx = self.state.filtered_image_idxs[image_idx];
            let image_path = &self.state.image_paths[filtered_image_idx];
            image
                .load_image_file_by_path_async(cx, &image_path)
                .unwrap();
        } else {
            image
                .load_image_dep_by_path(cx, self.placeholder.as_str())
                .unwrap();
        }
        self.ui.redraw(cx);
    }

    pub fn navigate_left(&mut self, cx: &mut Cx) {
        if let Some(image_idx) = self.state.current_image_idx {
            if image_idx > 0 {
                self.set_current_image(cx, Some(image_idx - 1));
            } else {
                self.show_alert(cx, "already is first img")
            }
        }
    }

    pub fn navigate_right(&mut self, cx: &mut Cx) {
        if let Some(image_idx) = Some(self.state.num_images()) {
            if image_idx + 1 < self.state.image_paths.len() {
                self.set_current_image(cx, Some(image_idx + 1));
            } else {
                self.show_alert(cx, "already is last img");
            }
        }
    }

    pub fn filter_image_paths(&mut self, cx: &mut Cx, query: &str) {
        self.state.filtered_image_idxs.clear();
        for (image_idx, image_path) in self.state.image_paths.iter().enumerate() {
            if image_path.to_str().unwrap().contains(&query) {
                self.state.filtered_image_idxs.push(image_idx);
            }
        }
        if self.state.filtered_image_idxs.is_empty() {
            self.set_current_image(cx, None);
        } else {
            self.set_current_image(cx, Some(0));
        }
    }

    fn show_alert(&mut self, cx: &mut Cx, message: &str) {
        self.state.show_alert = true;
        self.state.alert_message = message.to_string();
        let modal_ref = self.ui.modal(id!(alert_dialog));
        let message_lable = modal_ref.label(id!(message));
        message_lable.set_text(cx, message);
        modal_ref.open(cx);
    }

    fn close_alert(&mut self, cx: &mut Cx) {
        self.state.show_alert = false;
        self.ui.modal(id!(alert_dialog)).close(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        let path = "C:\\Users\\Roy\\RustroverProjects\\betula\\resources";
        self.load_image_paths(_cx, path.as_ref());
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self.ui.button(id!(slideshow_button)).clicked(&actions) {
            self.ui
                .page_flip(id!(page_filp))
                .set_active_page(cx, live_id!(slideshow));
            self.ui.view(id!(slideshow.overlay)).set_key_focus(cx);
        }

        if self.ui.button(id!(navigate_left)).clicked(&actions) {
            self.navigate_left(cx);
        }
        if self.ui.button(id!(navigate_right)).clicked(&actions) {
            self.navigate_right(cx);
        }
        if let Some(query) = self.ui.text_input(id!(query)).changed(&actions) {
            self.filter_image_paths(cx, &query);
        }
        if self.ui.modal(id!(alert_dialog)).dismissed(actions) {
            self.close_alert(cx);
        }

        if let Some(event) = self.ui.view(id!(slideshow.overlay)).key_down(&actions) {
            match event.key_code {
                KeyCode::Escape => self
                    .ui
                    .page_flip(id!(page_flip))
                    .set_active_page(cx, live_id!(image_browser)),
                KeyCode::ArrowLeft => self.navigate_left(cx),
                KeyCode::ArrowRight => self.navigate_right(cx),
                _ => {}
            }
        }

        for action in actions {
            if let ImageClickedAction::Clicked(image_idx) = action.cast() {
                self.set_current_image(cx, Some(image_idx));
                self.ui
                    .page_flip(id!(page_flip))
                    .set_active_page(cx, live_id!(slideshow));
                self.ui.view(id!(slideshow.overlay)).set_key_focus(cx);
            }
        }
    }
}
#[derive(Live, LiveHook, Widget)]
pub struct ImageRow {
    #[deref]
    view: View,
}

impl Widget for ImageRow {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {
        self.view.handle_event(_cx, _event, _scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().unwrap();
                let row_idx = *scope.props.get::<usize>().unwrap();

                list.set_item_range(cx, 0, state.num_images_for_row(row_idx));
                while let Some(item_idx) = list.next_visible_item(cx) {
                    if item_idx >= state.num_images_for_row(row_idx) {
                        continue;
                    }

                    let item = list.item(cx, item_idx, live_id!(ImageItem));
                    let image_idx = state.first_image_idx_for_row(row_idx) + item_idx;
                    let filtered_image_idx = state.filtered_image_idxs[image_idx];
                    let image_path = &state.image_paths[filtered_image_idx];
                    let image = item.image(id!(image));
                    image
                        .load_image_file_by_path_async(cx, &image_path)
                        .unwrap();
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ImageGrid {
    #[deref]
    view: View,
}

impl Widget for ImageGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.view.handle_event(cx, event, &mut Scope::empty());
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                let state = scope.data.get_mut::<State>().unwrap();

                list.set_item_range(cx, 0, state.num_rows());
                while let Some(row_idx) = list.next_visible_item(cx) {
                    if row_idx >= state.num_rows() {
                        continue;
                    }

                    let row = list.item(cx, row_idx, live_id!(ImageRow));
                    let mut scope = Scope::with_data_props(state, &row_idx);
                    row.draw_all(cx, &mut scope);
                }
            }
        }
        DrawStep::done()
    }
}

#[derive(Debug)]
pub struct State {
    pub(crate) image_paths: Vec<PathBuf>,
    pub(crate) filtered_image_idxs: Vec<usize>,
    max_images_per_row: usize,
    current_image_idx: Option<usize>,
    show_alert: bool,
    alert_message: String,
}

impl Default for State {
    fn default() -> Self {
        Self {
            image_paths: Vec::new(),
            filtered_image_idxs: Vec::new(),
            max_images_per_row: 4,
            current_image_idx: None,
            show_alert: false,
            alert_message: String::new(),
        }
    }
}

impl State {
    pub fn num_images(&self) -> usize {
        self.filtered_image_idxs.len()
    }

    pub fn num_rows(&self) -> usize {
        self.num_images().div_ceil(self.max_images_per_row)
    }

    pub fn first_image_idx_for_row(&self, row_idx: usize) -> usize {
        row_idx * self.max_images_per_row
    }

    pub fn num_images_for_row(&self, row_idx: usize) -> usize {
        let first_image_idx = self.first_image_idx_for_row(row_idx);
        let num_remaining_images = self.num_images() - first_image_idx;
        self.max_images_per_row.min(num_remaining_images)
    }
}

app_main!(App);
