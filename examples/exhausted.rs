use riker::actors::*;
use riker_producer::prelude::*;
use std::time::Duration;


pub struct Counter {
    num: i32,
}

impl ProducerBehaviour for Counter {
    type Product = i32;
    type Completed = ();

    fn produce(&mut self) -> ProducerOutput<Self::Product, Self::Completed> {
        let num = self.num;
        self.num += 1;
        if num < 100 {
            ProducerOutput::Produced(num)
        } else {
            ProducerOutput::Completed(())
        }
    }
}

impl ProducerBehaviourFactoryArgs<()> for Counter {
    fn create_args(_: ()) -> Self {
        Self { num: 0 }
    }
}

pub struct Count;

impl ProducerProcessor<Counter> for Count {
    fn post_process(&mut self, _: &Context<ProducerOutput<<Counter as ProducerBehaviour>::Product, <Counter as ProducerBehaviour>::Completed>>, value: <Counter as ProducerBehaviour>::Product, _: Sender) -> Option<ProducerControl> {
        println!("Processing number: {}", value);
        None
    }

    fn post_stop(&mut self, _: &Context<ProducerOutput<<Counter as ProducerBehaviour>::Product, <Counter as ProducerBehaviour>::Completed>>, _: <Counter as ProducerBehaviour>::Completed, _: Sender) {
        println!("Producer exhausted itself");
    }
}

impl ProducerProcessorFactoryArgs<Counter, ()> for Count {
    fn create_args(_: ()) -> Self {
        Self
    }
}

fn main() {
    let sys = ActorSystem::new()
        .expect("Failed to create actor system");
    let _ = sys.actor_of_args::<Producer<Counter, Count>, _>("counter", ((), ()));
    std::thread::sleep(Duration::from_secs(1));
}
