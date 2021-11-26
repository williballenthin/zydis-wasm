use std::io::Write;
 
use wasm_bindgen::prelude::*;
use zydis::*;

#[wasm_bindgen]
pub fn disassemble(buf: Vec<u8>) -> String {
    let formatter = Formatter::new(FormatterStyle::INTEL).unwrap();
    let decoder = Decoder::new(MachineMode::LONG_64, AddressWidth::_64).unwrap();

    let mut buffer = [0u8; 200];
    let mut buffer = OutputBuffer::new(&mut buffer[..]);

    let mut w = Vec::new();

    let addr = 0x007FFFFFFF400000;
    for (instruction, ip) in decoder.instruction_iterator(&buf, addr) {
        formatter.format_instruction(&instruction, &mut buffer, Some(ip), None).unwrap();
        writeln!(&mut w, "0x{:016X} {}", ip, buffer).unwrap();
    }

    return String::from_utf8(w).unwrap();
}
