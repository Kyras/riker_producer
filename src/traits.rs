use crate::utility::*;
use riker::actor::{ActorArgs, Context, Sender};

pub trait ProducerBehaviour {
    type Product: Producible;
    type Completed: Producible;

    fn pre_start(&mut self) -> bool { true }
    fn produce(&mut self) -> ProducerOutput<Self::Product, Self::Completed>;
    fn post_stop(&mut self) {}
}

pub trait ProducerBehaviourFactoryArgs<Args: ActorArgs>: ProducerBehaviour {
    fn create_args(args: Args) -> Self;
}

#[allow(unused_variables)]
pub trait ProducerProcessor<Producer: ProducerBehaviour> {
    fn pre_start(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>) {}
    fn post_start(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>) {}

    fn post_process(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>, value: Producer::Product, sender: Sender) -> Option<ProducerControl>;
    fn post_stop(&mut self, ctx: &Context<ProducerOutput<Producer::Product, Producer::Completed>>, value: Producer::Completed, sender: Sender) {}
}

pub trait ProducerProcessorFactoryArgs<T: ProducerBehaviour, Args: ActorArgs>: ProducerProcessor<T> {
    fn create_args(args: Args) -> Self;
}
