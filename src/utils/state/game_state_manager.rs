use std::borrow::Borrow;

pub struct StateManager<'state_lifetime> {
    pub current_states: Vec<state>,
    pub current_state: &'state_lifetime state,
    pub time_to_set: bool
}

//// the lifetime of the struct and impl aren't the same despite the name
impl<'state_lifetime> StateManager <'state_lifetime>{
    pub fn set_to_new_state(& mut self, to_add: state){

        if  !self.current_states.contains(&to_add){
        }
        else {

        }
    }

    pub fn remove_state(&self, state_string: &str){

    }

    pub fn change_state(&self){
        if self.time_to_set {

        }


    }

    pub fn return_state(&self) -> &i32 {
        let i : &i32 = &self.current_state.state_id;
        i
    }
}


pub struct state{
    pub state_name: String,
    pub state_id: i32
}


impl PartialEq for state {
    fn eq(&self, other: &Self) -> bool {
        self.state_name == other.state_name
    }
}

