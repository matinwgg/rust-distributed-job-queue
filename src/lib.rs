use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Job {
    pub id: Uuid,
    pub kind: String,
    pub payload: serde_json::Value,
    pub attempts: u32,
    pub max_attempts: u32,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum QueueError {
    #[error("job {0} is already completed")]
    AlreadyCompleted(Uuid),
    #[error("job {0} is not leased")]
    NotLeased(Uuid),
    #[error("job {0} lease belongs to another worker")]
    WrongWorker(Uuid),
}

#[derive(Debug, Default)]
pub struct Queue {
    ready: VecDeque<Job>,
    leased: HashMap<Uuid, (Job, String)>,
    completed: HashMap<Uuid, Job>,
    dead_letters: Vec<Job>,
}

impl Queue {
    pub fn enqueue(&mut self, kind: impl Into<String>, payload: serde_json::Value, max_attempts: u32) -> Uuid {
        let id = Uuid::new_v4();
        self.ready.push_back(Job { id, kind: kind.into(), payload, attempts: 0, max_attempts: max_attempts.max(1) });
        id
    }

    pub fn lease(&mut self, worker: impl Into<String>) -> Option<Job> {
        let worker = worker.into();
        let mut job = self.ready.pop_front()?;
        job.attempts += 1;
        self.leased.insert(job.id, (job.clone(), worker));
        Some(job)
    }

    pub fn ack(&mut self, job_id: Uuid, worker: &str) -> Result<(), QueueError> {
        let (job, owner) = self.leased.remove(&job_id).ok_or(QueueError::NotLeased(job_id))?;
        if owner != worker {
            self.leased.insert(job_id, (job, owner));
            return Err(QueueError::WrongWorker(job_id));
        }
        if self.completed.contains_key(&job_id) {
            return Err(QueueError::AlreadyCompleted(job_id));
        }
        self.completed.insert(job_id, job);
        Ok(())
    }

    pub fn nack(&mut self, job_id: Uuid, worker: &str) -> Result<(), QueueError> {
        let (job, owner) = self.leased.remove(&job_id).ok_or(QueueError::NotLeased(job_id))?;
        if owner != worker {
            self.leased.insert(job_id, (job, owner));
            return Err(QueueError::WrongWorker(job_id));
        }
        if job.attempts >= job.max_attempts {
            self.dead_letters.push(job);
        } else {
            self.ready.push_back(job);
        }
        Ok(())
    }

    pub fn ready_len(&self) -> usize { self.ready.len() }
    pub fn leased_len(&self) -> usize { self.leased.len() }
    pub fn completed_len(&self) -> usize { self.completed.len() }
    pub fn dead_letter_len(&self) -> usize { self.dead_letters.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn failed_jobs_are_retried_until_dead_lettered() {
        let mut q = Queue::default();
        q.enqueue("email", json!({"to":"a@example.com"}), 2);
        let first = q.lease("worker-a").unwrap();
        q.nack(first.id, "worker-a").unwrap();
        let second = q.lease("worker-a").unwrap();
        q.nack(second.id, "worker-a").unwrap();
        assert_eq!(q.ready_len(), 0);
        assert_eq!(q.dead_letter_len(), 1);
    }

    #[test]
    fn only_lease_owner_can_ack() {
        let mut q = Queue::default();
        let id = q.enqueue("task", json!({}), 1);
        q.lease("worker-a").unwrap();
        assert_eq!(q.ack(id, "worker-b"), Err(QueueError::WrongWorker(id)));
        q.ack(id, "worker-a").unwrap();
        assert_eq!(q.completed_len(), 1);
    }
}
