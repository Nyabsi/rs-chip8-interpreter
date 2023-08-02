// MIT License
// 
// Copyright (c) 2023 LumenTuoma
// 
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// 
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// 
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// UTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
            .window("CHIP-8 Interpreter", WIDTH * SCALE_FACTOR, HEIGHT * SCALE_FACTOR)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        Self {
            canvas: canvas,
        }
    }

    pub fn draw(&mut self, pixels: &[[bool; WIDTH as usize]; HEIGHT as usize]) {
        self.canvas.clear();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if pixels[y as usize][x as usize] {
                    // Foreground
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    // Background
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

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