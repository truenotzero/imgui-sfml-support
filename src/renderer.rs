use std::ffi::CString;

use imgui::Context;
pub use imgui_opengl_renderer::Renderer as SFMLRenderer;

pub fn init(imgui: &mut Context) -> SFMLRenderer {
    imgui_opengl_renderer::Renderer::new(imgui, |opengl_func_name| {
        let name = CString::new(opengl_func_name.as_bytes()).unwrap();
        unsafe { sfml::window::Context::get_function(&name) }
    })
}