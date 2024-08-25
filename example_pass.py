import llvmcpy.llvm as cllvm

def run_on_module(m: cllvm.Module):
    print("hello from python!")
    print(m.print_module_to_string().decode())
    return 0
