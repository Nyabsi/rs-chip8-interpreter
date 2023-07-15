use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const SCALE_FACTOR: u32 = 10;

pub struct Display {
    canvas: Canvas<Window>
}

impl Display{
    pub fn new(sdl_context: &Sdl) -> Self {

        let video = sdl_context.video().unwrap();

        let window = video
            .window("SDL2 Window", WIDTH * SCALE_FACTOR, HEIGHT * SCALE_FACTOR)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Self {
            canvas: canvas,
            // pixels: [[false; WIDTH as usize]; HEIGHT as usize],
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH as usize]; HEIGHT as usize], clamp_pos: i32, clamp_size: u32) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y as usize][x as usize] {
                    // Foreground
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    // Background
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                // x, y, w, h
                let x1 = (x * SCALE_FACTOR) as i32;
                let y1 = (y * SCALE_FACTOR) as i32;
                let x2 = ((x + 1) * SCALE_FACTOR) as u32;
                let y2 = ((y + 1) * SCALE_FACTOR) as u32;

                self.canvas.fill_rect(Rect::new(x1, y1, x2, y2)).unwrap();
            }
        }
        self.canvas.present();
    }
}