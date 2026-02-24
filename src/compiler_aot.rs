// src/compiler_aot.rs - FalconCore AOT Compiler (Cranelift backend)
use cranelift::codegen::ir::{types, AbiParam, Function, UserFuncName};
use cranelift::codegen::Context;
use cranelift::frontend::{FunctionBuilder, FunctionBuilderContext, Variable};
use cranelift::prelude::*;
use cranelift_module::{DataId, Linkage, Module};
use cranelift_object::{ObjectModule, ObjectProduct};

pub fn compile_to_executable(ast: &Vec<Expr>) -> Result<Vec<u8>, String> {
    let mut module = ObjectModule::new();
    let mut ctx = module.make_context();
    let mut builder_ctx = FunctionBuilderContext::new();

    let sig = module.make_signature();
    let mut func = Function::with_name_signature(UserFuncName::user(0, 0), sig);
    let mut builder = FunctionBuilder::new(&mut func, &mut ctx.func, &mut builder_ctx);

    let mut variables = HashMap::new();

    // Simple "main" function
    let entry = builder.create_block();
    builder.append_block_params_for_function_params(entry);
    builder.switch_to_block(entry);
    builder.seal_block(entry);

    // Compile AST to Cranelift IR
    for expr in ast {
        compile_expr_to_cranelift(&mut builder, expr, &mut variables);
    }

    builder.finalize();

    let id = module.declare_function("main", Linkage::Export, &ctx.func.signature).unwrap();
    module.define_function(id, &ctx).unwrap();

    let product = module.finish();
    let bytes = product.object.write().unwrap();

    Ok(bytes)
}

fn compile_expr_to_cranelift(builder: &mut FunctionBuilder, expr: &Expr, variables: &mut HashMap<String, Variable>) {
    // Very simple implementation - expand later
    match expr {
        Expr::Number(n) => {
            let val = builder.ins().iconst(types::I64, *n as i64);
            builder.ins().return_(&[val]);
        }
        // Add more cases (print, add, if, loop etc.)
        _ => {}
    }
      }
