unsafe fn codegen(input: String) {
    // Create an llvm context
    let context = llvm::core::LLVMContextCreate();
    
    // Create an llvm module
    let module = llvm::core::LLVMModuleCreateWithName(
        name.as_ptr() as *const _);
}
