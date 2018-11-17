// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/elem.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 4
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0)
      (elem (i32.const 0))
      (elem (i32.const 0) 0 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 23

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
}
fn create_module_2() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_2(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 28

#[test]
fn test_module_2() {
    let result_object = create_module_2();
    // We group the calls together
    start_module_2(&result_object);
}
fn create_module_3() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_3(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 34

#[test]
fn test_module_3() {
    let result_object = create_module_3();
    // We group the calls together
    start_module_3(&result_object);
}
fn create_module_4() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (i32.const 0) 0)
      (elem (i32.const 3) 0)
      (elem (i32.const 7) 0)
      (elem (i32.const 5) 0)
      (elem (i32.const 3) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_4(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 43

#[test]
fn test_module_4() {
    let result_object = create_module_4();
    // We group the calls together
    start_module_4(&result_object);
}
fn create_module_5() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 9) 0)
      (elem (i32.const 3) 0)
      (elem (i32.const 7) 0)
      (elem (i32.const 3) 0)
      (elem (i32.const 5) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_5(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 69

#[test]
fn test_module_5() {
    let result_object = create_module_5();
    // We group the calls together
    start_module_5(&result_object);
}
fn create_module_6() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 7
        call_indirect (type 0))
      (func (;3;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"call-7\" (func 2))
      (export \"call-9\" (func 3))
      (elem (i32.const 7) 0)
      (elem (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_6(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 83
fn c6_l83_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c6_l83_action_invoke");
    let func_index = match result_object.module.info.exports.get("call-7") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 65 as i32);
}

// Line 84
fn c7_l84_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c7_l84_action_invoke");
    let func_index = match result_object.module.info.exports.get("call-9") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 66 as i32);
}

// Line 88

#[test]
fn test_module_6() {
    let result_object = create_module_6();
    // We group the calls together
    start_module_6(&result_object);
    c6_l83_action_invoke(&result_object);
    c7_l84_action_invoke(&result_object);
}
fn create_module_7() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (func (;0;) (type 0))
      (table (;0;) 10 anyfunc)
      (elem (i32.const 9) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_7(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 93

#[test]
fn test_module_7() {
    let result_object = create_module_7();
    // We group the calls together
    start_module_7(&result_object);
}
fn create_module_8() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 9) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_8(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 99

#[test]
fn test_module_8() {
    let result_object = create_module_8();
    // We group the calls together
    start_module_8(&result_object);
}
fn create_module_9() -> ResultObject {
    let module_str = "(module
      (table (;0;) 0 anyfunc)
      (elem (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_9(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 103

#[test]
fn test_module_9() {
    let result_object = create_module_9();
    // We group the calls together
    start_module_9(&result_object);
}
fn create_module_10() -> ResultObject {
    let module_str = "(module
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (elem (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_10(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 108

#[test]
fn test_module_10() {
    let result_object = create_module_10();
    // We group the calls together
    start_module_10(&result_object);
}
fn create_module_11() -> ResultObject {
    let module_str = "(module
      (table (;0;) 0 0 anyfunc)
      (elem (i32.const 0)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_11(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 113

#[test]
fn test_module_11() {
    let result_object = create_module_11();
    // We group the calls together
    start_module_11(&result_object);
}
fn create_module_12() -> ResultObject {
    let module_str = "(module
      (table (;0;) 20 anyfunc)
      (elem (i32.const 20)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_12(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 118

#[test]
fn test_module_12() {
    let result_object = create_module_12();
    // We group the calls together
    start_module_12(&result_object);
}
fn create_module_13() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_13(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 124

#[test]
fn test_module_13() {
    let result_object = create_module_13();
    // We group the calls together
    start_module_13(&result_object);
}
fn create_module_14() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 100 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 0) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_14(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 130

#[test]
fn test_module_14() {
    let result_object = create_module_14();
    // We group the calls together
    start_module_14(&result_object);
}
fn create_module_15() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 1) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_15(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 136

#[test]
fn test_module_15() {
    let result_object = create_module_15();
    // We group the calls together
    start_module_15(&result_object);
}
fn create_module_16() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func))
      (import \"spectest\" \"table\" (table (;0;) 0 30 anyfunc))
      (func (;0;) (type 0))
      (elem (i32.const 1) 0))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_16(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 145

// Line 154

// Line 163

// Line 172

// Line 180

// Line 188

// Line 197

// Line 205

// Line 214

// Line 222

// Line 231

// Line 239

// Line 250
#[test]
fn c30_l250_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 9, 7, 1, 0, 65, 0, 11, 1, 0, 10, 4, 1, 2, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 260
#[test]
fn c31_l260_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 6, 1, 0, 66, 0, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 268
#[test]
fn c32_l268_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 65, 0, 104, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 276
#[test]
fn c33_l276_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 5, 1, 0, 1, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 284
#[test]
fn c34_l284_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 1, 65, 0, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 292
#[test]
fn c35_l292_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 4, 4, 1, 112, 0, 1, 9, 7, 1, 0, 65, 0, 1, 11, 0];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 307

#[test]
fn test_module_16() {
    let result_object = create_module_16();
    // We group the calls together
    start_module_16(&result_object);
}
fn create_module_17() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"call-overwritten\" (func 2))
      (elem (i32.const 9) 0)
      (elem (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_17(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 318
fn c37_l318_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c37_l318_action_invoke");
    let func_index = match result_object.module.info.exports.get("call-overwritten") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 66 as i32);
}

// Line 320

#[test]
fn test_module_17() {
    let result_object = create_module_17();
    // We group the calls together
    start_module_17(&result_object);
    c37_l318_action_invoke(&result_object);
}
fn create_module_18() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (import \"spectest\" \"table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (export \"call-overwritten-element\" (func 2))
      (elem (i32.const 9) 0)
      (elem (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_18(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 331
fn c39_l331_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c39_l331_action_invoke");
    let func_index = match result_object.module.info.exports.get("call-overwritten-element") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 66 as i32);
}

// Line 335

#[test]
fn test_module_18() {
    let result_object = create_module_18();
    // We group the calls together
    start_module_18(&result_object);
    c39_l331_action_invoke(&result_object);
}
fn create_module_19() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (func (;0;) (type 0) (result i32)
        i32.const 65)
      (func (;1;) (type 0) (result i32)
        i32.const 66)
      (func (;2;) (type 0) (result i32)
        i32.const 7
        call_indirect (type 0))
      (func (;3;) (type 0) (result i32)
        i32.const 8
        call_indirect (type 0))
      (func (;4;) (type 0) (result i32)
        i32.const 9
        call_indirect (type 0))
      (table (;0;) 10 anyfunc)
      (export \"shared-table\" (table 0))
      (export \"call-7\" (func 2))
      (export \"call-8\" (func 3))
      (export \"call-9\" (func 4))
      (elem (i32.const 8) 0)
      (elem (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_19(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 353

// Line 360

#[test]
fn test_module_19() {
    let result_object = create_module_19();
    // We group the calls together
    start_module_19(&result_object);
}
fn create_module_20() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (import \"module1\" \"shared-table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0) (result i32)
        i32.const 67)
      (func (;1;) (type 0) (result i32)
        i32.const 68)
      (elem (i32.const 7) 0)
      (elem (i32.const 8) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_20(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 374

#[test]
fn test_module_20() {
    let result_object = create_module_20();
    // We group the calls together
    start_module_20(&result_object);
}
fn create_module_21() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (import \"module1\" \"shared-table\" (table (;0;) 10 anyfunc))
      (func (;0;) (type 0) (result i32)
        i32.const 69)
      (func (;1;) (type 0) (result i32)
        i32.const 70)
      (elem (i32.const 8) 0)
      (elem (i32.const 9) 1))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_21(result_object: &ResultObject) {
    result_object.instance.start();
}

#[test]
fn test_module_21() {
    let result_object = create_module_21();
    // We group the calls together
    start_module_21(&result_object);
}