use shipyard::{World, EntityId};
use raylib::math::Vector3;

pub struct eHandler {
    ECSworld: World,
    IDlist : Vec<EntityId>

}

pub fn return_handler() -> eHandler{
    let handler = eHandler{
        ECSworld: World::default(),
        IDlist: vec![]
    };
    handler
}

impl eHandler{

    // entity code
    pub fn add_entity(&mut self){
        let empty_entity = self.ECSworld.add_entity(());
        self.IDlist.push(empty_entity);
    }

    pub fn dummy_entity(&mut self){

        let dummy = self.ECSworld.add_entity(());
        self.ECSworld.add_component(dummy, (Vector3::zero(),));
        
    }

    pub fn delete_entity(&mut self, id: EntityId){
        self.ECSworld.delete_entity(id);
    }
    // conponent code




}