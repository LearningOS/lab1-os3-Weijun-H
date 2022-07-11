//! Types related to task management

use super::TaskContext;
use crate::config::{MAX_SYSCALL_NUM};

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    pub start_time: usize,
    pub running_time: usize,
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone, PartialEq)]
pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

impl TaskInfo {
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }

    pub fn set_syscall_times(&mut self, time: [u32; MAX_SYSCALL_NUM]) {
        self.syscall_times = time;
    }

    pub fn set_time (&mut self, val: usize) {
        self.time = val;
    }

}

