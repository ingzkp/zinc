//!
//! A semantic analyzer test.
//!

#![cfg(test)]

use num_bigint::BigInt;

use zinc_bytecode::Add;
use zinc_bytecode::Call;
use zinc_bytecode::Exit;
use zinc_bytecode::Instruction;
use zinc_bytecode::PopStore;
use zinc_bytecode::PushConst;
use zinc_bytecode::Return;

#[test]
fn test() {
    let input = r#"
enum First {
    A = 42,
    B = 512,
}

enum Second {
    A = 25,
    B = 314,
}

fn main() {
    let result = First::B + Second::A;
}
"#;

    let expected = Ok(vec![
        Instruction::Call(Call::new(2, 0)),
        Instruction::Exit(Exit::new(0)),
        Instruction::PushConst(PushConst::new(BigInt::from(25), false, 16)),
        Instruction::PushConst(PushConst::new(BigInt::from(512), false, 16)),
        Instruction::Add(Add),
        Instruction::PopStore(PopStore::new(0)),
        Instruction::Return(Return::new(0)),
    ]);

    let result = super::instructions(input);

    assert_eq!(expected, result);
}
