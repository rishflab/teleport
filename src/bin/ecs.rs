extern crate blackhole;

use blackhole::renderer::pathtracer::Pathtracer;
use blackhole::window::WindowState;
use blackhole::renderer::core::backend::{create_backend};
use blackhole::input::{InputState, Command, MoveCommand};
use blackhole::scene::{Scene};
use specs::prelude::*;
use blackhole::asset::{load_gltf, MeshData};
use blackhole::scene::mesh::{StaticMeshData, MeshInstance};
use blackhole::scene;
use blackhole::components::*;
use blackhole::systems::FpsMovement::PlayerMovement;
use blackhole::systems::SceneBuilder::SceneBuilder;
use nalgebra_glm::{vec3, vec3_to_vec4};
use nalgebra_glm as glm;


fn main() {
    env_logger::init();

    let asset_folder = "assets";
    let gltf = load_gltf(asset_folder, "untitled.gltf")
        .expect("failed to load gltf");

    let mut window = WindowState::new();
    let mut input = InputState::new();
    let (backend, _instance) = create_backend(&mut window, &mut input);

    let mesh_data = MeshData::from_gltf(&gltf, asset_folder);

    let cube_mesh = StaticMeshData {
        id: 0,
        indices: mesh_data.indices.clone(),
        vertices: mesh_data.vertices.clone(),
    };

    let mut world = World::new();

    let mut init = DispatcherBuilder::new()
        .with(SceneBuilder, "scene_builder", &[])
        .build();

    init.setup(&mut world);

    let mut dispatcher = DispatcherBuilder::new()
    .with(PlayerMovement, "player_movement", &[])
    .with(SceneBuilder, "scene_builder", &[])
    .build();

    dispatcher.setup(&mut world);

    world.insert(Scene::default());
    world.insert(MoveCommand::default());

    let floor = world.create_entity()
        .with(StaticMesh(0))
        .with(Transform {
            position: glm::vec3(0.0, 0.0, 0.0),
            scale: glm::vec3(10.0, 1.0, 10.0),
            rotation: glm::vec3(0.0, 0.0, 1.0),
        })
        .build();

    let player = world.create_entity()
        .with(Transform {
            position: vec3(0.0, 4.0, 8.0),
            scale: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
        })
        .with(Camera {
            look_at: vec3(0.0, 2.0, -7.0)
        })
        .with(Player)
        .build();


    let light = world.create_entity()
        .with(PointLight(20.0))
        .with(Transform {
            position: vec3(1.5, 4.0, 4.0),
            scale: vec3(0.0, 0.0, 0.0),
            rotation: vec3(0.0, 0.0, 0.0),
        })
        .build();


    init.dispatch(&world);

    let mut renderer = unsafe {
        Pathtracer::new(backend, window, &world.fetch::<Scene>())
    };

    let mut running = true;

    while running {
        use std::time::Instant;

        {
            let mut move_command = world.write_resource::<MoveCommand>();
            *move_command = MoveCommand::None;
        }
        match input.process_raw_input() {
            Some(command) => {
                match command {
                    Command::Close => {
                        running = false;
                    },
                    Command::MoveCmd(next_move) => {
                        let mut move_command = world.write_resource::<MoveCommand>();
                        *move_command = next_move
                    },
                }
            },
            None => (),
        }

        dispatcher.dispatch(&world);


        let start = Instant::now();

        renderer.render(&world.fetch::<Scene>());

        let duration = start.elapsed();

        println!("Frame time {:?}", duration);
    }

}