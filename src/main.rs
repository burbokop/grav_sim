use std::{future::Future, sync::Arc};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::KeyCode,
    window::Window,
};

#[allow(unused_macros)]
#[cfg(target_arch = "wasm32")]
macro_rules! println {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into());
    }
}

#[cfg(not(target_arch = "wasm32"))]
use std::println;

/// Runs a future to completion. On native this blocks synchronously via pollster.
/// On wasm this spawns a local task so control returns to the browser immediately.
#[cfg(not(target_arch = "wasm32"))]
fn spawn(f: impl Future<Output = ()> + 'static) {
    pollster::block_on(f);
}

/// Runs a future to completion. On native this blocks synchronously via pollster.
/// On wasm this spawns a local task so control returns to the browser immediately.
#[cfg(target_arch = "wasm32")]
fn spawn(f: impl Future<Output = ()> + 'static) {
    wasm_bindgen_futures::spawn_local(f);
}

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Duration, Instant};
#[cfg(target_arch = "wasm32")]
use web_time::{Duration, Instant};

use crate::game_state::GameState;

mod game_state;

const TICK_RATE_HZ: u64 = 60;
const TIME_PER_TICK: Duration = Duration::from_nanos(1_000_000_000 / TICK_RATE_HZ);

struct WgpuState {
    window: Arc<Window>,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    vger: vger::Vger,
}

enum TriangleAction {
    Initialized(WgpuState),
}

#[expect(clippy::large_enum_variant)]
enum AppState {
    Uninitialized,
    Loading,
    Running(WgpuState),
}

struct App {
    game_state: GameState,
    proxy: EventLoopProxy<TriangleAction>,
    window: Option<Arc<Window>>,
    state: AppState,
    last_time: Instant,
    accumulated_time: Duration,
}

impl App {
    fn new(event_loop: &EventLoop<TriangleAction>) -> Self {
        Self {
            game_state: GameState::new(),
            proxy: event_loop.create_proxy(),
            window: None,
            state: AppState::Uninitialized,
            last_time: Instant::now(),
            accumulated_time: Duration::ZERO,
        }
    }
}

impl ApplicationHandler<TriangleAction> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if !matches!(self.state, AppState::Uninitialized) {
            return;
        }
        self.state = AppState::Loading;

        #[cfg_attr(
            not(target_arch = "wasm32"),
            expect(unused_mut, reason = "wasm32 re-assigns to specify canvas")
        )]
        let mut attributes = Window::default_attributes();

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;
            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("canvas")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();
            attributes = attributes.with_canvas(Some(canvas));
        }

        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .expect("Failed to create window"),
        );
        self.window = Some(window.clone());

        let proxy = self.proxy.clone();

        spawn(async move {
            let size = window.inner_size();
            println!("window.inner_size: {:?}", size);

            let instance = wgpu::Instance::default();

            let surface = instance.create_surface(window.clone()).unwrap();
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
                    force_fallback_adapter: false,
                    compatible_surface: Some(&surface),
                })
                .await
                .expect("Failed to find an appropriate adapter");

            let device_descriptor = wgpu::DeviceDescriptor::default();

            let (device, queue) = adapter
                .request_device(&device_descriptor)
                .await
                .expect("Failed to create device");

            let swapchain_capabilities = surface.get_capabilities(&adapter);
            let swapchain_format = swapchain_capabilities.formats[0];

            let config = surface
                .get_default_config(&adapter, size.width, size.height)
                .unwrap();
            surface.configure(&device, &config);

            let device = Arc::new(device);
            let queue = Arc::new(queue);

            let vger = vger::Vger::new(device.clone(), queue.clone(), swapchain_format);

            let _ = proxy.send_event(TriangleAction::Initialized(WgpuState {
                window,
                device,
                queue,
                surface,
                config,
                vger,
            }));
        });
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let current_time = Instant::now();
        let frame_time = current_time.duration_since(self.last_time);
        self.last_time = current_time;

        self.accumulated_time += frame_time.min(Duration::from_millis(250));
        while self.accumulated_time >= TIME_PER_TICK {
            self.game_state.update();
            self.accumulated_time -= TIME_PER_TICK;
        }

        if let AppState::Running(wgpu_state) = &mut self.state {
            let size = wgpu_state.window.inner_size();
            if wgpu_state.config.width != size.width || wgpu_state.config.height != size.height {
                wgpu_state.config.width = size.width;
                wgpu_state.config.height = size.height;
                wgpu_state
                    .surface
                    .configure(&wgpu_state.device, &wgpu_state.config);
            }

            wgpu_state.window.request_redraw();
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: TriangleAction) {
        match event {
            TriangleAction::Initialized(wgpu_state) => {
                wgpu_state.window.request_redraw();
                self.state = AppState::Running(wgpu_state);
                self.last_time = Instant::now();
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let AppState::Running(wgpu_state) = &mut self.state else {
            return;
        };

        match event {
            WindowEvent::Resized(new_size) => {
                wgpu_state.config.width = new_size.width;
                wgpu_state.config.height = new_size.height;
                wgpu_state
                    .surface
                    .configure(&wgpu_state.device, &wgpu_state.config);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            WindowEvent::RedrawRequested => {
                let AppState::Running(wgpu_state) = &mut self.state else {
                    return;
                };

                let frame = match wgpu_state.surface.get_current_texture() {
                    Ok(frame) => frame,
                    Err(e) => {
                        println!("Failed to acquire next surface texture: {:?}", e);
                        return;
                    }
                };

                let view = frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());
                let size = wgpu_state.window.inner_size();

                wgpu_state
                    .vger
                    .begin(size.width as f32, size.height as f32, 1.0);

                self.game_state
                    .render(&mut wgpu_state.vger, size.width as f32, size.height as f32);

                let desc = wgpu::RenderPassDescriptor {
                    label: Some("Vger Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                };

                let encoder = wgpu_state
                    .device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                wgpu_state.vger.encode(&desc);
                wgpu_state.queue.submit(Some(encoder.finish()));
                frame.present();
            }
            WindowEvent::Occluded(is_occluded) => {
                if !is_occluded {
                    if let Some(window) = &self.window {
                        window.request_redraw();
                    }
                }
            }
            WindowEvent::CloseRequested => event_loop.exit(),
            event => self.game_state.event(event),
        }
    }
}

pub fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }

    let event_loop = EventLoop::with_user_event().build().unwrap();

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    #[cfg_attr(target_arch = "wasm32", expect(unused_mut))]
    let mut app = App::new(&event_loop);

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::EventLoopExtWebSys;
        event_loop.spawn_app(app);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        event_loop.run_app(&mut app).unwrap();
    }
}
