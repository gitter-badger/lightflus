use std::thread::JoinHandle;

use common::err::ExecutionException;
use common::event::LocalEvent;
use common::types::SinkId;
use proto::common::common::ResourceId;
use proto::common::event::KeyedDataEvent;
use proto::worker::worker::DispatchDataEventStatusEnum;
use stream::actor::{DataflowContext, Sink, SinkImpl, SinkableMessageImpl};

pub struct LocalExecutorManager {
    pub job_id: ResourceId,
    handlers: Vec<JoinHandle<()>>,
    inner_sinks: Vec<SinkImpl>,
}

impl Drop for LocalExecutorManager {
    fn drop(&mut self) {
        log::info!("LocalExecutorManager for Job {:?} is dropping", self.job_id);
        for sink in &self.inner_sinks {
            let event = LocalEvent::Terminate {
                job_id: self.job_id.clone(),
                to: sink.sink_id(),
            };
            match sink.sink(SinkableMessageImpl::LocalMessage(event.clone())) {
                Err(err) => {
                    log::error!(
                        "termintate node {} failed. details: {:?}",
                        sink.sink_id(),
                        err
                    );
                }
                _ => log::info!("terminate node {} success", sink.sink_id()),
            }
        }
        self.handlers.clear();
        self.inner_sinks.clear();
    }
}

impl LocalExecutorManager {
    pub fn dispatch_events(&self, events: &Vec<KeyedDataEvent>) -> DispatchDataEventStatusEnum {
        // only one sink will be dispatched
        let local_events = events
            .iter()
            .map(|e| LocalEvent::KeyedDataStreamEvent(e.clone()));

        events
            .iter()
            .next()
            .map(|e| e.to_operator_id as SinkId)
            .map(|sink_id| {
                self.inner_sinks
                    .iter()
                    .filter(|sink| sink.sink_id() == sink_id)
                    .next()
                    .map(|sink| {
                        local_events
                            .map(|e| sink.sink(SinkableMessageImpl::LocalMessage(e)))
                            .map(|result| match result {
                                Ok(status) => status,
                                Err(err) => {
                                    // TODO fault tolerant
                                    log::error!("dispatch event failed: {:?}", err);
                                    DispatchDataEventStatusEnum::FAILURE
                                }
                            })
                            .reduce(|accum, result| match accum {
                                DispatchDataEventStatusEnum::FAILURE => accum,
                                _ => result,
                            })
                            .unwrap_or(DispatchDataEventStatusEnum::DONE)
                    })
                    .unwrap_or(DispatchDataEventStatusEnum::DONE)
            })
            .unwrap_or(DispatchDataEventStatusEnum::DONE)
    }

    pub fn new(ctx: DataflowContext) -> Result<Self, ExecutionException> {
        if !ctx.validate() {
            return Err(ExecutionException::invalid_dataflow(&ctx.job_id));
        }

        let executors = ctx.create_executors();

        Ok(Self {
            job_id: ctx.job_id.clone(),
            inner_sinks: executors.iter().map(|exec| exec.as_sinkable()).collect(),
            handlers: executors.iter().map(|exec| exec.clone().run()).collect(),
        })
    }
}
