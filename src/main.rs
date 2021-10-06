extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;
use scene_graph::SceneNode;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()



// == // Modify and complete the function below for the first task
unsafe fn setup_vao(vertices: &Vec<f32>, indices: &Vec<u32>, color: &Vec<f32>, normals: &Vec<f32>) -> u32 {
    //Declare the VAO and VBO
    let mut VAO: u32 = 0;
    let mut vertex_VBO: u32 = 0;
    let mut index_VBO: u32 = 0;
    let mut color_VBO: u32 = 0;
    let mut normals_VBO: u32 = 0;


    // Generate vertex arrays and buffers
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut vertex_VBO);

    // Binds the vertex array
    gl::BindVertexArray(VAO);


    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_VBO);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(&vertices), pointer_to_array(&vertices), gl::STATIC_DRAW);
    

    // Generate buffer for indices
    gl::GenBuffers(1, &mut index_VBO);

    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_VBO);
    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(&indices), pointer_to_array(&indices), gl::STATIC_DRAW);

    gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>(), ptr::null());

    gl::EnableVertexAttribArray(0);


    // Generate buffer for color
    gl::GenBuffers(1, &mut color_VBO);
    gl::BindBuffer(gl::ARRAY_BUFFER, color_VBO);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(&color), pointer_to_array(&color), gl::STATIC_DRAW);

    gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 4 * size_of::<f32>(), ptr::null());

    gl::EnableVertexAttribArray(1);

    //Generate buffer for normals
    gl::GenBuffers(1, &mut normals_VBO);
    gl::BindBuffer(gl::ARRAY_BUFFER, normals_VBO);
    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(&normals), pointer_to_array(&normals), gl::STATIC_DRAW);

    gl::VertexAttribPointer(5, 3, gl::FLOAT, gl::FALSE, 3 * size_of::<f32>(), ptr::null());
    gl::EnableVertexAttribArray(5);

    return VAO;
} 

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO here

        //Vector containing vertices for the triangles
        let vertex_vector: Vec<f32> = vec![
            //Task 1
            // -1.0, -1.0, 0.0,
            // 0.0, -1.0, 0.0,
            // -0.5, 0.0, 0.0,

            // 0.0, -1.0, 0.0,
            // 1.0, -1.0, 0.0,
            // 0.5, 0.0, 0.0,

            // 0.0, 0.0, 0.0,
            // 0.5, 1.0, 0.0,
            // -0.5, 1.0, 0.0

            //Task 2, 4
            -0.3, -0.3, 0.5, // first triangle
            0.3, -0.3, 0.5,
            0.0, 0.7, 0.5,

            -0.6, -0.4, 0.0, // second triangle
            0.0, -0.4, 0.0,
            -0.3, 0.4, 0.0,

            -0.5, -0.5, -1.0, // third triangle
            0.1, -0.5, -1.0,
            -0.2, 0.1, -1.0,

        ];
      
        let indices_array: Vec<u32> = vec![0,1,2,3,4,5,6,7,8];
        
        //Task 2b_ii
        //let indices_array: Vec<u32> = vec![6,7,8,3,4,5,0,1,2];

        let color_array: Vec<f32> = vec![
            //Task 1
            // 1.0, 0.0, 0.0, 1.0, // first triangle
            // 0.0, 1.0, 0.0, 1.0,
            // 0.0, 0.0, 1.0, 1.0,

            // 0.2, 0.3, 0.3, 1.0, // second triangle
            // 1.0, 0.0, 0.6, 1.0,
            // 0.0, 0.8, 0.3, 1.0,

            // 0.5, 0.6, 0.1, 1.0, // third triangle
            // 0.3, 0.2, 0.8, 1.0,
            // 0.8, 0.1, 0.5, 1.0,

            //Task 2, 4
            1.0, 0.0, 0.0, 0.4,
            1.0, 0.0, 0.0, 0.4,
            1.0, 0.0, 0.0, 0.4,

            0.0, 1.0, 0.0, 0.4,
            0.0, 1.0, 0.0, 0.4,
            0.0, 1.0, 0.0, 0.4,

            0.0, 0.0, 1.0, 0.4,
            0.0, 0.0, 1.0, 0.4,
            0.0, 0.0, 1.0, 0.4,

        ];

       
        //Load models
        let terrain = mesh::Terrain::load("./resources/lunarsurface.obj");
        let helicopter = mesh::Helicopter::load("./resources/helicopter.obj");

        //Build vaos
        let vao_terrain;
        let vao_body;
        let vao_door;
        let vao_main_rotor;
        let vao_tail_rotor;
        unsafe {
            //vao = setup_vao(&vertex_vector, &indices_array, &color_array);
            vao_terrain = setup_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals);
            vao_body = setup_vao(&helicopter.body.vertices,&helicopter.body.indices, &helicopter.body.colors, &helicopter.body.normals);
            vao_door = setup_vao(&helicopter.door.vertices,&helicopter.door.indices, &helicopter.door.colors, &helicopter.door.normals);
            vao_main_rotor = setup_vao(&helicopter.main_rotor.vertices,&helicopter.main_rotor.indices, &helicopter.main_rotor.colors, &helicopter.main_rotor.normals);
            vao_tail_rotor = setup_vao(&helicopter.tail_rotor.vertices,&helicopter.tail_rotor.indices, &helicopter.tail_rotor.colors, &helicopter.tail_rotor.normals);
        };

        
        let mut root_node = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(vao_terrain, terrain.index_count);
        let mut body_node = SceneNode::from_vao(vao_body, helicopter.body.index_count);
        let mut door_node = SceneNode::from_vao(vao_door, helicopter.door.index_count);
        let mut main_rotor_node = SceneNode::from_vao(vao_main_rotor, helicopter.main_rotor.index_count);
        let mut tail_rotor_node = SceneNode::from_vao(vao_tail_rotor, helicopter.tail_rotor.index_count);

        root_node.add_child(&terrain_node);
        terrain_node.add_child(&body_node);
        body_node.add_child(&door_node);
        body_node.add_child(&main_rotor_node);
        body_node.add_child(&tail_rotor_node);
    
        body_node.print();

        body_node.position = glm::vec3(0.0, 00.0, -40.0);
        body_node.rotation.y = 3.00;
        

        tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        main_rotor_node.reference_point = glm::vec3(0.0, 0.0, 0.0);



        // Basic usage of shader helper:
        // The example code below returns a shader object, which contains the field `.program_id`.
        // The snippet is not enough to do the assignment, and will need to be modified (outside of
        // just using the correct path), but it only needs to be called once
        //
        let shader: shader::Shader;
        unsafe {
            shader = shader::ShaderBuilder::new()
            .attach_file("./shaders/simple.vert")   
            .attach_file("./shaders/simple.frag")
            .link();
            shader.activate();
        }

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;

        // Variables to store the motion
        let mut movement_coords = glm::vec3(0.0, 0.0, 0.0);
        let mut rotation_coords =  glm::vec3(0.0, 0.0, 0.0);
        let speed_constant: f32 = 100.0;

        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::D => {
                            movement_coords[0] += delta_time * speed_constant;
                        },
                        VirtualKeyCode::A => {
                            movement_coords[0] -= delta_time * speed_constant;
                        },
                        VirtualKeyCode::W => {
                            movement_coords[1] += delta_time * speed_constant;
                        },
                        VirtualKeyCode::S => {
                            movement_coords[1] -= delta_time * speed_constant;
                        },
                        VirtualKeyCode::Q => {
                            movement_coords[2] += delta_time * speed_constant;
                        },
                        VirtualKeyCode::E => {
                            movement_coords[2] -= delta_time * speed_constant;
                        },
                        VirtualKeyCode::Left => {
                            rotation_coords[0] += delta_time
                        },
                        VirtualKeyCode::Right => {
                            rotation_coords[0] -= delta_time
                        },
                        VirtualKeyCode::Up => {
                            rotation_coords[1] += delta_time
                        },
                        VirtualKeyCode::Down => {
                            rotation_coords[1] -= delta_time
                        },
                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {



                *delta = (0.0, 0.0);
            }

            unsafe {
                //let matrix = glm::Mat4::from([
                //     [1.0, 0.0, 0.0, elapsed.sin()],
                //     [0.0, 1.0, 0.0, elapsed.sin()],
                //     [1.0, 0.0, 1.0, 0],
                //     [0.0, 0.0, 0.0, 1.0],
                // ]);
                //Identity matrix
                let identity_matrix: glm::Mat4 = glm::identity();

                // Projection
                let projection: glm::Mat4 = glm::perspective(SCREEN_H as f32 / SCREEN_W as f32, 0.5, 1.0, 1000.0);

                //View
                let movement: glm::Mat4 = glm::translation(&movement_coords);
                let rotation: glm::Mat4 = glm::rotation(-rotation_coords[1], &glm::vec3(1.0, 0.0, 0.0)) * glm::rotation(rotation_coords[0], &glm::vec3(0.0, 1.0, 0.0));
                let combination: glm::Mat4 = movement * rotation;

                //Model
                let model: glm::Mat4 = glm::translation(&glm::vec3(0.0, 0.0, -4.0));

                let matrix: glm::Mat4 = (projection * identity_matrix) * (model * identity_matrix) * (movement * rotation * identity_matrix); 

                let transformation_loc = shader.get_uniform_location("transformation");
               // gl::UniformMatrix4fv(transformation_loc, 1, gl::FALSE, matrix.as_ptr());

                gl::ClearColor(0.2, 0.3, 0.3, 1.0); // moon raker, full opacity
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                gl::Enable(gl::BLEND);
                gl::Disable(gl::CULL_FACE); 
                gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

                unsafe fn draw_scene(node: &scene_graph::SceneNode, view_projection_matrix: &glm::Mat4) {
                    if node.index_count > 0 {  
                        gl::UniformMatrix4fv(2, 1, 0, (view_projection_matrix*node.current_transformation_matrix).as_ptr());
                        gl::BindVertexArray(node.vao_id);
                        gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, ptr::null());
                    }
                    for &child in &node.children {
                        draw_scene(&*child, view_projection_matrix);
                    }
                }

                unsafe fn update_node_transformations(node: &mut scene_graph::SceneNode, transformation_so_far: &glm::Mat4) {
                    //Construct the correct transformation matrix
                    let mut transformation = *transformation_so_far; //The first transformation_so_far will be an identity matrix

                    transformation = glm::translation(&glm::vec3(-node.reference_point.x, -node.reference_point.y, -node.reference_point.z))*transformation;

                    transformation = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0))*transformation;
                    transformation = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0))*transformation;
                    transformation = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0))*transformation;

                    transformation = glm::translation(&glm::vec3(node.reference_point.x, node.reference_point.y, node.reference_point.z))*transformation;

                    transformation = glm::translation(&glm::vec3(node.position.x, node.position.y, node.position.z))*transformation;

                    //Update the node's transformation matrix
                    node.current_transformation_matrix = transformation;

                    for &child in &node.children {
                        update_node_transformations(&mut *child, &node.current_transformation_matrix);
                    }
                }

                update_node_transformations(&mut root_node , &identity_matrix);

                draw_scene(&root_node, &projection);


                // // Issue the necessary commands to draw your scene here
                // gl::BindVertexArray(vao_terrain);
                // gl::DrawElements(gl::TRIANGLES, terrain.index_count, gl::UNSIGNED_INT, ptr::null());
                // gl::BindVertexArray(vao_body);
                // gl::DrawElements(gl::TRIANGLES, helicopter.body.index_count, gl::UNSIGNED_INT, ptr::null());
                // gl::BindVertexArray(vao_door);
                // gl::DrawElements(gl::TRIANGLES,  helicopter.door.index_count, gl::UNSIGNED_INT, ptr::null());
                // gl::BindVertexArray(vao_main_rotor);
                // gl::DrawElements(gl::TRIANGLES,  helicopter.main_rotor.index_count, gl::UNSIGNED_INT, ptr::null());
                // gl::BindVertexArray(vao_tail_rotor);
                // gl::DrawElements(gl::TRIANGLES,  helicopter.tail_rotor.index_count, gl::UNSIGNED_INT, ptr::null());

                    

            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    // Q => {
                    //     *control_flow = ControlFlow::Exit;
                    // }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}
