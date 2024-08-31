mod graphics;

use glfw::{Glfw, PWindow};
use graphics::Graphics;

pub struct Client {
    glfw: Glfw,
    window: PWindow,
    graphics: Graphics,
}

impl Client {
    pub fn new() -> Result<Self, &'static str> {
        let mut glfw = glfw::init_no_callbacks().unwrap();

        glfw.window_hint(glfw::WindowHint::Visible(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let (window, _) = glfw
            .create_window(800, 600, "BulletBlaster", glfw::WindowMode::Windowed)
            .expect("Failed to create window");

        Ok(Self {
            graphics: Graphics::new(&glfw, &window)?,
            glfw,
            window,
        })
    }
}
