# PyLLVMPass

Build LLVM passes in python! This library aims to provide a generic shared
object  that when passed into llvm's optimizer will load arbitrary python
modules to implement the pass.

To use, request an optimizer named `pyllvmpass[<module name>]` after loading the
`pyllvmpass` shared object:

```bash
cargo build
opt --load-pass-plugin=./target/debug/libpypllvmpass.so --passes=pyllvmpass[my_mod] <inputs>...
```

in `my_mod.py`:

```python
from llvmcpy.llvm import Module
def run_on_module(module: Module) -> int:
    # return 0 to indicate that all preserved analysis can be kept, and 1
    # otherwise
    ...
```

Currently only ModulePasses are supported.
