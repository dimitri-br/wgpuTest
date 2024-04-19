use log::info;
use winit::event_loop::ControlFlow;
use minirender::{Renderer, Command, Transform};


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
    let mut transform = Transform{
        position: [0.0, 0.0, -5.0, 0.0],
        rotation: [-0.5, 0.785, 0.2, 0.0],
        scale: [1.0, 1.0, 1.0, 0.0],
    };

    let transform_buffer = render_node.add_uniform_buffer(transform, minirender::UniformBufferType::STATIC);
    render_node.add_command(
        Command::DrawMesh("examples/meshes/cube obj.obj".to_string())
    );




    renderer.add_render_node(render_node);

    let mut instanced_render_node = renderer.get_render_node("Instanced Cube".to_string());
    instanced_render_node.use_depth(true);

    instanced_render_node.add_command(
        Command::LoadShader("examples/shaders/hello_inst.wgsl".to_string())
    );

    let mut transforms = Vec::new();
    // Generate a bunch of cubes in a grid
    for x in -25..25 {
        for y in -25..25 {
            for z in -75..-15 {
                let transform = Transform{
                    position: [x as f32, y as f32, z as f32, 0.0],
                    rotation: [0.0, 0.0, 0.0, 0.0],
                    scale: [0.1, 0.1, 0.1, 0.0],
                };
                transforms.push(transform);
            }
        }
    }

    instanced_render_node.add_command(
        Command::DrawMeshInstanced("examples/meshes/cube obj.obj".to_string(), transforms.len() as u32, transforms)
    );

    renderer.add_render_node(instanced_render_node);

    // Once this is run, all the render nodes will be built and the pipeline will be created
    renderer.initialize();

    event_loop
        .run(|event, target| {
            target.set_control_flow(ControlFlow::Poll);


            if let Some(target) = &transform_buffer {
                // Spin the cube
                transform.rotation[1] += 0.005;
                transform.rotation[2] += 0.005;

                // Scale up and down
                transform.scale[0] = 1.0 + (transform.rotation[1].sin() * 0.5);
                transform.scale[1] = 1.0 + (transform.rotation[1].sin() * 0.5);
                transform.scale[2] = 1.0 + (transform.rotation[1].sin() * 0.5);

                // Move back and forth on the z axis
                transform.position[2] = -15.0 + (transform.rotation[1].sin() * 2.0);


                // Update the transform buffer
                target.update(transform);
            }

            // Check for exit event
            match event {
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    target.exit();
                }
                _ => {}
            }

            if !target.exiting() {
                renderer.update(event);
            }
        })
        .unwrap();
}
