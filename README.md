# rs-8chip-interpreter

This is my take on the famous CHIP-8 system and it's emulation.

# What has been implemented so far?

- 00e0 // Clear Display
- 1nnn // Jump
- 2nnn // Call Subroutine
- 6xnn // Set
- annn // Set Index
- dxyn // Draw
- 7xnn // Add
- fx33 // Binary-coded decimal conversion
- 3xnn // Skip
- 4xnn  // Skip
- 5xy0 // Skip
- 9xy0 // Skip
- 00ee // Return Subroutine
- 8xy0 // Copy
- 8xy1 // Binary OR
- 8xy2 // Binary AND
- 8xy3 // Logical XOR
- 8xy4 // Sum
- 8xy5 // Substract
- 8xy6 // Shift (Right)
- 8xy7 // Substract (reverse)
- 8xye // Shift (Left)
- fx55 // Store Memory
- fx65 // Load Memory