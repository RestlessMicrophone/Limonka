
pub struct StateManager {
    pub(crate) current_states: Vec<state>,
    pub(crate) current_state: state
}

impl StateManager {
    pub fn add_state_on_top(& mut self, to_add: state){

        if  !self.current_states.contains(&to_add){
            self.current_states.push(to_add);
        }
    }

    pub fn remove_state(&self, state_string: &str){

    }

    fn set_to_new_state(&self){

    }
}


pub struct state{
    pub(crate) state_name: str
}

fn main(){
    let begin_state = state{
        state_name: *String::from("stateA")
    };

    let state_to_add = state{
        state_name: *String::from("stateB")
    };

    let mut x: StateManager = StateManager{
        current_states: vec![],
        current_state: begin_state
    };

    x.add_state_on_top(state_to_add);

}

impl PartialEq for state {
    fn eq(&self, other: &Self) -> bool {
        self.state_name == other.state_name
    }
}

