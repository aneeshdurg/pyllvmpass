"""
This example starts an interactive shell to allow interacting with llvmcpy
objects during an analysis pass
"""

from llvmcpy import LLVMCPy
import code

cllvm = LLVMCPy()


def run_on_module(module: cllvm.Module):
    fns = [fn for fn in module.iter_functions()]
    fn = fns[0]
    bbs = [bb for bb in fn.iter_basic_blocks()]
    code.InteractiveConsole(locals={**locals(), **globals()}).interact()

    return 0
