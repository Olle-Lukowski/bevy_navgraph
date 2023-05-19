use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use rand::Rng;

pub struct NavGraphPlugin {
    pub starting_position: Vec3,
    pub max_bounces: usize,
    pub splits_per_bounce: usize,
}

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub id: usize,
    pub position: Vec3,
    pub connections: Vec<usize>,
    pub bounces_before: usize,
}

impl Node {
    pub fn new(id: usize, position: Vec3, bounces_before: usize) -> Self {
        Node {
            id,
            position,
            connections: Vec::new(),
            bounces_before,
        }
    }
}

#[derive(Default, Resource)]
pub struct NavGraph {
    pub current_nodes: Vec<Node>,
    pub root: Node,
    pub count: usize,
    pub max_bounces: usize,
    pub splits_per_bounce: usize,
    pub done: bool,
    pub first_run: bool,
    pub nodes: Vec<Node>,
}

impl Plugin for NavGraphPlugin {
    fn build(&self, app: &mut App) {
        let root_node = Node::new(0, self.starting_position, 0);
        app.insert_resource(NavGraph {
            current_nodes: vec![root_node.clone()],
            root: root_node.clone(),
            count: 1,
            max_bounces: self.max_bounces,
            splits_per_bounce: self.splits_per_bounce,
            done: false,
            first_run: true,
            nodes: vec![root_node.clone()],
        });
        app.add_system(scan.run_if(|graph: Res<NavGraph>| !graph.done));
    }
}
pub fn scan(mut graph: ResMut<NavGraph>, rapier_context: Res<RapierContext>) {
    let mut new_nodes: Vec<Node> = Vec::new();
    let mut count = graph.count;
    let max_bounces = graph.max_bounces;
    let splits_per_bounce = graph.splits_per_bounce;
    let mut done = false;
    let mut bounced = false;
    let first_run = graph.first_run;
    let mut root = graph.root.clone();
    let mut nodes = graph.nodes.clone();
    for node in graph.current_nodes.iter_mut() {
        if node.bounces_before < max_bounces {
            for _ in 0..splits_per_bounce {
                let x_direction = rand::thread_rng().gen_range(-1.0..1.0);
                let y_direction = rand::thread_rng().gen_range(-1.0..1.0);
                let z_direction = rand::thread_rng().gen_range(-1.0..1.0);
                if let Some((_, position)) = rapier_context.cast_ray_and_get_normal(
                    node.position,
                    Vec3::new(x_direction, y_direction, z_direction),
                    Real::MAX,
                    false,
                    Default::default(),
                ) {
                    let mut new_node = Node::new(count, position.point, node.bounces_before + 1);
                    new_node.connections.push(node.id);
                    new_nodes.push(new_node.clone());
                    nodes.push(new_node.clone());
                    node.connections.push(new_node.id);
                    if first_run {
                        root.connections.push(new_node.id);
                    }
                    count += 1;
                    bounced = true;
                }
            }
        } else {
            done = true;
        }
    }
    graph.done = done;
    graph.root = root;
    graph.nodes = nodes;
    if bounced {
        graph.first_run = false;
        graph.current_nodes = new_nodes;
    }
    graph.count = count;
}
