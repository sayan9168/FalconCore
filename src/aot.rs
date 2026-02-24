// src/aot.rs - FalconCore AOT Compiler (Cranelift backend - Hello World binary)
use cranelift::codegen::ir::{types, AbiParam, Function, UserFuncName};
use cranelift::codegen::Context;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectModule, ObjectProduct};
use std::fs::File;
use std::io::Write;

pub fn compile_hello_world() -> Result<(), String> {
    let mut module = ObjectModule::new();
    let mut ctx = module.make_context();

    // Signature for main()
    let mut sig = module.make_signature();
    sig.params.push(AbiParam::new(types::I32));
    sig.returns.push(AbiParam::new(types::I32));

    let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
    let mut builder_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut func, &mut ctx.func, &mut builder_ctx);

    let entry = builder.create_block();
    builder.append_block_params_for_function_params(entry);
    builder.switch_to_block(entry);
    builder.seal_block(entry);

    // Call puts("Hello from FalconCore AOT!")
    let hello = builder.ins().iconst(types::I64, 0x48656c6c6f2066726f6d2046616c636f6e436f726520414f5421); // "Hello from FalconCore AOT!"
    builder.ins().call(hello, &[]); // placeholder for puts

    let zero = builder.ins().iconst(types::I32, 0);
    builder.ins().return_(&[zero]);

    builder.finalize();

    let id = module.declare_function("main", Linkage::Export, &ctx.func.signature).unwrap();
    module.define_function(id, &ctx).unwrap();

    let product = module.finish();
    let bytes = product.object.write().unwrap();

    let mut file = File::create("falconcore_hello").map_err(|e| e.to_string())?;
    file.write_all(&bytes).map_err(|e| e.to_string())?;

    println!("AOT binary created: falconcore_hello");
    println!("Run with: ./falconcore_hello (chmod +x first)");

    Ok(())
  }
