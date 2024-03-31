use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::globals::{CONTROL, DOCUMENT};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MainMenuButton {
    Arrow,
    Rect,
    Text,
    #[default]
    Select,
}

impl MainMenuButton {
    pub fn as_title(&self) -> &'static str {
        match self {
            MainMenuButton::Arrow => "Arrow",
            MainMenuButton::Rect => "Rectangle",
            MainMenuButton::Select => "Select",
            MainMenuButton::Text => "Text",
        }
    }

    pub fn as_icon(&self) -> &'static str {
        match self {
            MainMenuButton::Arrow => "north_west",
            MainMenuButton::Rect => "check_box_outline_blank",
            MainMenuButton::Select => "arrow_selector_tool",
            MainMenuButton::Text => "match_case",
        }
    }

    pub fn as_id(&self) -> &'static str {
        match self {
            MainMenuButton::Arrow => "cc_button_arrow",
            MainMenuButton::Rect => "cc_button_rect",
            MainMenuButton::Select => "cc_button_select",
            MainMenuButton::Text => "cc_button_text",
        }
    }
}

pub fn setup() -> Result<(), JsValue> {
    let main_buttons = [
        MainMenuButton::Select,
        MainMenuButton::Arrow,
        MainMenuButton::Rect,
        MainMenuButton::Text,
    ];
    DOCUMENT.with(|d| {
        let div = d
            .get_element_by_id("cc_menu_main")
            .expect("cc_menu_main div not found");
        let wrapper = d.create_element("div")?;
        wrapper.set_attribute("id", "cc_menu_top")?;
        wrapper.set_attribute("role", "group")?;
        wrapper.set_class_name("cc_menu_top");
        for (idx, menu_button) in main_buttons.iter().enumerate() {
            let button = d
                .create_element("button")?
                .dyn_into::<web_sys::HtmlButtonElement>()?;
            button.set_attribute("id", menu_button.as_id())?;
            button.set_attribute("class", "cc_nav_button")?;
            button.set_attribute("type", "button")?;
            button.set_attribute("title", menu_button.as_title())?;
            if idx == 0 {
                button.class_list().add_1("cc_nav_left")?;
            }
            if idx == main_buttons.len() - 1 {
                button.class_list().add_1("cc_nav_right")?;
            }
            let i = d.create_element("i")?;
            i.set_attribute("class", "material-symbols-rounded cc_icon")?;
            i.set_inner_html(menu_button.as_icon());
            button.append_child(&i)?;
            let state = *menu_button;
            let closure = Closure::<dyn FnMut()>::new(move || {
                CONTROL.with(|c| {
                    c.borrow_mut().set_button_state(state);
                });
            });
            button.set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();
            wrapper.append_child(&button)?;
        }
        div.append_child(&wrapper)?;
        Ok(())
    })
}

pub fn update(selected: MainMenuButton) -> Result<(), JsValue> {
    DOCUMENT.with(|d| {
        let active_buttons = d.get_elements_by_class_name("cc_nav_active");
        for i in 0..active_buttons.length() {
            let button = active_buttons
                .item(i)
                .expect("Failed to get active button")
                .dyn_into::<web_sys::HtmlButtonElement>()?;
            button.class_list().remove_1("cc_nav_active")?;
        }
        let new_active = d
            .get_element_by_id(selected.as_id())
            .expect("Failed to get new active button")
            .dyn_into::<web_sys::HtmlButtonElement>()?;
        new_active.class_list().add_1("cc_nav_active")?;
        Ok(())
    })
}
