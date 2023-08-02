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

use std::collections::HashMap;
use std::collections::HashSet;

use sdl2::keyboard::Keycode;

pub struct Keypad {
    key_map: HashMap<u8, Keycode>,
    keys_active: HashMap<Keycode, bool>,
    prev_keys: HashSet<Keycode>
}

// TODO: if you can improve this, please do. It was quite rushed because I just had to get it working.
impl Keypad {

    // dummy
    pub fn new() -> Self {
        Self {
            key_map: HashMap::from([
                (0x0, Keycode::X),
                (0x1, Keycode::Num1),
                (0x2, Keycode::Num2),
                (0x3, Keycode::Num3),
                (0x4, Keycode::Q),
                (0x5, Keycode::W),
                (0x6, Keycode::E),
                (0x7, Keycode::A),
                (0x8, Keycode::S),
                (0x9, Keycode::D),
                (0xA, Keycode::Z),
                (0xB, Keycode::C),
                (0xC, Keycode::Num4),
                (0xD, Keycode::R),
                (0xE, Keycode::F),
                (0xF, Keycode::V),
            ]),                   
            keys_active: HashMap::from([
                (Keycode::X, false),
                (Keycode::Num1, false),
                (Keycode::Num2, false),
                (Keycode::Num3, false),
                (Keycode::Q, false),
                (Keycode::W, false),
                (Keycode::F, false),
                (Keycode::A, false),
                (Keycode::S, false),
                (Keycode::D, false),
                (Keycode::Z, false),
                (Keycode::C, false),
                (Keycode::Num4, false),
                (Keycode::R, false),
                (Keycode::E, false),
                (Keycode::V, false),
            ]),
            prev_keys: HashSet::from([])      
        }
    }

    pub fn is_key_down(&mut self, e: &sdl2::EventPump) {

        let keys = e
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let new_keys: HashSet<Keycode> = &keys - &self.prev_keys;
        let old_keys: HashSet<Keycode> = &self.prev_keys - &keys;

        for new_key in new_keys.iter() {
            *self.keys_active.get_mut(new_key).unwrap() = true;
        }

        for old_key in old_keys.iter() {
            *self.keys_active.get_mut(old_key).unwrap() = false;
        }

        self.prev_keys = keys;
    }

    pub fn is_any_key_down_emulator(&mut self) -> (u8, bool) {
        // TODO: implement
        return (0x0, false);
    }

    pub fn is_key_down_emulator(&mut self, key_code: u8) -> bool {
        let emu_key = self.key_map[&key_code];
        return self.keys_active[&emu_key];
    }

}