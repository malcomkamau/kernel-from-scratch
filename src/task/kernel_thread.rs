use super::{TaskId, context::TaskContext};
use alloc::vec::Vec;

const KERNEL_STACK_SIZE: usize = 4096 * 4; // 16 KiB stack per task

/// A kernel thread that uses context switching (not async/await)
pub struct KernelThread {
    pub id: TaskId,
    pub context: TaskContext,
    stack: Vec<u8>,
}

impl KernelThread {
    /// Create a new kernel thread
    pub fn new(entry_point: extern "C" fn()) -> Self {
        let mut stack = Vec::with_capacity(KERNEL_STACK_SIZE);
        stack.resize(KERNEL_STACK_SIZE, 0);
        
        // Stack grows downward, so the top is at the end
        let stack_top = stack.as_ptr() as u64 + KERNEL_STACK_SIZE as u64;
        
        // Initialize context with entry point
        let mut context = TaskContext::init(
            entry_point as u64,
            stack_top
        );
        
        // Set r15 to the entry point for task_entry_wrapper
        context.r15 = entry_point as u64;
        
        // Set rip to task_entry_wrapper
        context.rip = super::context::task_entry_wrapper as *const () as u64;
        
        KernelThread {
            id: TaskId::new(),
            context,
            stack, // Stack is kept alive for the lifetime of the thread
        }
    }
    
    /// Get the task ID
    pub fn id(&self) -> TaskId {
        self.id
    }
}

// Implement new() for TaskId to make it accessible from kernel_thread
impl TaskId {
    pub(crate) fn new() -> Self {
        use core::sync::atomic::{AtomicU64, Ordering};
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}
