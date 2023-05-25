use std::cell::RefCell;

use glib::{ParamSpec, Properties, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::NodeData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::NodeObject)]
pub struct NodeObject {
    #[property(name = "index", get, set, type = u32, member = index)]
    #[property(name = "x", get, set, type = f64, member = x)]
    #[property(name = "y", get, set, type = f64, member = y)]
    pub data: RefCell<NodeData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for NodeObject {
    const NAME: &'static str = "NodeObject";
    type Type = super::NodeObject;
}

// Trait shared by all GObjects
impl ObjectImpl for NodeObject {
    fn properties() -> &'static [ParamSpec] {
        Self::derived_properties()
    }

    fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
        self.derived_set_property(id, value, pspec)
    }

    fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
        self.derived_property(id, pspec)
    }
}
