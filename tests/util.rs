#[cfg(not(target_arch = "wasm32"))]
pub use easy_parallel::Parallel;

// A WASM shim for [`easy_parallel::Parallel`] that is good enough for us to run tests with
#[cfg(target_arch = "wasm32")]
pub struct Parallel<T> {
    closures: Vec<Box<dyn FnOnce() -> T + Send>>,
}

impl<T: 'static> Parallel<T> {
    pub fn new() -> Self {
        Self {
            closures: Default::default(),
        }
    }

    pub fn add<F: FnOnce() -> T + Send + 'static>(mut self, closure: F) -> Self {
        self.closures.push(Box::new(closure));

        self
    }

    pub fn run(self) {
        for closure in self.closures {
            wasm_bindgen_futures::spawn_local(async move {
                (closure)();
            })
        }
    }
}
