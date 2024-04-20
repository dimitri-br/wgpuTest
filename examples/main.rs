use log::{info, warn};
use rand::Rng;
use winit::event::{Event, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use minirender::{Command, Renderer, Transform, UniformBufferType};



fn rand_0_1() -> i32 {
    let mut rng = rand::thread_rng();
    let n: f32 = rng.gen();
    (n * 10.0) as i32
}


fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut renderer = match Renderer::new(window) {
        Ok(renderer) => renderer,
        Err(e) => {
            eprintln!("Error creating renderer: {}", e);
            return;
        }
    };


    let mut render_node = renderer.get_render_node("Normal Cube".to_string());
    render_node.use_depth(true);

    render_node.add_command(
        Command::LoadShader("examples/shaders/hello.wgsl".to_string())
    );

    let transform = Transform{
        position: [0.0, 0.0, -20.0].into(),
        rotation: [0.0, 0.0, 0.0].into(),
        scale: [1.0, 1.0, 1.0].into(),
    };
    let _transform_buffer = render_node.add_uniform_buffer(&transform, UniformBufferType::DYNAMIC);

    let mut camera = minirender::Camera::new([0.0, 1.0, 5.0].into(), [0.0, 0.0, 0.0].into(), 45.0, renderer.get_surface_configuration());
    let camera_buffer = render_node.add_uniform_buffer(&camera, UniformBufferType::DYNAMIC);

    render_node.add_command(
        Command::BindTexture(1, "examples/textures/cube.jpeg".to_string())
    );

    render_node.add_command(
        Command::DrawMesh("examples/meshes/cube obj.obj".to_string())
    );


    renderer.add_render_node(render_node);




    let mut instanced_render_node = renderer.get_render_node("Instanced Cube".to_string());
    instanced_render_node.use_depth(true);

    instanced_render_node.add_command(
        Command::LoadShader("examples/shaders/hello_inst.wgsl".to_string())
    );

    instanced_render_node.add_uniform_buffer_handle(camera_buffer.clone().unwrap(), UniformBufferType::DYNAMIC);

    let mut transforms = Vec::new();
    // Generate a bunch of cubes in a grid
    for x in -15..15 {
        for y in -15..15 {
            for z in -35..-10 {
                // Random Positions
                let x = x + rand_0_1() * 2;
                let y = y + rand_0_1() * 2;
                let z = z + rand_0_1() * 2;


                let transform = Transform{
                    // Random posiitons
                    position: [x as f32, y as f32, z as f32].into(),
                    rotation: [0.0, 0.0, 0.0].into(),
                    scale: [0.1, 0.1, 0.1].into(),
                };
                transforms.push(transform);
            }
        }
    }

    instanced_render_node.add_command(
        Command::BindTexture(1, "examples/textures/instance.png".to_string())
    );

    instanced_render_node.add_command(
        Command::DrawMeshInstanced("examples/meshes/cube obj.obj".to_string(), transforms)
    );

    renderer.add_render_node(instanced_render_node);

    // Once this is run, all the render nodes will be built and the pipeline will be created
    renderer.initialize();

    event_loop
        .run(|event, target| {

            // Check for exit event
            match event {
                Event::WindowEvent { ref event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            target.exit();
                        }
                        // On RedrawRequested, request a redraw
                        WindowEvent::RedrawRequested => {
                            // Update buffers here
                            if let Some(c_buffer) = &camera_buffer {
                                c_buffer.update(&camera);
                            }
                        }
                        WindowEvent::Resized(new_size) => {
                            camera.resize(*new_size);
                        },
                        WindowEvent::KeyboardInput { event, .. } => {
                            match event {
                                KeyEvent { physical_key, ..} => {
                                    match physical_key {
                                        PhysicalKey::Code(code) => {
                                            handle_input(&code, &mut camera);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                // Move the camera
                Event::DeviceEvent { ref event, .. } => {
                    match event {
                        winit::event::DeviceEvent::MouseMotion { delta } => {
                            let delta = nalgebra::Vector3::new(delta.0 as f32, -delta.1 as f32, 0.0);
                            camera.move_rotation(delta / 10.0);
                        }
                        winit::event::DeviceEvent::Key(input) => {
                            match input.physical_key {
                                PhysicalKey::Code(code) =>{
                                    handle_input(&code, &mut camera);
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                _ => {}
            }

            if !target.exiting() {
                renderer.update(event);
            }
        })
        .unwrap();
}


fn handle_input(code: &KeyCode, camera: &mut minirender::Camera) {
    match code {
        KeyCode::KeyW => {
            camera.move_position(nalgebra::Vector3::new(0.0, 0.0, 0.25));
        }
        KeyCode::KeyS => {
            camera.move_position(nalgebra::Vector3::new(0.0, 0.0, -0.25));
        }
        KeyCode::KeyA => {
            camera.move_position(nalgebra::Vector3::new(-0.25, 0.0, 0.0));
        }
        KeyCode::KeyD => {
            camera.move_position(nalgebra::Vector3::new(0.25, 0.0, 0.0));
        }
        KeyCode::KeyQ => {
            camera.move_position(nalgebra::Vector3::new(0.0, -0.25, 0.0));
        }
        KeyCode::KeyE => {
            camera.move_position(nalgebra::Vector3::new(0.0, 0.25, 0.0));
        }
        _ => {}
    }
}
