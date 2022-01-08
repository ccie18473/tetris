// TBlock is a structure which defines the geometric game piece
//
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TBlock {
    pub size: isize, // 2x2, 3x3, or 4x4 (size of square which completely
    // contains the piece)
    pub elements: [char; 16],
}

impl TBlock {
    pub fn new() -> Self {
        Self {
            size: 0,
            elements: [' '; 16],
        }
    }
    // Rotate -- rotate the block (only rotates one direction)
    //
    pub fn rotate(&mut self) {
        let mut temp_block: [char; 16];

        match self.size {
            4 => {
                temp_block = self.elements;
                for i in 0..4 {
                    for j in 0..4 {
                        temp_block[i * 4 + j] = self.elements[j * 4 + i];
                    }
                }
                self.elements = temp_block;
            }
            3 => {
                temp_block = self.elements;
                for i in 0..3 {
                    for j in 0..3 {
                        temp_block[(2 - i) * 4 + j] = self.elements[j * 4 + i];
                    }
                }
                self.elements = temp_block;
            }
            2 => {
                // no 2x2 blocks can rotate, so do nothing
                ()
            }
            _ => (),
        }
    }
}

// Define all the game pieces.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TBlocks {
    pub blocks: Vec<TBlock>,
}

impl TBlocks {
    pub fn new() -> Vec<TBlock> {
        let mut blocks = Vec::new();

        let block = TBlock {
            size: 4,
            elements: [
                ' ', '*', ' ', ' ', ' ', '*', ' ', ' ', ' ', '*', ' ', ' ', ' ', '*', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 3,
            elements: [
                ' ', '*', '*', ' ', ' ', '*', ' ', ' ', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 3,
            elements: [
                '*', '*', ' ', ' ', ' ', '*', ' ', ' ', ' ', '*', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 3,
            elements: [
                ' ', '*', ' ', ' ', '*', '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 3,
            elements: [
                '*', '*', ' ', ' ', ' ', '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 3,
            elements: [
                ' ', '*', '*', ' ', '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);
        let block = TBlock {
            size: 2,
            elements: [
                '*', '*', ' ', ' ', '*', '*', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
            ],
        };
        blocks.push(block);

        blocks
    }
}
