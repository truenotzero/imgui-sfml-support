# ImGui-SFML-Support
A SFML backend for Rust's ImGui port

## Code example
```rust
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

```

## Installation instructions
### IMPORTANT NOTE: This crate won't work out of the box!  
As of writing this, please note that the official Rust port of SFML can't be used.  
To fix this, change in your `Cargo.toml` under `[dependencies]`:
```toml
# change this line
sfml = "0.21.0"
# to this line
sfml = { git = "https://github.com/truenotzero/rust-sfml.git", branch = "context_get_function" }
```
I have submitted a pull request in order to fix the issue.  
This is needed because the port doesn't provide `sf::Context::getFunction()` which is required by the renderer used.

## Credits
[imgui-opengl-renderer](https://github.com/michaelfairley/rust-imgui-opengl-renderer): For providing the renderer  
[imgui-sfml](https://github.com/SFML/imgui-sfml): A C++ library which does the same thing

## Licensing
Code is MIT licensed
