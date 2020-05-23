use crate::utility::*;
use riker::actor::{ActorArgs, Context, Sender};

pub trait ProducerBehaviour {
    type Product: Producible;
    type Completed: Producible;

    fn produce(&mut self) -> ProducerOutput<Self::Product, Self::Completed>;
}

pub trait ProducerBehaviourFactoryArgs<Args: ActorArgs>: ProducerBehaviour {
    fn create_args(args: Args) -> Self;
}

pub trait ProducerProcessor<Producer: ProducerBehaviour> {
    fn post_process(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>, value: Producer::Product, sender: Sender) -> Option<ProducerControl>;
    fn post_stop(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>, value: Producer::Completed, sender: Sender) {
        let _ = (ctx, value, sender);
    }
}

pub trait ProducerProcessorFactoryArgs<T: ProducerBehaviour, Args: ActorArgs>: ProducerProcessor<T> {
    fn create_args(args: Args) -> Self;
}
