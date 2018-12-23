use amethyst_core::{
    bundle::{Result, ResultExt, SystemBundle},
    shred::DispatcherBuilder,
};

use crate::metrics::{MetricObserver, NetworkMetricObject, NetworkMetrics};

use super::NetSocketSystem;

/// A convenience bundle to create the infrastructure needed to send and receive network messages.
pub struct MetricsBundle<T> {

}

impl<T> MetricsBundle<T> {
    pub fn new() -> Self {
        MetricsBundle {

        }
    }

    pub fn whit_metrics(metrics: NetworkMetricObject + Send + Sync>,) {

    }
}

impl<'a, 'b, T> SystemBundle<'a, 'b> for MetricsBundle<T>
    where
        T: Send + Sync + PartialEq + Serialize + Clone + DeserializeOwned + 'static,
{
    fn build(self, builder: &mut DispatcherBuilder<'_, '_>) -> Result<()> {

        Ok(())
    }
}
