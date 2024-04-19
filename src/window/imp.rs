use std::cell::Cell;
use std::f64::consts::PI;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::Path;

use glib::subclass::InitializingObject;
use gtk::gio::Settings;
use gtk::glib::{clone, BindingFlags, Propagation};
use gtk::prelude::{
    ButtonExt, DrawingAreaExtManual, GestureDragExt, ObjectExt, ToValue, WidgetExt,
};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Button, CompositeTemplate, GestureDrag};
use std::cell::OnceCell;

use crate::drawing::CustomDrawingArea;
use crate::edge_object::{EdgeData, EdgeObject};
use crate::node_object::{NodeData, NodeObject};

const OUTPUT_FILE_NAME: &'static str = "graph.txt";

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/simomaster1/GraphGenerator/window.ui")]
pub struct Window {
    #[template_child]
    pub drawing_area: TemplateChild<CustomDrawingArea>,
    #[template_child]
    pub save_button: TemplateChild<Button>,
    pub settings: OnceCell<Settings>,
    pub vertex_counter: Cell<u32>,
    pub drag_start_x: Cell<f64>,
    pub drag_start_y: Cell<f64>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "GraphGeneratorWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        // Call "constructed" on parent
        self.parent_constructed();

        // Setup
        let tmp: &CustomDrawingArea = self.drawing_area.as_ref();
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
        obj.bind_property("default-width", tmp, "content-width")
            .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
            .build();
        obj.bind_property("default-height", tmp, "content-height")
            .transform_to(move |_, height: i32| {
                let edited_value = height - 71;
                Some(edited_value.to_value())
            })
            .flags(BindingFlags::BIDIRECTIONAL | BindingFlags::SYNC_CREATE)
            .build();

        self.drawing_area
            .set_vertices(gio::ListStore::new::<NodeObject>());
        self.drawing_area
            .set_edges(gio::ListStore::new::<EdgeObject>());
        self.vertex_counter.set(0);
        self.drawing_area.set_draw_func(clone!(@weak self as win => move |_area, context, _width, _height|{

            // Detect the current theme
            let mode = dark_light::detect();

            // Draw edges
            let edge_collection = win.drawing_area.to_collection_data().edges_data;
            let vertex_collection = win.drawing_area.to_collection_data().vertices_data;
            for edge in &edge_collection {
                let node = vertex_collection.clone().into_iter().find(|node| node.index == edge.u).unwrap();
                let node1 = vertex_collection.clone().into_iter().find(|node| node.index == edge.v).unwrap();
                context.set_line_width(7.5);
                match mode {
                    // Dark mode
                    dark_light::Mode::Dark => {context.set_source_rgb(255.0, 255.0, 255.0)},
                    // Light mode
                    dark_light::Mode::Light => {context.set_source_rgb(0.0, 0.0, 0.0)},
                    // Unspecified
                    dark_light::Mode::Default => {context.set_source_rgb(255.0, 255.0, 255.0)},
                }
                context.move_to(node.x, node.y);
                context.line_to(node1.x, node1.y);
                context.stroke().expect("Unable to draw!");
            }
            // Draw vertices
            for vertex in &vertex_collection {
                context.arc(vertex.x, vertex.y, 25.0, 0.0, 2.0 * PI);
                context.set_source_rgb(0.0, 0.0, 0.0);
                context.fill().expect("Unable to draw!");
                context.arc(vertex.x, vertex.y, 22.0, 0.0, 2.0 * PI);
                context.set_source_rgb(255.0, 255.0, 255.0);
                context.fill().expect("Unable to draw!");
                context.set_source_rgb(0.0, 0.0, 0.0);
                context.select_font_face("Sans", gtk::cairo::FontSlant::Normal, gtk::cairo::FontWeight::Normal);
                context.set_font_size(22.0);
                let extents = context.text_extents(vertex.index.to_string().as_str()).unwrap();
                let x = vertex.x - (extents.width() / 2.0 + extents.x_bearing());
                let y = vertex.y - (extents.height() / 2.0 + extents.y_bearing());
                context.move_to(x, y);
                context.show_text(vertex.index.to_string().as_str()).expect("Unable to show text!");    
            }
        }));

        let gesture_controller = GestureDrag::builder().build();
        gesture_controller.connect_drag_begin(
            clone!(@weak self as win => move |_controller, x, y| {
                win.drag_start_x.set(x);
                win.drag_start_y.set(y);
            }),
        );

        gesture_controller.connect_drag_end(clone!(@weak self as win => move |_controller, x, y| {
            // Check whether previous vertices have been already created, so probably the user wanted to create an edge
            let x_start = win.drag_start_x.get();
            let y_start = win.drag_start_y.get();
            let x_end = x_start + x;
            let y_end = y_start + y;
            let mut vertex_exists = false;
            let nodes = win.drawing_area.to_collection_data().vertices_data;
            for node in &nodes {
                if node.x - 25.0 <= x_start && x_start <= node.x + 25.0 && node.y - 25.0 <= y_start && y_start <= node.y + 25.0 {
                    // The user probably want to create an edge, check if it released the drag on another vertex
                    vertex_exists = true;
                    for node1 in &nodes {
                        if node.index != node1.index && node1.x - 25.0 <= x_end && x_end <= node1.x + 25.0 && node1.y - 25.0 <= y_end && y_end <= node1.y + 25.0 {
                            win.drawing_area.edges().extend_from_slice(&[EdgeObject::from_edge_data(EdgeData{v: node.index, u: node1.index})]);
                            win.drawing_area.queue_draw();
                            return;
                        }
                    }
                }
            }
            if !vertex_exists {
                win.drawing_area.vertices().extend_from_slice(&[NodeObject::from_node_data(NodeData { index: win.vertex_counter.get(), x: x_start, y: y_start })]);
                win.vertex_counter.set(win.vertex_counter.get() + 1);
                win.drawing_area.queue_draw();
            }
        }));

        self.drawing_area.set_cursor_from_name(Some("pointer"));
        self.drawing_area.add_controller(gesture_controller);
        self.save_button.connect_clicked(clone!(@weak self as win => move |_| {
            if Path::new(OUTPUT_FILE_NAME).exists() {
                remove_file(OUTPUT_FILE_NAME).expect("Error deleting the output file!");
            }
            let mut output_file = File::create(OUTPUT_FILE_NAME).expect("Error creating the output file!");
            let mut collection = win.drawing_area.to_collection_data();
            let vertices: Vec<u32> = collection.vertices_data.into_iter().map(|node| node.index).collect();
            collection.edges_data.sort_by(|e, e1| e.v.cmp(&e1.v));
            writeln!(output_file, "{}\n{}", vertices.len(), collection.edges_data.len()).expect("Error writing to output file!");
            for vertex in vertices {
                writeln!(output_file, "{}", vertex).expect("Error writing to output file!");
            }
            for edge in collection.edges_data {
                writeln!(output_file, "{} {}", edge.v, edge.u).expect("Error writing to output file!");
            }
        }));
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Propagation {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        self.parent_close_request()
    }
}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
