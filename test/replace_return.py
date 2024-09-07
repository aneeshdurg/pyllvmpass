"""LLVM Module pass that replace all return statements with `return 1`"""

import llvmcpy.llvm as cllvm

def run_on_module(module: cllvm.Module):
    ctx = module.get_context()
    i32_t = ctx.int32_type()
    # constant i32(1)
    const_1 = i32_t.const_int(1, False)

    for fn in module.iter_functions():
        for bb in fn.iter_basic_blocks():
            for inst in bb.iter_instructions():
                if inst.instruction_opcode != cllvm.Ret:
                    continue
                inst.set_arg_operand(0, const_1)
    return 1
