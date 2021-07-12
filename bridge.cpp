#include "globals.hpp"

namespace Baasha {

    void initializeModuleAndPassManager() {

        DEBUG_LOG("initialize module and pass manager visited");

        llvm::InitializeNativeTarget();
        llvm::InitializeNativeTargetAsmPrinter();
        llvm::InitializeNativeTargetAsmParser();

        the_context = std::make_unique<llvm::LLVMContext>();

        the_JIT = exit_on_error(llvm::orc::BaashaJIT::Create());
        
        the_module = std::make_unique<llvm::Module>("manik baasha", *the_context);
        the_module->setDataLayout(the_JIT->getDataLayout());

        ir_builder = std::make_unique<llvm::IRBuilder<>>(*the_context);
    }

}