use super::{TaskId, kernel_thread::KernelThread, context::{TaskContext, switch_context}};
use alloc::collections::BTreeMap;
use spin::Mutex;

/// Thread-aware scheduler with context switching support
pub struct ThreadScheduler {
    threads: BTreeMap<TaskId, KernelThread>,
    current_thread: Option<TaskId>,
    idle_context: TaskContext,
}

impl ThreadScheduler {
    pub const fn new() -> Self {
        ThreadScheduler {
            threads: BTreeMap::new(),
            current_thread: None,
            idle_context: TaskContext::new(),
        }
    }

    /// Add a kernel thread to the scheduler
    pub fn add_thread(&mut self, thread: KernelThread) {
        let thread_id = thread.id();
        self.threads.insert(thread_id, thread);
    }

    /// Switch to the next thread (round-robin)
    pub fn schedule_next(&mut self) {
        // Get the next thread to run
        let next_thread_id = self.get_next_thread();
        
        if let Some(next_id) = next_thread_id {
            if Some(next_id) != self.current_thread {
                self.switch_to(next_id);
            }
        }
    }

    /// Get the next thread ID to run (round-robin)
    fn get_next_thread(&self) -> Option<TaskId> {
        if self.threads.is_empty() {
            return None;
        }

        // If there's a current thread, get the next one after it
        if let Some(current_id) = self.current_thread {
            let mut found_current = false;
            for (&thread_id, _) in &self.threads {
                if found_current {
                    return Some(thread_id);
                }
                if thread_id == current_id {
                    found_current = true;
                }
            }
        }

        // Wrap around to the first thread
        self.threads.keys().next().copied()
    }

    /// Switch to a specific thread
    fn switch_to(&mut self, new_thread_id: TaskId) {
        let old_context = if let Some(current_id) = self.current_thread {
            // Get mutable reference to current thread's context
            if let Some(current_thread) = self.threads.get_mut(&current_id) {
                &mut current_thread.context as *mut TaskContext
            } else {
                &mut self.idle_context as *mut TaskContext
            }
        } else {
            &mut self.idle_context as *mut TaskContext
        };

        // Get the new thread's context
        if let Some(new_thread) = self.threads.get(&new_thread_id) {
            let new_context = &new_thread.context as *const TaskContext;
            
            // Update current thread
            self.current_thread = Some(new_thread_id);
            
            // Perform the context switch
            unsafe {
                switch_context(old_context, new_context);
            }
        }
    }

    /// Get the currently running thread ID
    pub fn current_thread(&self) -> Option<TaskId> {
        self.current_thread
    }
}

static THREAD_SCHEDULER: Mutex<ThreadScheduler> = Mutex::new(ThreadScheduler::new());

/// Add a kernel thread to the global scheduler
pub fn add_kernel_thread(thread: KernelThread) {
    THREAD_SCHEDULER.lock().add_thread(thread);
}

/// Schedule the next thread
pub fn schedule_next_thread() {
    THREAD_SCHEDULER.lock().schedule_next();
}

/// Get the currently running thread
pub fn current_thread() -> Option<TaskId> {
    THREAD_SCHEDULER.lock().current_thread()
}
