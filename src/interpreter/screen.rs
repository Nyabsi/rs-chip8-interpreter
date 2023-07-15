pub struct Screen {
    buffer: [u8; 32 * 64 * 4],
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            buffer: [0; 32 * 64 * 4],
        }
    }

    pub fn convert_buffer(&mut self, screen_buffer: &[[u8; 64]; 32]) -> &[u8] {
        for y in 0..32 {
            for x in 0..64 {
                let pixel = screen_buffer[y][x];
                let offset = ((y * 64) + x) * 4;
                self.buffer[offset] = 0xFF;
                self.buffer[offset + 1] = 0xFF;
                self.buffer[offset + 2] = 0xFF;
                self.buffer[offset + 3] = pixel;
            }
        }
        return &self.buffer
    }
}
