use super::processor::*;
use super::utils::*;

pub struct InstructionHandler {}

impl InstructionHandler {
    pub fn execute(processor: &mut Processor, nibbles: [char; 4]) {
        let mut instruction = String::new();
        for i in nibbles {
            instruction.push(i);
        }

        match nibbles[0] {
            '0' => match &nibbles[1..] {
                &['0', 'E', '0'] => processor.clear_screen(),
                &['0', 'E', 'E'] => InstructionHandler::sub_return(processor),
                _ => panic!("Instruction not recognized: {:?}", nibbles),
            }
            '1' => InstructionHandler::jump(processor, nibbles),
            '2' => InstructionHandler::sub_call(processor, nibbles),
            '6' => InstructionHandler::set_register(processor, nibbles),
            '7' => InstructionHandler::add_immediate(processor, nibbles),
            'A' => InstructionHandler::set_index(processor, nibbles),
            'D' => processor.draw_sprite(nibbles),
            _ => panic!("Instruction not recognized: {:?}", nibbles),
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

    fn set_register(processor: &mut Processor, nibbles: [char; 4]) {
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
}
