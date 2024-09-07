# PyLLVMPass

Build LLVM passes in python! This library aims to provide a generic shared
object  that when passed into LLVM's optimizer will load arbitrary python
modules to implement the pass.

## Installation

```bash
# Currently depends on llvm-18 being installed
pip install llvmcpy
git clone https://github.com/aneeshdurg/pyllvmpass
cd pyllvmpass
cargo build
# Make sure that either LLVM_CONFIG is set in the environment or llvm-config is
# on the PATH.
```

## Usage

To use, request an optimizer named `pyllvmpass[<module name>]` after loading the
`pyllvmpass` shared object. For example, if you had a module named
`example_pass`, either in the current directory as `example_pass.py` or
as an installed library, you can do:

```bash
# Note that the module being loaded is supplied to pyllvmpass in the square
# brackets
opt --load-pass-plugin=./target/debug/libpypllvmpass.so --passes=pyllvmpass[example_pass] <inputs>...
# If you have multiple modules that you'd like to run as a pass you can include
# multiple instances of pyllvmpass[] above.
```

The module *MUST* define a function `run_on_module` that accepts the module
being transformed as a `llvmcpy.llvm.Module`. See
[llvmcpy](https://github.com/revng/llvmcpy) for API details. It is more or less
a thin wrapper around the LLVM C API.

For our example in `example_pass.py`, we could do the following:
```python
from llvmcpy.llvm import Module
def run_on_module(module: Module) -> int:
    print("hello from python!")
    src = module.print_module_to_string().decode()
    # Print the llvm IR
    print(src)

    # Any python code can be written here. Even interactive debugging with
    # breakpoint is possible!
    # breakpoint()

    # return 0 to indicate that all preserved analysis can be kept, and 1
    # otherwise
    return 0
    ...
```

Currently only ModulePasses are supported. See the [test](test/) directory for examples that
transform the IR.

## Why does this exist?

I think having an interactive console is a great tool for learning new things.
C++ and Rust struggle with being interactive due to their compiled nature. While
I love coding in C++ and Rust, I think python is a very powerful language for
prototyping, and having the choice to work in python lowers the barrier of entry
to newcomers. However, the LLVM C API has a lot of limitations, so we might not
ever see a lot of more involved LLVM passes ever written in anything except C++.
