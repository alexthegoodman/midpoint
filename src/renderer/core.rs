use winit::{
    dpi::LogicalSize,
    event::*,
    event_loop::{self, ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

// use raw_window_handle::{
//     HasDisplayHandle, HasRawWindowHandle, HasWindowHandle, RawWindowHandle, WebCanvasWindowHandle,
//     WebDisplayHandle, WebWindowHandle,
// };
// use std::ptr::NonNull;
// use std::sync::Arc;
use wasm_bindgen::prelude::*;

use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub async fn init_wgpu() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("scene-canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    // let context: WgpuRenderingContext = canvas.get_context("webgpu")?.into();

    // context.configure(wgpu::SurfaceConfiguration {
    //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //     format: wgpu::TextureFormat::Bgra8UnormSrgb,
    //     width: canvas.width(),
    //     height: canvas.height(),
    //     present_mode: wgpu::PresentMode::Fifo,
    // });

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

    // SurfaceTarget::from(value)

    // let surface = unsafe { instance.create_surface(arc_canvas) };

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
            .create_surface(&winit_window)
            .expect("Couldn't create GPU Surface")
    };

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or("Failed to find an appropriate adapter")?;

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

    event_loop
        .run(move |event, target| {
            if let Event::WindowEvent {
                window_id: _,
                event,
            } = event
            {
                match event {
                    // WindowEvent::CursorMoved { position, .. } => {
                    //     // Update the mouse position
                    //     // println!("Mouse Position: {:?}", position);
                    //     mouse_position = (position.x as f64, position.y as f64);
                    // }
                    // WindowEvent::MouseInput {
                    //     state: ElementState::Pressed,
                    //     button: MouseButton::Left,
                    //     ..
                    // } => {
                    //     let window_size = (size.width as f64, size.height as f64);
                    //     handle_click(window_size, mouse_position, &buttons, &labels);
                    // }
                    WindowEvent::Resized(new_size) => {
                        // Reconfigure the surface with the new size
                        config.width = new_size.width.max(1);
                        config.height = new_size.height.max(1);
                        surface.configure(&device, &config);
                        // On macos the window needs to be redrawn manually after resizing
                        // window.request_redraw();
                    }
                    WindowEvent::RedrawRequested => {
                        // draw frames...

                        // Render a frame
                        let frame = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Render Encoder"),
                            });

                        {
                            let color = wgpu::Color {
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            };
                            let _render_pass =
                                encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    WindowEvent::CloseRequested => target.exit(),
                    _ => {}
                };
            }
        })
        .unwrap();

    Ok(())
}
