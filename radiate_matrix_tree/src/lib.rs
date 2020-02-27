
pub mod prelude;
pub mod matrix_tree;

pub use matrix_tree::{
    evtree::Evtree,
    evenv::TreeEnvionment,
    iterators::{
        InOrderIterator,
        LevelOrderIterator,
        IterMut
    },
    network::NeuralNetwork,
    node::Node
};

/// get a default environment settings for evtree, these are very basic and 
/// are used to solve the xor problem. These are just settings to evolve thre
/// tree, more can be added or taken away depending on the desired problem to solve
pub fn defualt_evtree_env() -> TreeEnvionment {
    TreeEnvionment::new()
        .set_input_size(2)
        .set_outputs(vec![0, 1])
        .set_start_height(2)
        .set_max_height(3)
        .set_network_mutation_rate(0.5)
        .set_node_add_rate(0.1)
        .set_gut_rate(0.05)
        .set_shuffle_rate(0.05)
        .set_layer_mutate_rate(0.8)
        .set_weight_mutate_rate(0.8)
        .set_weight_transform_rate(5.0)
}