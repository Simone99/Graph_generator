mod imp;

use glib::Object;
use gtk::glib;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct EdgeObject(ObjectSubclass<imp::EdgeObject>);
}

impl EdgeObject {
    pub fn new(v: u32, u: u32) -> Self {
        Object::builder().property("v", v).property("u", u).build()
    }

    pub fn edge_data(&self) -> EdgeData {
        self.imp().data.borrow().clone()
    }

    pub fn from_edge_data(edge_data: EdgeData) -> Self {
        Self::new(edge_data.v, edge_data.u)
    }
}

#[derive(Default, Clone)]
pub struct EdgeData {
    pub v: u32,
    pub u: u32,
}
