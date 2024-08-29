use llvm_plugin::inkwell::module::Module;
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};
use pyo3::prelude::*;
use std::process::Command;

// A name and version is required.
#[llvm_plugin::plugin(name = "pyllvmpass", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    // Add a callback to parse a name from the textual representation of the pipeline to be run.
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name.starts_with("pyllvmpass") {
            assert!(name.len() > ("pyllvmpass".len() + 2));
            assert!(name.as_bytes()["pyllvmpass".len()] == b'[');
            assert!(name.as_bytes()[name.len() - 1] == b']');
            let module = &name[("pyllvmpass".len() + 1)..(name.len() - 1)];
            // the input pipeline contains the name "custom-pass", so we add our custom pass to the
            // pass manager
            manager.add_pass(PyLLVMPass {
                module: module.to_string(),
            });
            // we notify the caller that we were able to parse
            // the given name
            PipelineParsing::Parsed
        } else {
            // in any other cases, we notify the caller that our
            // callback wasn't able to parse the given name
            PipelineParsing::NotParsed
        }
    });
}

struct PyLLVMPass {
    module: String,
}

impl LlvmModulePass for PyLLVMPass {
    fn run_pass(&self, module: &mut Module, _manager: &ModuleAnalysisManager) -> PreservedAnalyses {
        // We need to get the contents of sys.path because PyO3 seems to have a more restrictive set
        // of default entries in path (no current directory, no conda site pkgs, etc)
        let pypath_out = Command::new("python")
            .arg("-c")
            .arg("import sys; print(sys.path)")
            .output()
            .expect("failed to get python path");
        assert!(pypath_out.status.success());
        let pypath = String::from_utf8(pypath_out.stdout).expect("failed to parse python path");

        let raw_ptr = module.as_mut_ptr();
        let py_res = Python::with_gil(|py| {
            let path = py.eval_bound(&pypath, None, None)?;
            let sys = PyModule::import_bound(py, "sys")?;
            sys.setattr("path", path)?;

            let cllvm = PyModule::import_bound(py, "llvmcpy.llvm")?;
            let pyffi = cllvm.getattr("ffi")?;
            let llvm_module_ptr = pyffi
                .getattr("cast")?
                .call1(("struct LLVMOpaqueModule *", raw_ptr as usize))?;
            let cllvm_module_constructor = cllvm.getattr("Module")?;
            let llvm_module = cllvm_module_constructor.call1((llvm_module_ptr,))?;

            let mod_: &str = &self.module;
            let mod_ = PyModule::import_bound(py, mod_)?;
            let run_on_module = mod_.getattr("run_on_module")?;
            let res: i64 = run_on_module.call1((llvm_module,))?.extract()?;
            PyResult::Ok(res)
        });

        if let Err(ref x) = py_res {
            Python::with_gil(|py| {
                x.print_and_set_sys_last_vars(py);
            });
        }

        let py_res = py_res.expect("Failed to get value for PreservedAnalysis - If there are no errors below, check if the python module returns a value in all codepaths\n");
        if py_res == 0 {
            return PreservedAnalyses::All;
        }
        PreservedAnalyses::None
    }
}
