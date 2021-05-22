use crate::utils::world_generator::world_components;
use crate::utils::world_generator::world_components::position3D;

use specs::{Builder, Component, ReadStorage, System, VecStorage, World, WorldExt, RunNow, DispatcherBuilder};

fn world_update(){
    let mut world = World::new();
    world.register::<world_components::position3D>();

    let block = world.create_entity().with(position3D{ posX: 0.0, posY: 0.0 }).build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world);
    world.maintain();

    let mut dispatcher = DispatcherBuilder::new()
        .with(HelloWorld, "hello_world", &[])
        .build();

}

struct HelloWorld;

impl<'a> System<'a> for HelloWorld {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}