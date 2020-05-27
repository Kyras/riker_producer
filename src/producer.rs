use riker::actors::*;
use crossbeam::{
    Sender as ChannelSender, unbounded, TryRecvError,
};
use crate::{
    utility::*,
    traits::*,
};
use std::thread::JoinHandle;


pub struct Producer<T: ProducerBehaviour + Sync + Send + 'static, U: ProducerProcessor<T> + Sync + Send + 'static> {
    producer: Option<T>,
    processor: U,
    stopped: bool,
    handle: Option<JoinHandle<T>>,
    ctrl_channel: Option<ChannelSender<ProducerControl>>,
}

impl<T: ProducerBehaviour + Sync + Send + 'static, U: ProducerProcessor<T> + Sync + Send + 'static> Producer<T, U> {
    fn stop_producer_thread(&mut self) {
        self.stopped = true;
        if let Some(ref mut tx) = self.ctrl_channel {
            let _ = tx.send(ProducerControl::Stop);
        }
    }
}

impl<ProdArgs, ProcArgs, ProdFact, ProcFact> ActorFactoryArgs<(ProdArgs, ProcArgs)> for Producer<ProdFact, ProcFact>
    where
        ProdArgs: ActorArgs,
        ProcArgs: ActorArgs,
        ProdFact: ProducerBehaviourFactoryArgs<ProdArgs> + Sync + Send + 'static,
        ProcFact: ProducerProcessorFactoryArgs<ProdFact, ProcArgs> + Sync + Send + 'static,
{
    fn create_args(args: (ProdArgs, ProcArgs)) -> Self {
        let (bargs, parcs) = args;
        let producer = ProdFact::create_args(bargs);
        let processor = ProcFact::create_args(parcs);
        Self {
            processor,
            producer: Some(producer),
            handle: None,
            stopped: false,
            ctrl_channel: None,
        }
    }
}

impl<T: ProducerBehaviour + Sync + Send + 'static, U: ProducerProcessor<T> + Sync + Send + 'static> Actor for Producer<T, U> {
    type Msg = ProducerOutput<T::Product, T::Completed>;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        let producer = std::mem::take(&mut self.producer);
        if let Some(producer) = producer {
            let self_ref = ctx.myself();
            let (tx, rx) = unbounded();
            let handle = std::thread::spawn(move || {
                let ctrl_chan = rx;
                let mut producer = producer;
                let actor_ref = self_ref;
                if producer.pre_start() {
                    loop {
                        match ctrl_chan.try_recv() {
                            Ok(_) | Err(TryRecvError::Disconnected) => {
                                return producer;
                            }
                            Err(TryRecvError::Empty) => {
                                let product = producer.produce();
                                let is_completed = product.is_completed();
                                actor_ref.send_msg(product, actor_ref.clone());
                                if is_completed {
                                    producer.post_stop();
                                    return producer;
                                }
                            }
                        }
                    }
                } else {
                    return producer;
                }
            });
            self.ctrl_channel = Some(tx);
            self.handle = Some(handle);
        }
    }

    fn post_stop(&mut self) {
        self.stop_producer_thread()
    }

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        if !self.stopped {
            match msg {
                ProducerOutput::Produced(value) => {
                    let output = self.processor.post_process(ctx, value, sender);
                    if output.is_some() {
                        self.stop_producer_thread()
                    }
                }
                ProducerOutput::Completed(value) => {
                    self.processor.post_stop(ctx, value, sender);
                    ctx.stop(ctx.myself());
                }
            }
        }
    }
}