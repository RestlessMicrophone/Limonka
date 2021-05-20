
use glam::vec2;
use std::collections::BinaryHeap;


pub struct Astar{

    Nodes: Vec<Node>,

}

impl Astar{
    fn search(from: &Node, to: &Node) -> Vec<Node>{

        let mut open = BinaryHeap::new();
        //// using vector2 for closed so that we don't need to make any new nodes beforehand
        let mut closed: Vec<Vec2> = vec![];

        let finding_path = true;

        let mut current: &Node = from;

        while finding_path {

            if assert_eq!(current.position, to.position){

                let mut path_to_return: Vec<&Node> = vec![];

                while !assert_eq!(current.position, from.position) {
                    path_to_return.push(current);
                    current = &current.parent;
                }

                path_to_return
            }
            else {
         //// here the actual pathfinding loop is happening

                let connected_nodes: Vec<Node> = current.get_connected_nodes();

                for Node  in connected_nodes{

                    /// no need to use already closed nodes
                    if !closed.contains(&Node.position) {

                    if &Node.F < &current.F {
                        open.push(Node);
                    }

                    if &current.F < &Node.F {

                        closed.push(&Node.position);

                    }

                    closed.push(&current.position);
                    }
                }
            }
        }
        none
    }
}

//// node functions
pub struct Node{
    pub position: Vec2,
    parent: Node,

    F: f32,
    G: f32,
    H: f32,

    expected_size: i32

}

impl Node {
    fn dis(&self, to: Node) -> f32 {
        let result = Vec2::new(&to.position.x - self.position.x, &to.position.y - self.position.y);
        f32: to_return = result.x + result.y;
        to_return
    }

    /*
    fn get_connected_nodes(&self) -> Vec<Node>{




    }
     */
}

impl init_node for Node {
    fn new(&self, node_orgin: Node, parent_node: Node, node_goal: Node) -> Self {
        Self{
            position: (),
            parent: parent_node,
            G: self.dis(node_orgin),
            H: self.dis(node_goal),
            F: H + G,
            expected_size: 1 + parent.expected_size
        }
    }
}