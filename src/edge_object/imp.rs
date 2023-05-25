use std::cell::RefCell;

use glib::{ParamSpec, Properties, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use super::EdgeData;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::EdgeObject)]
pub struct EdgeObject {
    #[property(name = "v", get, set, type = u32, member = v)]
    #[property(name = "u", get, set, type = u32, member = u)]
    pub data: RefCell<EdgeData>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for EdgeObject {
    const NAME: &'static str = "EdgeObject";
    type Type = super::EdgeObject;
}

// Trait shared by all GObjects
impl ObjectImpl for EdgeObject {
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
