use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use commitcanvas::model::{Guid, ShapeConfig};

use crate::globals::{CONTROL, DOCUMENT, SVG_CONTROL_GROUP};
use crate::utils::to_error;
use commitcanvas::control::ModificationType;

use commitcanvas::control::selection::Selection as SelectionInterface;

pub struct Nodes {
    pub node1: web_sys::SvgElement,
    pub node2: web_sys::SvgElement,
    pub node3: web_sys::SvgElement,
    pub node4: web_sys::SvgElement,
    pub node5: web_sys::SvgElement,
    pub node6: web_sys::SvgElement,
    pub node7: web_sys::SvgElement,
    pub node8: web_sys::SvgElement,
    #[allow(dead_code)]
    closure1: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure2: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure3: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure4: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure5: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure6: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure7: Closure<dyn Fn(web_sys::MouseEvent)>,
    #[allow(dead_code)]
    closure8: Closure<dyn Fn(web_sys::MouseEvent)>,
    // TODO add node to center
}

impl Nodes {
    pub fn new(guid: Guid, config: &ShapeConfig) -> Result<Self, JsValue> {
        let node1 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node1.set_attribute("class", "cc_selection_node cc_selection_node_tl")?;
        node1.set_attribute("r", "5")?;
        let closure1 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::TL);
                });
            });
        node1.set_onmousedown(Some(closure1.as_ref().unchecked_ref()));

        let node2 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node2.set_attribute("class", "cc_selection_node cc_selection_node_tr")?;
        node2.set_attribute("r", "5")?;
        let closure2 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::TR);
                });
            });
        node2.set_onmousedown(Some(closure2.as_ref().unchecked_ref()));

        let node3 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node3.set_attribute("class", "cc_selection_node cc_selection_node_br")?;
        node3.set_attribute("r", "5")?;
        let closure3 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::BR);
                });
            });
        node3.set_onmousedown(Some(closure3.as_ref().unchecked_ref()));

        let node4 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node4.set_attribute("class", "cc_selection_node cc_selection_node_bl")?;
        node4.set_attribute("r", "5")?;
        let closure4 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::BL);
                });
            });
        node4.set_onmousedown(Some(closure4.as_ref().unchecked_ref()));

        let node5 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node5.set_attribute("class", "cc_selection_node cc_selection_node_t")?;
        node5.set_attribute("r", "5")?;
        let closure5 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::T);
                });
            });
        node5.set_onmousedown(Some(closure5.as_ref().unchecked_ref()));

        let node6 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node6.set_attribute("class", "cc_selection_node cc_selection_node_r")?;
        node6.set_attribute("r", "5")?;
        let closure6 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::R);
                });
            });
        node6.set_onmousedown(Some(closure6.as_ref().unchecked_ref()));

        let node7 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node7.set_attribute("class", "cc_selection_node cc_selection_node_b")?;
        node7.set_attribute("r", "5")?;
        let closure7 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::B);
                });
            });
        node7.set_onmousedown(Some(closure7.as_ref().unchecked_ref()));

        let node8 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node8.set_attribute("class", "cc_selection_node cc_selection_node_l")?;
        node8.set_attribute("r", "5")?;
        let closure8 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, ModificationType::L);
                });
            });
        node8.set_onmousedown(Some(closure8.as_ref().unchecked_ref()));

        SVG_CONTROL_GROUP.with(|g| {
            g.append_child(&node1)?;
            g.append_child(&node2)?;
            g.append_child(&node3)?;
            g.append_child(&node4)?;
            g.append_child(&node5)?;
            g.append_child(&node6)?;
            g.append_child(&node7)?;
            g.append_child(&node8)
        })?;

        let mut result = Self {
            node1,
            node2,
            node3,
            node4,
            node5,
            node6,
            node7,
            node8,
            closure1,
            closure2,
            closure3,
            closure4,
            closure5,
            closure6,
            closure7,
            closure8,
        };
        result.update(config)?;
        Ok(result)
    }

    pub fn update(&mut self, config: &ShapeConfig) -> Result<(), JsValue> {
        self.node1
            .set_attribute("cx", config.start.x.to_string().as_str())?;
        self.node1
            .set_attribute("cy", config.start.y.to_string().as_str())?;
        self.node2
            .set_attribute("cx", config.end.x.to_string().as_str())?;
        self.node2
            .set_attribute("cy", config.start.y.to_string().as_str())?;
        self.node3
            .set_attribute("cx", config.end.x.to_string().as_str())?;
        self.node3
            .set_attribute("cy", config.end.y.to_string().as_str())?;
        self.node4
            .set_attribute("cx", config.start.x.to_string().as_str())?;
        self.node4
            .set_attribute("cy", config.end.y.to_string().as_str())?;
        self.node5.set_attribute(
            "cx",
            ((config.start.x + config.end.x) / 2.0).to_string().as_str(),
        )?;
        self.node5
            .set_attribute("cy", config.start.y.to_string().as_str())?;
        self.node6
            .set_attribute("cx", config.end.x.to_string().as_str())?;
        self.node6.set_attribute(
            "cy",
            ((config.start.y + config.end.y) / 2.0).to_string().as_str(),
        )?;
        self.node7.set_attribute(
            "cx",
            ((config.start.x + config.end.x) / 2.0).to_string().as_str(),
        )?;
        self.node7
            .set_attribute("cy", config.end.y.to_string().as_str())?;
        self.node8
            .set_attribute("cx", config.start.x.to_string().as_str())?;
        self.node8.set_attribute(
            "cy",
            ((config.start.y + config.end.y) / 2.0).to_string().as_str(),
        )?;
        Ok(())
    }
}

impl Drop for Nodes {
    fn drop(&mut self) {
        self.node1.remove();
        self.node2.remove();
        self.node3.remove();
        self.node4.remove();
        self.node5.remove();
        self.node6.remove();
        self.node7.remove();
        self.node8.remove();
    }
}

pub struct Selection {
    #[allow(dead_code)]
    pub selected: Guid,
    pub path: web_sys::SvgElement,
    pub nodes: Nodes,
}

impl Drop for Selection {
    fn drop(&mut self) {
        self.path.remove();
    }
}

impl SelectionInterface for Selection {
    fn new(
        guid: Guid,
        config: &ShapeConfig,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let path = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "path"))
            .map_err(to_error)?;
        path.set_id("cc_selection_rect");
        path.set_attribute("class", "cc_selection_rect")
            .map_err(to_error)?;

        SVG_CONTROL_GROUP
            .with(|g| g.append_child(&path))
            .map_err(to_error)?;
        let nodes = Nodes::new(guid, config).map_err(to_error)?;

        let mut result = Self {
            selected: guid,
            path: path
                .dyn_into::<web_sys::SvgElement>()
                .map_err(|e| to_error(e.into()))?,
            nodes,
        };

        result.update(config)?;

        Ok(result)
    }

    fn update(
        &mut self,
        config: &ShapeConfig,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const EXTRA: f32 = 4096.0;

        let min_x = config.start.x.min(config.end.x);
        let min_y = config.start.y.min(config.end.y);
        let max_x = config.start.x.max(config.end.x);
        let max_y = config.start.y.max(config.end.y);

        let d = format!(
            "M {} {} L {} {} M {} {} L {} {} M {} {} L {} {} M {} {} L {} {}",
            min_x - EXTRA,
            min_y,
            max_x + EXTRA,
            min_y,
            min_x,
            min_y - EXTRA,
            min_x,
            max_y + EXTRA,
            max_x,
            min_y - EXTRA,
            max_x,
            max_y + EXTRA,
            min_x - EXTRA,
            max_y,
            max_x + EXTRA,
            max_y,
        );
        self.path.set_attribute("d", &d).map_err(to_error)?;
        self.nodes.update(config).map_err(to_error)?;

        Ok(())
    }
}
