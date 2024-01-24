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
            '6' => InstructionHandler::set_register(processor, nibbles),
            '7' => InstructionHandler::add_immediate(processor, nibbles),
            'A' => InstructionHandler::set_index(processor, nibbles),
            'D' => processor.draw_sprite(nibbles),
            _ => panic!("Instruction not recognized: {:?}", nibbles),
        }
    }

    fn sub_return(processor: &mut Processor) {
        let return_addr = processor.stack.pop().expect("No return address on stack to return");
        InstructionHandler::jump(processor, );
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

    fn jump(processor: &mut Processor, nibbles: &[char]) {
        let mut address = nibbles[1].to_string();
        address.push(nibbles[2]);
        address.push(nibbles[3]);
        let address = usize::from_str_radix(&address, 16).expect("Failed to resolve jump address");
        processor.PC = address;
    }
}
