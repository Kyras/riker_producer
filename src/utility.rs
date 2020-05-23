pub enum ProducerControl {
    Stop
}

pub trait Producible: std::fmt::Debug + Clone + Sync + Send + 'static {}

impl<T: std::fmt::Debug + Clone + Sync + Send + 'static> Producible for T {}

#[derive(Debug, Clone)]
pub enum ProducerOutput<Product: Producible, Value: Producible> {
    Produced(Product),
    Completed(Value),
}

impl<Product: Producible, Value: Producible> ProducerOutput<Product, Value> {
    pub fn produced(value: Product) -> Self {
        Self::Produced(value)
    }

    pub fn completed(value: Value) -> Self {
        Self::Completed(value)
    }

    pub fn is_completed(&self) -> bool {
        if let ProducerOutput::Completed(_) = self {
            true
        } else {
            false
        }
    }
}