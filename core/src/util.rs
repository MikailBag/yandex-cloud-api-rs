pub(crate) struct RaiiTask<T>(tokio::task::JoinHandle<T>);

impl<T: Send + 'static> RaiiTask<T> {
    pub(crate) fn spawn<F: std::future::Future<Output = T> + Send + 'static>(future: F) -> Self {
        let t = tokio::task::spawn(future);
        RaiiTask(t)
    }
}

impl<T> Drop for RaiiTask<T> {
    fn drop(&mut self) {
        self.0.abort();
    }
}
