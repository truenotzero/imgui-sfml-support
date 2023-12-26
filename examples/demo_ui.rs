use imgui::Context;
use imgui_sfml_support::*;
use sfml::{graphics::{RenderWindow, RenderTarget, Color}, window::{Style, ContextSettings, Event}};

fn main() {
    let mut wnd = RenderWindow::new((800, 800), "Rust: SFML-ImGui", Style::CLOSE, &ContextSettings::default());

    let mut imgui = Context::create();
    let mut renderer = SFMLRenderer::init(&mut imgui);
    let mut platform = SFMLPlatform::init(&mut imgui, &wnd);


    while wnd.is_open() {
        while let Some(event) = wnd.poll_event() {
            platform.handle_event(&mut imgui, event);
            match event {
                Event::Closed => wnd.close(),
                _ => (),
            }


            platform.prepare_frame(&mut imgui);
            let ui = imgui.new_frame();
            // do your imgui work here
            ui.show_demo_window(&mut true);

            wnd.clear(Color::BLACK);
            wnd.reset_gl_states();

            renderer.render(&mut imgui, &mut wnd);

            wnd.display();
        }
    }
}
