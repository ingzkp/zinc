extern crate franklin_crypto;

use crate::{RuntimeError, VirtualMachine, VMInstruction, ElementOperator, Element};
use zrust_bytecode::instructions::Lt;

impl<E, O> VMInstruction<E, O> for Lt
    where E: Element, O: ElementOperator<E>
{
    fn execute(&mut self, vm: &mut VirtualMachine<E, O>) -> Result<(), RuntimeError> {
        let left = vm.stack_pop()?;
        let right = vm.stack_pop()?;

        let lt = vm.get_operator().lt(left, right)?;

        vm.stack_push(lt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::testing_utils;
    use zrust_bytecode::*;
    use num_bigint::BigInt;

    #[test]
    fn test_lt() -> Result<(), RuntimeError> {
        let mut bytecode = testing_utils::create_instructions_vec();
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Lt));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Lt));
        bytecode.push(Box::new(Push { value: BigInt::from(2) }));
        bytecode.push(Box::new(Push { value: BigInt::from(1) }));
        bytecode.push(Box::new(Lt));

        let mut vm = testing_utils::create_vm();
        vm.run(bytecode.as_mut_slice())?;

        testing_utils::assert_stack_eq(&vm, &[1, 0, 0]);

        Ok(())
    }
}
