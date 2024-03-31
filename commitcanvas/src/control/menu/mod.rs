mod main;
use wasm_bindgen::JsValue;

use crate::globals::DOCUMENT;

pub fn setup() -> Result<(), JsValue> {
    let main_buttons = [
        ("cc_button_select", "arrow_selector_tool", "Select"),
        ("cc_button_arrow", "north_west", "Arrow"),
        ("cc_button_rect", "check_box_outline_blank", "Rectangle"),
        ("cc_button_text", "match_case", "Text"),
    ];
    DOCUMENT.with(|d| {
        let div = d
            .get_element_by_id("cc_menu_main")
            .expect("cc_menu_main div not found");
        let wrapper = d.create_element("div")?;
        wrapper.set_attribute("id", "cc_menu_top")?;
        wrapper.set_attribute("role", "group")?;
        wrapper.set_class_name("cc_menu_top");
        for (idx, (id, icon, title)) in main_buttons.iter().enumerate() {
            let button = d.create_element("button")?;
            button.set_attribute("id", id)?;
            button.set_attribute("class", "cc_nav_button")?;
            button.set_attribute("type", "button")?;
            button.set_attribute("title", title)?;
            if idx == 0 {
                button.class_list().add_2("cc_nav_left", "cc_nav_active")?;
            }
            if idx == main_buttons.len() - 1 {
                button.class_list().add_1("cc_nav_right")?;
            }
            let i = d.create_element("i")?;
            i.set_attribute("class", "material-symbols-rounded cc_icon")?;
            i.set_inner_html(icon);
            button.append_child(&i)?;
            wrapper.append_child(&button)?;
        }
        div.append_child(&wrapper)?;
        Ok(())
    })
}
