use nalgebra::{Matrix4, Point3, Vector3};
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;
use wgpu::util::DeviceExt;
use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::renderer::SimpleCamera::SimpleCamera;

use bytemuck::{Pod, Zeroable};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use winit::event::{ElementState, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

#[repr(C)]
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// Ensure Vertex is Pod and Zeroable
unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

struct Pyramid {
    position: Vector3<f32>,
    rotation: Vector3<f32>,
    scale: Vector3<f32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl Pyramid {
    fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let empty_buffer = Matrix4::<f32>::identity();
        let raw_matrix = matrix4_to_raw_array(&empty_buffer);

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Pyramid Uniform Buffer"),
            contents: bytemuck::cast_slice(&raw_matrix),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: None,
        });

        Self {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            vertex_buffer,
            index_buffer,
            uniform_buffer,
            bind_group,
        }
    }

    fn update_transform(&self) -> Matrix4<f32> {
        let translation = Matrix4::new_translation(&self.position);
        let rotation =
            Matrix4::from_euler_angles(self.rotation.x, self.rotation.y, self.rotation.z);
        let scale = Matrix4::new_nonuniform_scaling(&self.scale);
        // web_sys::console::log_1(&format!("Pyramid translation: {:?}", translation).into());
        translation * rotation * scale
    }

    fn update_uniform_buffer(&self, queue: &wgpu::Queue) {
        let transform_matrix = self.update_transform();
        let transform_matrix = transform_matrix.transpose(); // Transpose to match wgpu layout
        let raw_matrix = matrix4_to_raw_array(&transform_matrix);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&raw_matrix));
    }

    fn translate(&mut self, translation: Vector3<f32>) {
        self.position += translation;
        // web_sys::console::log_1(&format!("Pyramid position: {:?}", self.position).into());
        // alternative translation method
        // let translation_matrix = Matrix4::new_translation(&translation);
        // let translation_vector = translation_matrix.transform_vector(&self.position);
        // self.position = translation_vector;
    }

    fn rotate(&mut self, rotation: Vector3<f32>) {
        self.rotation += rotation;
    }

    fn scale(&mut self, scale: Vector3<f32>) {
        self.scale.component_mul_assign(&scale);
    }
}

fn matrix4_to_raw_array(matrix: &Matrix4<f32>) -> [[f32; 4]; 4] {
    let mut array = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            array[i][j] = matrix[(i, j)];
        }
    }
    array
}

// fn matrix4_to_raw_array(matrix: &nalgebra::Matrix4<f32>) -> [f32; 16] {
//     let mut raw_array = [0.0; 16];
//     for i in 0..4 {
//         for j in 0..4 {
//             raw_array[i * 4 + j] = matrix[(i, j)];
//         }
//     }
//     raw_array
// }

// Vertices for a pyramid
const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 1.0, 0.0],
        color: [1.0, 0.0, 0.0],
    }, // Apex
    Vertex {
        position: [-1.0, -1.0, -1.0],
        color: [0.0, 1.0, 0.0],
    }, // Base vertices
    Vertex {
        position: [1.0, -1.0, -1.0],
        color: [0.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0],
        color: [1.0, 1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0],
        color: [0.0, 1.0, 1.0],
    },
];

// Indices for a pyramid
const INDICES: &[u16] = &[
    0, 1, 2, // Side 1
    0, 2, 3, // Side 2
    0, 3, 4, // Side 3
    0, 4, 1, // Side 4
    1, 3, 2, // Base 1
    1, 4, 3, // Base 2
];

static mut CAMERA: Option<SimpleCamera> = None;

thread_local! {
    static CAMERA_INIT: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn get_camera() -> &'static mut SimpleCamera {
    CAMERA_INIT.with(|init| {
        if !init.get() {
            unsafe {
                CAMERA = Some(SimpleCamera::new(
                    Point3::new(0.0, 0.0, 5.0),
                    Vector3::new(0.0, 0.0, -1.0),
                    Vector3::new(0.0, 1.0, 0.0),
                    45.0f32.to_radians(),
                    0.1,
                    100.0,
                ));
            }
            init.set(true);
        }
    });

    unsafe { CAMERA.as_mut().unwrap() }
}

struct RendererState {
    pyramids: Vec<Pyramid>,
    // other fields like device, queue, etc.
}

impl RendererState {
    fn new(device: &wgpu::Device, bind_group_layout: &wgpu::BindGroupLayout) -> Self {
        let mut pyramids = Vec::new();
        pyramids.push(Pyramid::new(device, bind_group_layout));
        // add more pyramids as needed

        Self {
            pyramids,
            // initialize other fields
        }
    }
}

static mut RENDERER_STATE: Option<RendererState> = None;

thread_local! {
    static RENDERER_STATE_INIT: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn get_renderer_state() -> &'static mut RendererState {
    RENDERER_STATE_INIT.with(|init| {
        if !init.get() {
            panic!("RendererState not initialized");
        }
    });

    unsafe { RENDERER_STATE.as_mut().unwrap() }
}

// native rendering loop
// #[wasm_bindgen]
// pub async fn init_wgpu() -> Result<(), JsValue> {
//     // pipeline setup...

//     event_loop
//         .run(move |event, target| {
//             if let Event::WindowEvent {
//                 window_id: _,
//                 event,
//             } = event
//             {
//                 match event {
//                     // WindowEvent::CursorMoved { position, .. } => {
//                     //     // Update the mouse position
//                     //     // println!("Mouse Position: {:?}", position);
//                     //     mouse_position = (position.x as f64, position.y as f64);
//                     // }
//                     // WindowEvent::MouseInput {
//                     //     state: ElementState::Pressed,
//                     //     button: MouseButton::Left,
//                     //     ..
//                     // } => {
//                     //     let window_size = (size.width as f64, size.height as f64);
//                     //     handle_click(window_size, mouse_position, &buttons, &labels);
//                     // }
//                     // TODO: handle mouse events for native purposes
//                     WindowEvent::Resized(new_size) => {
//                         // Reconfigure the surface with the new size
//                         // config.width = new_size.width.max(1);
//                         // config.height = new_size.height.max(1);
//                         // surface.configure(&device, &config);
//                         // On macos the window needs to be redrawn manually after resizing
//                         // window.request_redraw();
//                     }
//                     WindowEvent::RedrawRequested => {
//                         // necessary for native purposes? (not needed for web)
//                     }
//                     WindowEvent::CloseRequested => target.exit(),
//                     _ => {}
//                 };
//             }
//         })
//         .unwrap();

//     Ok(())
// }

// Your other imports...

#[wasm_bindgen]
pub async fn start_render_loop() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("scene-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Set focus on the canvas
    // canvas.focus().unwrap();

    // TODO: improve settings

    // Create logical components (instance, adapter, device, queue, surface, etc.)
    let dx12_compiler = wgpu::Dx12Compiler::Dxc {
        dxil_path: None, // Specify a path to custom location
        dxc_path: None,  // Specify a path to custom location
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        dx12_shader_compiler: dx12_compiler,
        flags: wgpu::InstanceFlags::empty(),
        gles_minor_version: wgpu::Gles3MinorVersion::Version2,
    });

    let height = canvas.height();
    let width = canvas.width();

    let event_loop = event_loop::EventLoop::new().unwrap();
    let builder = WindowBuilder::new().with_inner_size(LogicalSize::new(width, height));
    #[cfg(target_arch = "wasm32")] // necessary for web-sys
    let builder = {
        use winit::platform::web::WindowBuilderExtWebSys;
        builder.with_canvas(Some(canvas))
    };
    let winit_window = builder.build(&event_loop).unwrap();

    let surface = unsafe {
        instance
            .create_surface(winit_window)
            .expect("Couldn't create GPU Surface")
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or("Failed to find an appropriate adapter")
        .unwrap();

    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    // let swap_chain_format: wgpu::TextureFormat = surface.get_preferred_format(&adapter).unwrap();
    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swap_chain_format = swapchain_capabilities.formats[0]; // Choosing the first available format

    // let size = canvas.get_bounding_client_rect();
    // let swap_chain_descriptor = wgpu::SurfaceConfiguration {
    //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //     format: swap_chain_format,
    //     width: width as u32,
    //     height: height as u32,
    //     present_mode: wgpu::PresentMode::Fifo,
    //     desired_maximum_frame_latency: 1,
    //     alpha_mode: wgpu::CompositeAlphaMode::Opaque,
    //     view_formats: vec![swap_chain_format], // Check?
    // };

    // surface.configure(&device, &swap_chain_descriptor);

    let mut config = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &config);

    // Create the shader module
    let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Vertex Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/primary_vertex.wgsl").into()),
    });

    let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Fragment Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("./shaders/primary_fragment.wgsl").into()),
    });

    // let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Vertex Buffer"),
    //     contents: bytemuck::cast_slice(VERTICES),
    //     usage: wgpu::BufferUsages::VERTEX,
    // });

    // let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //     label: Some("Index Buffer"),
    //     contents: bytemuck::cast_slice(INDICES),
    //     usage: wgpu::BufferUsages::INDEX,
    // });

    // Define the bind group layout and pipeline layout
    // let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
    //     label: Some("Uniform Bind Group Layout"),
    //     entries: &[wgpu::BindGroupLayoutEntry {
    //         binding: 0,
    //         visibility: wgpu::ShaderStages::VERTEX,
    //         ty: wgpu::BindingType::Buffer {
    //             ty: wgpu::BufferBindingType::Uniform,
    //             has_dynamic_offset: false,
    //             min_binding_size: None,
    //         },
    //         count: None,
    //     }],
    // });

    let camera = get_camera();

    camera.update_aspect_ratio(config.width as f32 / config.height as f32);

    // hardcode position test
    // camera.position = Point3::new(0.0, 0.0, 10.0);

    // let usable_camera = camera.clone();
    // let usable_camera = usable_camera.borrow_mut();

    // Create the uniform buffer for the camera
    // let camera_matrix = camera.build_view_projection_matrix();
    camera.update_view_projection_matrix();
    let camera_matrix = camera.view_projection_matrix;
    let camera_uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Uniform Buffer"),
        contents: bytemuck::cast_slice(camera_matrix.as_slice()),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create the bind group for the uniform buffer
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    });

    let pyramid_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("pyramid_bind_group_layout"),
        });

    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_uniform_buffer.as_entire_binding(),
        }],
        label: Some("Bind Group"),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout, &pyramid_bind_group_layout],
        push_constant_ranges: &[],
    });

    // Create the render pipeline
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &vertex_shader,
            entry_point: "main",
            buffers: &[Vertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        },
        fragment: Some(wgpu::FragmentState {
            module: &fragment_shader,
            entry_point: "main",
            targets: &[Some(wgpu::ColorTargetState {
                format: swap_chain_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions {
                ..Default::default()
            },
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let mut state = RendererState::new(&device, &pyramid_bind_group_layout);
    unsafe {
        RENDERER_STATE = Some(state);
        RENDERER_STATE_INIT.with(|init| {
            init.set(true);
        });
    }

    let state = get_renderer_state();

    // web-based rendering loop
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let closure = Closure::wrap(Box::new(move || {
        // Call your rendering function here
        render_frame(
            &state,
            &surface,
            &device,
            &queue,
            &render_pipeline,
            // &vertex_buffer,
            // &index_buffer,
            // &uniform_buffer,
            &camera_bind_group,
            // camera,
            &camera_uniform_buffer,
        );

        // Schedule the next frame
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>);

    *g.borrow_mut() = Some(closure);

    // Start the rendering loop
    request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn render_frame(
    state: &RendererState,
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    render_pipeline: &wgpu::RenderPipeline,
    // vertex_buffer: &wgpu::Buffer,
    // index_buffer: &wgpu::Buffer,
    camera_bind_group: &wgpu::BindGroup,
    camera_uniform_buffer: &wgpu::Buffer,
    // camera: &mut SimpleCamera,
) {
    // draw frames...
    let mut camera = get_camera();

    // Render a frame
    let frame = surface
        .get_current_texture()
        .expect("Failed to acquire next swap chain texture");
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    {
        let color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // draw calls...
        render_pass.set_pipeline(&render_pipeline);

        // In the render loop, update the uniform buffer if necessary
        // TODO: why uniform if updating every frame?
        camera.update();
        web_sys::console::log_1(&format!("Camera position: {:?}", camera.position).into());
        let camera_matrix = camera.view_projection_matrix;
        queue.write_buffer(
            &camera_uniform_buffer,
            0,
            bytemuck::cast_slice(camera_matrix.as_slice()),
        );

        // draw render state
        for pyramid in &state.pyramids {
            pyramid.update_uniform_buffer(&queue);
            render_pass.set_bind_group(0, &camera_bind_group, &[]);
            render_pass.set_bind_group(1, &pyramid.bind_group, &[]);

            render_pass.set_vertex_buffer(0, pyramid.vertex_buffer.slice(..));
            render_pass.set_index_buffer(pyramid.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..INDICES.len() as u32, 0, 0..1);
        }
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}

#[wasm_bindgen]
pub fn handle_key_press(key_code: String, is_pressed: bool) {
    let camera = get_camera();
    let state = get_renderer_state();

    println!("Key pressed: {}", key_code);
    web_sys::console::log_1(&format!("Key pressed (2): {}", key_code).into());

    match key_code.as_str() {
        "w" => {
            if is_pressed {
                // Handle the key press for W
                web_sys::console::log_1(&"Key W pressed".into());
                camera.position += camera.direction * 0.1;
            }
        }
        "s" => {
            if is_pressed {
                // Handle the key press for S
                web_sys::console::log_1(&"Key S pressed".into());
                camera.position -= camera.direction * 0.1;
            }
        }
        "a" => {
            if is_pressed {
                // Handle the key press for A
                web_sys::console::log_1(&"Key A pressed".into());
                let right = camera.direction.cross(&camera.up).normalize();
                camera.position -= right * 0.1;
            }
        }
        "d" => {
            if is_pressed {
                // Handle the key press for D
                web_sys::console::log_1(&"Key D pressed".into());
                let right = camera.direction.cross(&camera.up).normalize();
                camera.position += right * 0.1;
            }
        }
        "ArrowUp" => {
            if is_pressed {
                // Handle the key press for ArrowUp
                web_sys::console::log_1(&"Key ArrowUp pressed".into());
                state.pyramids[0].translate(Vector3::new(0.0, 0.1, 0.0));
                // test rotation
                // state.pyramids[0].rotate(Vector3::new(0.0, 0.1, 0.0));
                // test scale
                // state.pyramids[0].scale(Vector3::new(1.1, 1.1, 1.1));
            }
        }
        "ArrowDown" => {
            if is_pressed {
                // Handle the key press for ArrowDown
                web_sys::console::log_1(&"Key ArrowDown pressed".into());
                state.pyramids[0].translate(Vector3::new(0.0, -0.1, 0.0));
            }
        }
        "ArrowLeft" => {
            if is_pressed {
                // Handle the key press for ArrowLeft
                web_sys::console::log_1(&"Key ArrowLeft pressed".into());
                state.pyramids[0].translate(Vector3::new(-0.1, 0.0, 0.0));
            }
        }
        "ArrowRight" => {
            if is_pressed {
                // Handle the key press for ArrowRight
                web_sys::console::log_1(&"Key ArrowRight pressed".into());
                state.pyramids[0].translate(Vector3::new(0.1, 0.0, 0.0));
            }
        }
        _ => {
            // Handle any other keys if necessary
        }
    }

    camera.update();
}
