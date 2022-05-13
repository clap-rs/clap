#[derive(Clone, Debug)]
pub struct AnyValue {
    inner: std::sync::Arc<dyn std::any::Any + Send + Sync + 'static>,
}

impl AnyValue {
    pub(crate) fn new(inner: impl std::any::Any + Send + Sync + 'static) -> Self {
        let inner = std::sync::Arc::new(inner);
        Self { inner }
    }

    pub(crate) fn downcast_ref<T: std::any::Any>(&self) -> Option<&T> {
        self.inner.downcast_ref::<T>()
    }
}
