use gtk::gio;
use gtk::glib::once_cell::sync::OnceCell;
use gtk::glib::{self, ParamSpec, Properties, Value};
use gtk::prelude::ObjectExt;
use gtk::subclass::prelude::*;

#[derive(Properties, Default)]
#[properties(wrapper_type = super::CustomDrawingArea)]
pub struct CustomDrawingArea {
    #[property(get, set)]
    pub vertices: OnceCell<gio::ListStore>,
    #[property(get, set)]
    pub edges: OnceCell<gio::ListStore>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for CustomDrawingArea {
    const NAME: &'static str = "CustomDrawingArea";
    type Type = super::CustomDrawingArea;
    type ParentType = gtk::DrawingArea;
}

impl DrawingAreaImpl for CustomDrawingArea {
    fn resize(&self, width: i32, height: i32) {
        self.parent_resize(width, height)
    }
}

impl WidgetImpl for CustomDrawingArea {
    fn compute_expand(&self, hexpand: &mut bool, vexpand: &mut bool) {
        self.parent_compute_expand(hexpand, vexpand)
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        self.parent_contains(x, y)
    }

    fn direction_changed(&self, previous_direction: gtk::TextDirection) {
        self.parent_direction_changed(previous_direction)
    }

    fn focus(&self, direction_type: gtk::DirectionType) -> bool {
        self.parent_focus(direction_type)
    }

    fn request_mode(&self) -> gtk::SizeRequestMode {
        self.parent_request_mode()
    }

    fn grab_focus(&self) -> bool {
        self.parent_grab_focus()
    }

    fn keynav_failed(&self, direction_type: gtk::DirectionType) -> bool {
        self.parent_keynav_failed(direction_type)
    }

    fn map(&self) {
        self.parent_map()
    }

    fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
        self.parent_measure(orientation, for_size)
    }

    fn mnemonic_activate(&self, group_cycling: bool) -> bool {
        self.parent_mnemonic_activate(group_cycling)
    }

    fn move_focus(&self, direction_type: gtk::DirectionType) {
        self.parent_move_focus(direction_type)
    }

    fn query_tooltip(
        &self,
        x: i32,
        y: i32,
        keyboard_tooltip: bool,
        tooltip: &gtk::Tooltip,
    ) -> bool {
        self.parent_query_tooltip(x, y, keyboard_tooltip, tooltip)
    }

    fn realize(&self) {
        self.parent_realize()
    }

    fn root(&self) {
        self.parent_root()
    }

    fn set_focus_child(&self, child: Option<&gtk::Widget>) {
        self.parent_set_focus_child(child)
    }

    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline)
    }

    fn snapshot(&self, snapshot: &gtk::Snapshot) {
        self.parent_snapshot(snapshot)
    }

    fn state_flags_changed(&self, state_flags: &gtk::StateFlags) {
        self.parent_state_flags_changed(state_flags)
    }

    fn system_setting_changed(&self, settings: &gtk::SystemSetting) {
        self.parent_system_setting_changed(settings)
    }

    fn unmap(&self) {
        self.parent_unmap()
    }

    fn unrealize(&self) {
        self.parent_unrealize()
    }

    fn unroot(&self) {
        self.parent_unroot()
    }
}

impl ObjectImpl for CustomDrawingArea {
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
