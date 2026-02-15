/// CPU context for task switching
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TaskContext {
    // Callee-saved registers (preserved across function calls)
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub rbx: u64,
    pub rbp: u64,
    
    // Stack pointer
    pub rsp: u64,
    
    // Instruction pointer (return address)
    pub rip: u64,
}

impl TaskContext {
    /// Create a new empty context
    pub const fn new() -> Self {
        TaskContext {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
            rsp: 0,
            rip: 0,
        }
    }

    /// Initialize a context for a new task
    pub fn init(entry_point: u64, stack_top: u64) -> Self {
        TaskContext {
            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            rbx: 0,
            rbp: 0,
            rsp: stack_top,
            rip: entry_point,
        }
    }
}

/// Switch from the current context to a new context
/// 
/// # Safety
/// This function is unsafe because it directly manipulates CPU registers
/// and the stack pointer. The caller must ensure that:
/// - `old_context` points to valid memory where the current context can be saved
/// - `new_context` points to a valid, initialized context
/// - The new context's stack is valid and properly aligned
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(old_context: *mut TaskContext, new_context: *const TaskContext) {
    core::arch::naked_asm!(
        // Save current context to old_context
        "mov [rdi + 0x00], r15",
        "mov [rdi + 0x08], r14",
        "mov [rdi + 0x10], r13",
        "mov [rdi + 0x18], r12",
        "mov [rdi + 0x20], rbx",
        "mov [rdi + 0x28], rbp",
        "mov [rdi + 0x30], rsp",
        
        // Save return address (rip)
        "lea rax, [rip + 1f]",
        "mov [rdi + 0x38], rax",
        
        // Load new context from new_context
        "mov r15, [rsi + 0x00]",
        "mov r14, [rsi + 0x08]",
        "mov r13, [rsi + 0x10]",
        "mov r12, [rsi + 0x18]",
        "mov rbx, [rsi + 0x20]",
        "mov rbp, [rsi + 0x28]",
        "mov rsp, [rsi + 0x30]",
        
        // Jump to new task (load rip)
        "jmp [rsi + 0x38]",
        
        // Return point for when we switch back
        "1:",
        "ret",
    )
}

/// Entry point wrapper for new tasks
/// This is called when a task is first scheduled
#[unsafe(naked)]
pub unsafe extern "C" fn task_entry_wrapper() {
    core::arch::naked_asm!(
        // The task function pointer is in r15 (we'll set this up when creating tasks)
        // Call the task function
        "call r15",
        
        // If the task returns, we should mark it as completed
        // For now, just loop forever (we'll improve this later)
        "2:",
        "hlt",
        "jmp 2b",
    )
}
