//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
use crate::timer::get_time_ms;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task syscall counter
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The task start time in ms 
    pub task_start_time: Option<usize>,
}

impl TaskControlBlock {
    /// Init start time
    pub fn set_start_time(&mut self) {
        if let None = self.task_start_time {
            self.task_start_time = Some(get_time_ms());
        }
    }

}
/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
