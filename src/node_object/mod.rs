mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct NodeObject(ObjectSubclass<imp::NodeObject>);
}

impl NodeObject {
    pub fn new(index: u32, x: f64, y: f64) -> Self {
        Object::builder()
            .property("index", index)
            .property("x", x)
            .property("y", y)
            .build()
    }

    pub fn node_data(&self) -> NodeData {
        self.imp().data.borrow().clone()
    }

    pub fn from_node_data(node_data: NodeData) -> Self {
        Self::new(node_data.index, node_data.x, node_data.y)
    }
}

#[derive(Default, Clone)]
pub struct NodeData {
    pub index: u32,
    pub x: f64,
    pub y: f64,
}
