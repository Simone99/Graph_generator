mod imp;

use glib::Object;
use gtk::{
    gio, glib,
    prelude::{Cast, ListModelExtManual, StaticType},
};

use crate::{
    edge_object::{EdgeData, EdgeObject},
    node_object::{NodeData, NodeObject},
};

glib::wrapper! {
    pub struct CustomDrawingArea(ObjectSubclass<imp::CustomDrawingArea>)
    @extends gtk::DrawingArea, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[derive(Default, Clone)]
pub struct CollectionData {
    pub vertices_data: Vec<NodeData>,
    pub edges_data: Vec<EdgeData>,
}

impl CustomDrawingArea {
    pub fn new(vertices: gio::ListStore, edges: gio::ListStore) -> Self {
        Object::builder()
            .property("vertices", vertices)
            .property("edges", edges)
            .build()
    }

    pub fn to_collection_data(&self) -> CollectionData {
        let vertex_data = self
            .vertices()
            .snapshot()
            .iter()
            // TODO: create a GObject containing just a node like TaskObject, so that I can downcast and map it to the original structure
            .filter_map(Cast::downcast_ref::<NodeObject>)
            .map(NodeObject::node_data)
            .collect();

        let edge_data = self
            .edges()
            .snapshot()
            .iter()
            // TODO: create a GObject containing just a node like TaskObject, so that I can downcast and map it to the original structure
            .filter_map(Cast::downcast_ref::<EdgeObject>)
            .map(EdgeObject::edge_data)
            .collect();

        CollectionData {
            vertices_data: vertex_data,
            edges_data: edge_data,
        }
    }

    pub fn from_collection_data(collection_data: CollectionData) -> Self {
        let nodes_to_extend: Vec<NodeObject> = collection_data
            .vertices_data
            .into_iter()
            .map(NodeObject::from_node_data)
            .collect();
        let edges_to_extend: Vec<EdgeObject> = collection_data
            .edges_data
            .into_iter()
            .map(EdgeObject::from_edge_data)
            .collect();

        let nodes = gio::ListStore::new(NodeObject::static_type());
        nodes.extend_from_slice(&nodes_to_extend);
        let edges = gio::ListStore::new(EdgeObject::static_type());
        edges.extend_from_slice(&edges_to_extend);

        Self::new(nodes, edges)
    }
}
