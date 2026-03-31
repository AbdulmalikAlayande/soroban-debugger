use super::DebuggerEngine;

fn create_test_engine() -> DebuggerEngine {
    let wasm_bytes = vec![
        0x00, 0x61, 0x73, 0x6d, // WASM magic
        0x01, 0x00, 0x00, 0x00, // WASM version
    ];
    let executor = crate::runtime::executor::ContractExecutor::new(wasm_bytes).unwrap();
    DebuggerEngine::new(executor, vec![])
}

#[test]
fn engine_starts_unpaused() {
    let engine = create_test_engine();
    assert!(!engine.is_paused());
}

#[test]
fn no_source_location_without_instruction_state() {
    let engine = create_test_engine();
    assert!(engine.current_source_location().is_none());
}
