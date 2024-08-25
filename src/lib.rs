use std::collections::HashMap;

use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values::*;
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};

use pyo3::prelude::*;
use pyo3::types::{PyList, PyString};

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
            //manager.add_pass(GlobalVariablePointerRenamePass);

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
        let raw_ptr = module.as_mut_ptr();
        let py_res = Python::with_gil(|py| {
            let sys = PyModule::import_bound(py, "sys")?;
            let py_path: Bound<'_, PyList> = sys.getattr("path")?.downcast_into()?;
            // TODO can PyO3 share the same default path as the system?
            py_path.append("")?;
            py_path.append("/home/aneesh/miniconda3/lib/python3.12/site-packages")?;

            let mod_: &str = &self.module;
            let mod_ = PyModule::import_bound(py, mod_)?;
            let run_on_module = mod_.getattr("run_on_module")?;
            // TODO import_bound("llvmcpy") and then convert raw_ptr into a llvmcpy.llvm.Module
            let res: i64 = run_on_module.call1(raw_ptr as usize)?.extract()?;
            PyResult::Ok(res)
        })
        .unwrap();

        if py_res == 0 {
            return PreservedAnalyses::All;
        }
        PreservedAnalyses::None
    }
}
