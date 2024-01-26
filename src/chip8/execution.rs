use super::processor::*;
use super::utils::*;
use rand::Rng;
const REAL_WIDTH: usize = 64;
const REAL_HEIGHT: usize = 32;
const BUFFER_CHUNK_SIZE: usize = 4;

#[derive(PartialEq, Clone, Copy)]
pub enum PixelState {
    Off,
    On,
}
use PixelState::*;

pub struct InstructionHandler {}

impl InstructionHandler {
    pub fn execute(processor: &mut Processor, nibbles: [char; 4]) {
        let mut instruction = String::new();
        for i in nibbles {
            instruction.push(i);
        }

        match nibbles[0] {
            '0' => match &nibbles[1..] {
                &['0', 'E', '0'] => InstructionHandler::clear_screen(processor),
                &['0', 'E', 'E'] => InstructionHandler::sub_return(processor),
                _ => panic!("Instruction not recognized: {:?}", nibbles),
            }
            '1' => InstructionHandler::jump(processor, nibbles),
            '2' => InstructionHandler::sub_call(processor, nibbles),
            '3' => InstructionHandler::skip_immediate_equal(processor, nibbles),
            '4' => InstructionHandler::skip_immediate_not_equal(processor, nibbles),
            '5' => InstructionHandler::skip_registers_equal(processor, nibbles),
            '6' => InstructionHandler::set_register_immediate(processor, nibbles),
            '7' => InstructionHandler::add_immediate(processor, nibbles),
            '8' => match nibbles[3] {
                '0' => InstructionHandler::set_register_register(processor, nibbles),
                '1' => InstructionHandler::or(processor, nibbles),
                '2' => InstructionHandler::and(processor, nibbles),
                '3' => InstructionHandler::xor(processor, nibbles),
                '4' => InstructionHandler::add_register(processor, nibbles),
                '5' => InstructionHandler::subtraction_regular(processor, nibbles),
                '6' => InstructionHandler::shift_right(processor, nibbles),
                '7' => InstructionHandler::subtraction_reversed(processor, nibbles),
                'E' => InstructionHandler::shift_left(processor, nibbles),
                _ => panic!("Instruction not recognized: {:?}", nibbles),
            }
            '9' => InstructionHandler::skip_registers_not_equal(processor, nibbles),
            'A' => InstructionHandler::set_index(processor, nibbles),
            'B' => InstructionHandler::jump_with_offset(processor, nibbles),
            'C' => InstructionHandler::random(processor, nibbles),
            'D' => InstructionHandler::draw_sprite(processor, nibbles),
            'E' => match nibbles[2] {
                '9' => (),
                _ => (),

            }
            _ => panic!("Instruction not recognized: {:?}", nibbles),
        }
    }

    fn random(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let mask = Utils::resolve_hex(&nibbles[2..4]) as u8;
        let mut rand_gen = rand::thread_rng();
        processor.V_REGS[register_x as usize] = rand_gen.gen::<u8>() & mask;
    }

    fn jump_with_offset(processor: &mut Processor, nibbles: [char; 4]) {
        let mut address = nibbles[1].to_string();
        address.push(nibbles[2]);
        address.push(nibbles[3]);
        let mut address = usize::from_str_radix(&address, 16).expect("Failed to resolve jump address");
        address += processor.V_REGS[0] as usize;
        processor.PC = address;
    }

    fn shift_left(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_y = processor.V_REGS[register_y as usize];
        processor.VF = (value_y & 128) >> 7;
        processor.V_REGS[register_x as usize] = value_y << 1;
    }

    fn shift_right(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_y = processor.V_REGS[register_y as usize];
        processor.VF = value_y & 1;
        processor.V_REGS[register_x as usize] = value_y >> 1;
    }

    fn subtraction_reversed(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        if value_y >= value_x {
            processor.VF = 1;
        } else {
            processor.VF = 0;
        }
        processor.V_REGS[register_x as usize] = value_y - value_x;
        
    }

    fn subtraction_regular(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        if value_x >= value_y {
            processor.VF = 1;
        } else {
            processor.VF = 0;
        }
        processor.V_REGS[register_x as usize] = value_x - value_y;
    }

    fn add_register(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        if ((value_x as u16) + (value_y as u16)) > 255 {
            processor.VF = 1;
        } else {
            processor.VF = 0;
        }
        processor.V_REGS[register_x as usize] = value_x.wrapping_add(value_y as u8);
    }

    fn xor(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        processor.V_REGS[register_x as usize] = value_x ^ value_y;
    }

    fn and(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        processor.V_REGS[register_x as usize] = value_x & value_y;
    }

    fn or(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        processor.V_REGS[register_x as usize] = value_x | value_y;
    }

    fn set_register_register(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_y = processor.V_REGS[register_y as usize];
        processor.V_REGS[register_x as usize] = value_y;
    }

    fn skip_registers_not_equal(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        if value_x != value_y {
            processor.PC += 2;
        }
    }

    fn skip_registers_equal(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let value_x = processor.V_REGS[register_x as usize];
        let value_y = processor.V_REGS[register_y as usize];
        if value_x == value_y {
            processor.PC += 2;
        }
    }

    fn skip_immediate_equal(processor: &mut Processor, nibbles: [char; 4]) {
        let register = Utils::resolve_hex(&[nibbles[1]]);
        let value = Utils::resolve_hex(&nibbles[2..4]);
        if processor.V_REGS[register as usize] == (value as u8) {
            processor.PC += 2;
        }
    }

    fn skip_immediate_not_equal(processor: &mut Processor, nibbles: [char; 4]) {
        let register = Utils::resolve_hex(&[nibbles[1]]);
        let value = Utils::resolve_hex(&nibbles[2..4]);
        if processor.V_REGS[register as usize] != (value as u8) {
            processor.PC += 2;
        }
    }

    fn sub_call(processor: &mut Processor, nibbles: [char; 4]) {
        let mut address = ['0'; 3];
        let address_chars: &[char] = &format!("{:0<3X}", processor.PC).chars().collect::<Vec<_>>().into_boxed_slice();
        address.copy_from_slice(&address_chars);
        processor.stack.push(address);

        let jump_address = Utils::resolve_hex(&nibbles[1..]);
        processor.PC = jump_address as usize;
    }

    fn sub_return(processor: &mut Processor) {
        let return_addr = processor.stack.pop().expect("Stack empty on return");
        let mut jump_addr = ['0'; 4];
        jump_addr[1..].copy_from_slice(&return_addr);
        InstructionHandler::jump(processor, jump_addr);
    }

    fn set_register_immediate(processor: &mut Processor, nibbles: [char; 4]) {
        let register = Utils::resolve_hex(&[nibbles[1]]);
        let value = Utils::resolve_hex(&nibbles[2..4]);
        processor.V_REGS[register as usize] = value as u8;
    }

    fn set_index(processor: &mut Processor, nibbles: [char; 4]) {
        let value = Utils::resolve_hex(&nibbles[1..4]);
        processor.I = value;
    }

    fn add_immediate(processor: &mut Processor, nibbles: [char; 4]) {
        let register = Utils::resolve_hex(&[nibbles[1]]);
        let value = Utils::resolve_hex(&nibbles[2..4]);
        processor.V_REGS[register as usize] =
            processor.V_REGS[register as usize].wrapping_add(value as u8);
    }

    fn jump(processor: &mut Processor, nibbles: [char; 4]) {
        let mut address = nibbles[1].to_string();
        address.push(nibbles[2]);
        address.push(nibbles[3]);
        let address = usize::from_str_radix(&address, 16).expect("Failed to resolve jump address");
        processor.PC = address;
    }

    pub fn draw_sprite(processor: &mut Processor, nibbles: [char; 4]) {
        let register_x = Utils::resolve_hex(&[nibbles[1]]);
        let register_y = Utils::resolve_hex(&[nibbles[2]]);
        let height = Utils::resolve_hex(&[nibbles[3]]);
        let x = processor.V_REGS[register_x as usize] % (REAL_WIDTH as u8);
        let y = processor.V_REGS[register_y as usize] % (REAL_HEIGHT as u8);
        let base_address = processor.I;

        processor.VF = 0;
        for i in 0..height {
            let sprite_byte = *processor
                .memory
                .get((base_address + i) as usize)
                .expect("Trying to access out-of-bounds memory");

            for j in 0..8 {
                let sprite_state = (sprite_byte & (128 >> j)) >> (7 - j);

                let pixel_state = InstructionHandler::get_pixel(processor, (x + j) as usize, (y + (i as u8)) as usize);
                if pixel_state.is_none() {
                    continue;
                }
                let pixel_state = pixel_state.unwrap() as u8;

                if sprite_state == 1 && pixel_state == 1 {
                    processor.VF = 1;
                }
                let final_state = if sprite_state ^ pixel_state == 1 {
                    On
                } else {
                    Off
                };
                InstructionHandler::set_pixel(processor, (x + j) as usize, (y + (i as u8)) as usize, final_state);
            }
        }

        processor.pixels
            .as_mut()
            .unwrap()
            .render()
            .expect("Failed to render pixel buffer on sprite draw");
    }

    pub fn get_pixel(processor: &mut Processor, x: usize, y: usize) -> Option<PixelState> {
        if x >= REAL_WIDTH || y >= REAL_HEIGHT {
            println!("NOTE: Attempting to fetch pixel beyond screen.");
            return None;
        }

        let pixel = processor
            .pixels
            .as_ref()
            .unwrap()
            .frame()
            .chunks_exact(BUFFER_CHUNK_SIZE)
            .nth(y * REAL_WIDTH + x)
            .unwrap();

        if pixel[0] == 0xff {
            Some(On)
        } else {
            Some(Off)
        }
    }

    pub fn set_pixel(processor: &mut Processor, x: usize, y: usize, state: PixelState) {
        if x >= REAL_WIDTH || y >= REAL_HEIGHT {
            println!("NOTE: Attempting to set pixel beyond screen.");
            return;
        }

        let pixel = processor
            .pixels
            .as_mut()
            .unwrap()
            .frame_mut()
            .chunks_exact_mut(BUFFER_CHUNK_SIZE)
            .nth(y * REAL_WIDTH + x)
            .unwrap();
        let color_state = state as u8;
        pixel[0] = 0xff * color_state;
        pixel[1] = 0xff * color_state;
        pixel[2] = 0xff * color_state;
        pixel[3] = 0xff;
    }

    pub fn clear_screen(processor: &mut Processor) {
        for pixel in processor
            .pixels
            .as_mut()
            .unwrap()
            .frame_mut()
            .chunks_exact_mut(BUFFER_CHUNK_SIZE)
        {
            pixel[0] = 0x00;
            pixel[1] = 0x00;
            pixel[2] = 0x00;
            pixel[3] = 0xff;
        }

        processor.pixels
            .as_ref()
            .unwrap()
            .render()
            .expect("Failed to render pixel buffer on screen clear");
    }
}
