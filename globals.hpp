#ifndef __BAASHA_GLOBAL_H
#define __BAASHA_GLOBAL_H

#include <llvm/ADT/APFloat.h>
#include <llvm/ADT/APInt.h>
#include <llvm/ADT/STLExtras.h>

#include <llvm/Analysis/BasicAliasAnalysis.h>
#include <llvm/Analysis/Passes.h>

#include <llvm/IR/DIBuilder.h>
#include <llvm/IR/IRBuilder.h>
#include <llvm/IR/LLVMContext.h>
#include <llvm/IR/LegacyPassManager.h>
#include <llvm/IR/Module.h>
#include <llvm/IR/Verifier.h>

#include <llvm/Support/Host.h>
#include <llvm/Support/TargetSelect.h>

#include <llvm/Transforms/Scalar.h>

#include <iostream>
#include <fstream>
#include <sstream>
#include <map>
#include <vector>
#include <cstdio>
#include <cstdint>
#include <string>
#include <memory>

#include "BaashaJIT.hpp"
#include "Logger.hpp"

namespace Baasha {
    
std::unique_ptr<llvm::LLVMContext> the_context;
std::unique_ptr<llvm::Module> the_module;
std::unique_ptr<llvm::IRBuilder<>> ir_builder;
llvm::ExitOnError exit_on_error;

std::map<std::string, llvm::AllocaInst*> named_values;
std::unique_ptr<llvm::orc::BaashaJIT> the_JIT;
std::unique_ptr<llvm::DIBuilder> dbg_builder;
std::shared_ptr<Logger> logger = Logger::getInstance();

std::string source_code;

uint32_t TAB_SPACE_COLS = 4;

// #define DEBUG

#ifdef DEBUG
#define DEBUG_LOG(x) printf("%s\n", x)
#else
#define DEBUG_LOG(x) printf("")
#endif
}

#endif // __BAASHA_GLOBAL_H

