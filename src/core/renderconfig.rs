pub struct RenderConfig {
    pub max_threads: usize,
    pub max_thread_area: i64,
    //pub fps: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            max_threads: std::thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4),
            max_thread_area: 160_000,
            //fps: 60,
        }
    }
}