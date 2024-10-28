use std::error::Error;

use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::{
    globals::{CONTROL, DOCUMENT},
    utils::to_error,
};
use commitcanvas::control::menu::MainMenuButton;

fn button_to_title(button: &MainMenuButton) -> &'static str {
    match button {
        MainMenuButton::Arrow => "Arrow",
        MainMenuButton::Rect => "Rectangle",
        MainMenuButton::Select => "Select",
        MainMenuButton::Text => "Text",
    }
}

fn button_to_icon(button: &MainMenuButton) -> &'static str {
    match button {
        MainMenuButton::Arrow => "north_west",
        MainMenuButton::Rect => "check_box_outline_blank",
        MainMenuButton::Select => "arrow_selector_tool",
        MainMenuButton::Text => "match_case",
    }
}

fn button_to_id(button: &MainMenuButton) -> &'static str {
    match button {
        MainMenuButton::Arrow => "cc_button_arrow",
        MainMenuButton::Rect => "cc_button_rect",
        MainMenuButton::Select => "cc_button_select",
        MainMenuButton::Text => "cc_button_text",
    }
}

pub fn setup() -> Result<(), JsValue> {
    log::info!("setting up main menu");
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
            button.set_attribute("id", button_to_id(menu_button))?;
            button.set_attribute("class", "cc_nav_button")?;
            button.set_attribute("type", "button")?;
            button.set_attribute("title", button_to_title(menu_button))?;
            if idx == 0 {
                button.class_list().add_1("cc_nav_left")?;
            }
            if idx == main_buttons.len() - 1 {
                button.class_list().add_1("cc_nav_right")?;
            }
            let i = d.create_element("i")?;
            i.set_attribute("class", "material-symbols-rounded cc_icon")?;
            i.set_inner_html(button_to_icon(menu_button));
            button.append_child(&i)?;
            let state = *menu_button;
            let closure = Closure::<dyn Fn()>::new(move || {
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

pub fn update(selected: MainMenuButton) -> Result<(), Box<dyn Error + Send + Sync>> {
    DOCUMENT.with(|d| {
        let active_buttons = d.get_elements_by_class_name("cc_nav_active");
        for i in 0..active_buttons.length() {
            let button = active_buttons
                .item(i)
                .expect("Failed to get active button")
                .dyn_into::<web_sys::HtmlButtonElement>()
                .map_err(|e| to_error(e.into()))?;
            button
                .class_list()
                .remove_1("cc_nav_active")
                .map_err(to_error)?;
        }
        let new_active = d
            .get_element_by_id(button_to_id(&selected))
            .expect("Failed to get new active button")
            .dyn_into::<web_sys::HtmlButtonElement>()
            .map_err(|e| to_error(e.into()))?;
        new_active
            .class_list()
            .add_1("cc_nav_active")
            .map_err(to_error)?;
        Ok(())
    })
}
