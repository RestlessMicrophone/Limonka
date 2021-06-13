

// this Astar is meant to work on every graph, and isn't indexed,


pub struct Astar<T>{


}

impl <T> Astar<T>{

    pub fn get_path(from : T, to : T){




    }

}

pub struct Node<T>{
    F : i32,
    G : i32,
    H : i32,
    Containing : T
}

impl<T> Node<T>{

    pub fn construct_node(G_cost: i32, H_cost : i32) -> Node<T>{
        let node = Node{
            G: G_cost,
            H: H_cost,
            F: G + H,
            Containing: (T)
        };
        node
    }


}

