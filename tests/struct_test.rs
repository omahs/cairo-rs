use cairo_rs::{
    hint_processor::builtin_hint_processor::builtin_hint_processor_definition::BuiltinHintProcessor,
    vm::vm_core::VirtualMachine,
};
use num_bigint::{BigInt, Sign};
use std::path::Path;

use cairo_rs::{
    types::program::Program,
    vm::{runners::cairo_runner::CairoRunner, trace::trace_entry::RelocatedTraceEntry},
};

#[test]
fn struct_integration_test() {
    let program = Program::new(Path::new("cairo_programs/struct.json"), "main")
        .expect("Failed to deserialize program");
    let hint_processor = BuiltinHintProcessor::new_empty();
    let mut cairo_runner = CairoRunner::new(&program, &hint_processor).unwrap();
    let mut vm = VirtualMachine::new(
        BigInt::new(Sign::Plus, vec![1, 0, 0, 0, 0, 0, 17, 134217728]),
        true,
    );
    cairo_runner.initialize_builtins(&mut vm).unwrap();
    cairo_runner.initialize_segments(&mut vm, None);
    let end = cairo_runner.initialize_main_entrypoint(&mut vm).unwrap();

    assert!(
        cairo_runner.initialize_vm(&mut vm) == Ok(()),
        "Execution failed"
    );
    assert!(
        cairo_runner.run_until_pc(end, &mut vm) == Ok(()),
        "Execution failed"
    );
    assert!(cairo_runner.relocate(&mut vm) == Ok(()), "Execution failed");
    let relocated_entry = RelocatedTraceEntry {
        pc: 1,
        ap: 4,
        fp: 4,
    };

    assert_eq!(cairo_runner.relocated_trace, Some(vec![relocated_entry]));
}
