use std::ffi::CString;

use imgui::Context;
use imgui_opengl_renderer::Renderer;
use sfml::graphics::RenderTarget;

pub struct SFMLRenderer {
    backend: Renderer,
}

impl SFMLRenderer {
    pub fn init(imgui: &mut Context) -> Self {
        let backend = imgui_opengl_renderer::Renderer::new(imgui, |opengl_func_name| {
            let name = CString::new(opengl_func_name.as_bytes()).unwrap();
            unsafe { sfml::window::Context::get_function(&name) }
        });

        Self { backend }
    }

    pub fn render(&mut self, imgui: &mut Context, tgt: &mut dyn RenderTarget) {
        tgt.push_gl_states();
        self.backend.render(imgui);
        tgt.pop_gl_states();
    }
}
