use minifb::{Result, Window, WindowOptions};

use crate::buffer::Buffer;

pub struct App<'a, T> {
    buffer: Buffer,
    window: Window,
    width: usize,
    height: usize,
    cb: Option<RenderCallback<T>>,
    on_init: Option<RenderCallback<T>>,
    state: &'a mut T,
}

type RenderCallback<T> = fn(&mut Buffer, &Window, &mut T);

impl<'a, T> App<'a, T> {
    pub fn create(
        width: usize,
        height: usize,
        state: &'a mut T,
        on_init: Option<RenderCallback<T>>,
    ) -> Result<Self> {
        let window = Window::new("App", width, height, WindowOptions::default())?;

        let mut buffer = Buffer::new(width, height);
        buffer.clear(0x00_000000);

        Ok(Self {
            buffer,
            window,
            width,
            height,
            cb: None,
            state,
            on_init,
        })
    }

    pub fn set_fps(&mut self, fps: usize) -> &mut Self {
        self.window.set_target_fps(fps);
        return self;
    }

    pub fn set_callback(&mut self, cb: RenderCallback<T>) -> &mut Self {
        self.cb = Some(cb);
        return self;
    }

    pub fn on_init(&mut self, on_init: RenderCallback<T>) -> &mut Self {
        self.on_init = Some(on_init);
        return self;
    }

    pub fn run(&mut self) {
        if let Some(on_init) = self.on_init.take() {
            on_init(&mut self.buffer, &self.window, &mut self.state);
        }

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            if let Some(cb) = self.cb {
                (cb)(&mut self.buffer, &self.window, &mut self.state);
                self.window
                    .update_with_buffer(&self.buffer.buffer(), self.width, self.height)
                    .unwrap();
            }
        }
    }

    pub fn run_with_callback(&mut self, cb: RenderCallback<T>) {
        self.set_callback(cb);
        self.run();
    }

    pub fn hide_cursor(&mut self) -> &mut Self {
        self.window.set_cursor_visibility(false);
        self
    }

    pub fn show_cursor(&mut self) -> &mut Self {
        self.window.set_cursor_visibility(true);
        self
    }
}
