use std::collections::{BTreeMap, HashMap};
use std::sync;

use common::collections::lang;
use common::err::TaskWorkerError;
use common::types::{ExecutorId, HashedResourceId};
use proto::common::common::ResourceId;
use proto::common::event::KeyedDataEvent;
use proto::common::stream::Dataflow;
use proto::worker::worker;
use stream::actor::DataflowContext;

use crate::manager::LocalExecutorManager;

type DataflowCacheRef = sync::RwLock<BTreeMap<HashedResourceId, LocalExecutorManager>>;

pub struct TaskWorker {
    cache: DataflowCacheRef,
}

struct TaskWorkerBuilder {}

impl TaskWorker {
    pub(crate) fn new() -> Self {
        TaskWorker {
            cache: Default::default(),
        }
    }

    pub fn stop_dataflow(&self, job_id: ResourceId) -> Result<(), TaskWorkerError> {
        let ref hashable_job_id = job_id.into();
        self.cache
            .try_write()
            .map(|mut managers| {
                managers.remove(hashable_job_id);
            })
            .map_err(|err| TaskWorkerError::ExecutionError(err.to_string()))
    }

    pub fn create_dataflow(
        &self,
        job_id: ResourceId,
        dataflow: Dataflow,
    ) -> Result<(), TaskWorkerError> {
        let ctx = DataflowContext::new(
            job_id.clone(),
            dataflow.meta.to_vec(),
            dataflow
                .nodes
                .iter()
                .map(|entry| (*entry.0 as ExecutorId, entry.1.clone()))
                .collect(),
        );

        match self
            .cache
            .try_write()
            .map(|mut managers| {
                LocalExecutorManager::new(ctx).map(|manager| {
                    managers.insert(job_id.into(), manager);
                })
            })
            .map_err(|err| TaskWorkerError::ExecutionError(err.to_string()))
        {
            Ok(r) => r.map_err(|err| err.into()),
            Err(err) => Err(err),
        }
    }

    pub fn dispatch_events(
        &self,
        events: Vec<KeyedDataEvent>,
    ) -> Result<HashMap<String, worker::DispatchDataEventStatusEnum>, TaskWorkerError> {
        let group = lang::group(&events, |e| {
            HashedResourceId::from(e.job_id.clone().unwrap())
        });

        group
            .iter()
            .map(|pair| {
                self.cache
                    .try_read()
                    .map(|managers| {
                        managers
                            .get(pair.0)
                            .map(|manager| {
                                (
                                    format!("{:?}", &manager.job_id),
                                    manager.dispatch_events(
                                        &group
                                            .get(&manager.job_id.clone().into())
                                            .map(|events| {
                                                events.iter().map(|e| (*e).clone()).collect()
                                            })
                                            .unwrap(),
                                    ),
                                )
                            })
                            .map(|pair| HashMap::from([pair]))
                            .unwrap_or_else(|| Default::default())
                    })
                    .map_err(|err| TaskWorkerError::ExecutionError(format!("{:?}", err)))
            })
            .next()
            .unwrap_or_else(|| Ok(Default::default()))
    }
}

impl TaskWorkerBuilder {
    pub(crate) fn build(&self) -> TaskWorker {
        TaskWorker::new()
    }

    pub(crate) fn new() -> Self {
        TaskWorkerBuilder {}
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct TaskWorkerConfig {
    pub port: usize,
}

pub fn new_worker() -> TaskWorker {
    TaskWorkerBuilder::new().build()
}
