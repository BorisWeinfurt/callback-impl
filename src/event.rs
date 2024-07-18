use crate::callback::Callback;

pub struct Event<Args, Output> {
    pub start_time: usize,
    pub callback: Callback<Args, Output>,
    pub arg: Args,
}

impl<Args, Output> Event<Args, Output> {
    pub fn new(start_time: usize, callback: Callback<Args, Output>, arg: Args) -> Self {
        Self {
            start_time,
            callback,
            arg,
        }
    }

    pub async fn execute(self) -> Output {
        self.callback.call(self.arg).await
    }
}

impl<Args, Output> PartialEq for Event<Args, Output> {
    fn eq(&self, other: &Self) -> bool {
        self.start_time == other.start_time
    }
}

impl<Args, Output> Eq for Event<Args, Output> {}

impl<Args, Output> PartialOrd for Event<Args, Output> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Args, Output> Ord for Event<Args, Output> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start_time.cmp(&other.start_time)
    }
}
