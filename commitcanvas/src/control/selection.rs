use wasm_bindgen::{closure::Closure, JsCast, JsValue};

use crate::{
    globals::{CONTROL, DOCUMENT, SVG_CONTROL_GROUP},
    model::{Guid, Shape},
};

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
    pub fn new(shape: &Shape) -> Result<Self, JsValue> {
        let guid = shape.guid;
        let node1 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node1.set_attribute("class", "cc_selection_node")?;
        node1.set_attribute("r", "5")?;
        let closure1 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::TL);
                });
            });
        node1.set_onmousedown(Some(closure1.as_ref().unchecked_ref()));

        let node2 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node2.set_attribute("class", "cc_selection_node")?;
        node2.set_attribute("r", "5")?;
        let closure2 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::TR);
                });
            });
        node2.set_onmousedown(Some(closure2.as_ref().unchecked_ref()));

        let node3 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node3.set_attribute("class", "cc_selection_node")?;
        node3.set_attribute("r", "5")?;
        let closure3 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::BR);
                });
            });
        node3.set_onmousedown(Some(closure3.as_ref().unchecked_ref()));

        let node4 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node4.set_attribute("class", "cc_selection_node")?;
        node4.set_attribute("r", "5")?;
        let closure4 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::BL);
                });
            });
        node4.set_onmousedown(Some(closure4.as_ref().unchecked_ref()));

        let node5 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node5.set_attribute("class", "cc_selection_node")?;
        node5.set_attribute("r", "5")?;
        let closure5 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::T);
                });
            });
        node5.set_onmousedown(Some(closure5.as_ref().unchecked_ref()));

        let node6 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node6.set_attribute("class", "cc_selection_node")?;
        node6.set_attribute("r", "5")?;
        let closure6 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::R);
                });
            });
        node6.set_onmousedown(Some(closure6.as_ref().unchecked_ref()));

        let node7 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node7.set_attribute("class", "cc_selection_node")?;
        node7.set_attribute("r", "5")?;
        let closure7 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::B);
                });
            });
        node7.set_onmousedown(Some(closure7.as_ref().unchecked_ref()));

        let node8 = DOCUMENT
            .with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "circle"))?
            .dyn_into::<web_sys::SvgElement>()?;
        node8.set_attribute("class", "cc_selection_node")?;
        node8.set_attribute("r", "5")?;
        let closure8 =
            Closure::<dyn Fn(web_sys::MouseEvent)>::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                CONTROL.with(|control| {
                    let mut c = control.borrow_mut();
                    c.modify(guid, super::ModificationType::L);
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
        result.update(shape)?;
        Ok(result)
    }

    pub fn update(&mut self, shape: &Shape) -> Result<(), JsValue> {
        self.node1
            .set_attribute("cx", shape.start.x.to_string().as_str())?;
        self.node1
            .set_attribute("cy", shape.start.y.to_string().as_str())?;
        self.node2
            .set_attribute("cx", shape.end.x.to_string().as_str())?;
        self.node2
            .set_attribute("cy", shape.start.y.to_string().as_str())?;
        self.node3
            .set_attribute("cx", shape.end.x.to_string().as_str())?;
        self.node3
            .set_attribute("cy", shape.end.y.to_string().as_str())?;
        self.node4
            .set_attribute("cx", shape.start.x.to_string().as_str())?;
        self.node4
            .set_attribute("cy", shape.end.y.to_string().as_str())?;
        self.node5.set_attribute(
            "cx",
            ((shape.start.x + shape.end.x) / 2.0).to_string().as_str(),
        )?;
        self.node5
            .set_attribute("cy", shape.start.y.to_string().as_str())?;
        self.node6
            .set_attribute("cx", shape.end.x.to_string().as_str())?;
        self.node6.set_attribute(
            "cy",
            ((shape.start.y + shape.end.y) / 2.0).to_string().as_str(),
        )?;
        self.node7.set_attribute(
            "cx",
            ((shape.start.x + shape.end.x) / 2.0).to_string().as_str(),
        )?;
        self.node7
            .set_attribute("cy", shape.end.y.to_string().as_str())?;
        self.node8
            .set_attribute("cx", shape.start.x.to_string().as_str())?;
        self.node8.set_attribute(
            "cy",
            ((shape.start.y + shape.end.y) / 2.0).to_string().as_str(),
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
    pub selected: Guid,
    pub path: web_sys::SvgElement,
    pub nodes: Nodes,
}

impl Drop for Selection {
    fn drop(&mut self) {
        self.path.remove();
    }
}

impl Selection {
    pub fn new(shape: &Shape) -> Result<Self, JsValue> {
        let path =
            DOCUMENT.with(|d| d.create_element_ns(Some("http://www.w3.org/2000/svg"), "path"))?;
        path.set_id("cc_selection_rect");
        path.set_attribute("class", "cc_selection_rect")?;

        SVG_CONTROL_GROUP.with(|g| g.append_child(&path))?;
        let nodes = Nodes::new(shape)?;

        let mut result = Self {
            selected: shape.guid,
            path: path.dyn_into::<web_sys::SvgElement>()?,
            nodes,
        };

        result.update(shape)?;

        Ok(result)
    }

    pub fn update(&mut self, shape: &Shape) -> Result<(), JsValue> {
        const EXTRA: f32 = 4096.0;

        let min_x = shape.start.x.min(shape.end.x);
        let min_y = shape.start.y.min(shape.end.y);
        let max_x = shape.start.x.max(shape.end.x);
        let max_y = shape.start.y.max(shape.end.y);

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
        self.path.set_attribute("d", &d)?;
        self.nodes.update(shape)?;

        Ok(())
    }
}
