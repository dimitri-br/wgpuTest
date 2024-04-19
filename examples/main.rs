use log::info;
use winit::event_loop::ControlFlow;
use winit::event::{Event, WindowEvent};
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
        position: [0.0, -5.0, -50.0, 0.0],
        rotation: [-0.72, 0.72, 0.25, 0.0],
        scale: [0.01, 0.01, 0.01, 0.0],
    };

    let transform_buffer = render_node.add_uniform_buffer(transform, minirender::UniformBufferType::STATIC);

    render_node.add_command(
        Command::BindTexture(1, "examples/textures/cube.jpeg".to_string())
    );

    render_node.add_command(
        Command::DrawMesh("examples/meshes/sponza.obj".to_string())
    );


    renderer.add_render_node(render_node);




    let mut instanced_render_node = renderer.get_render_node("Instanced Cube".to_string());
    instanced_render_node.use_depth(true);

    instanced_render_node.add_command(
        Command::LoadShader("examples/shaders/hello_inst.wgsl".to_string())
    );

    let mut transforms = Vec::new();
    // Generate a bunch of cubes in a grid
    for x in -15..15 {
        for y in -15..15 {
            for z in -35..-10 {
                let transform = Transform{
                    position: [x as f32, y as f32, z as f32, 0.0],
                    rotation: [0.0, 0.0, 0.0, 0.0],
                    scale: [0.25, 0.25, 0.25, 0.0],
                };
                transforms.push(transform);
            }
        }
    }

    instanced_render_node.add_command(
        Command::BindTexture(0, "examples/textures/instance.png".to_string())
    );

    instanced_render_node.add_command(
        Command::DrawMeshInstanced("examples/meshes/cube obj.obj".to_string(), transforms.len() as u32, transforms)
    );

    //renderer.add_render_node(instanced_render_node);

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
