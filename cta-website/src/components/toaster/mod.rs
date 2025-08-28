use std::{
    fmt::Display,
    sync::{Arc, Mutex},
};

use leptos::prelude::{GetUntracked, RwSignal, Set};

use crate::components::toaster::toast::{
    builder::ToastBuilder,
    data::{ToastData, ToastId},
};

mod toast;
#[allow(clippy::module_inception)]
mod toaster;

#[derive(Clone, Debug)]
pub struct ToasterContext {
    stats: Arc<Mutex<ToasterStats>>,
    pub queue: RwSignal<Vec<ToastData>>,
}

#[derive(Clone, Debug, Default)]
struct ToasterStats {
    visible: u32,
    total: u64,
}

impl ToasterContext {
    pub fn toast(&self, builder: ToastBuilder) {
        let mut stats = self.stats.lock().unwrap();
        let toast = builder.build(stats.total + 1);

        let mut queue = self.queue.get_untracked();
        queue.push(toast);
        stats.visible += 1;
        stats.total += 1;
    }

    pub fn info<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(toast::data::ToastLevel::Info));
    }

    pub fn success<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(toast::data::ToastLevel::Success));
    }

    pub fn warning<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(toast::data::ToastLevel::Warning));
    }

    pub fn error<T>(&self, message: T)
    where
        T: Display,
    {
        self.toast(ToastBuilder::new(message).with_level(toast::data::ToastLevel::Error));
    }

    pub fn clear(&self) {
        for toast in &self.queue.get_untracked() {
            toast.clear_signal.set(true);
        }
    }

    pub fn remove(&self, toast_id: ToastId) {
        let index = self
            .queue
            .get_untracked()
            .iter()
            .enumerate()
            .find(|(_, toast)| toast.id == toast_id)
            .map(|(index, _)| index);

        if let Some(index) = index {
            let mut queue = self.queue.get_untracked();
            queue.remove(index);
            self.queue.set(queue);

            self.stats.lock().unwrap().visible -= 1;
        }
    }
}

impl Default for ToasterContext {
    fn default() -> Self {
        ToasterContext {
            stats: Arc::new(Mutex::new(ToasterStats::default())),
            queue: RwSignal::new(Vec::new()),
        }
    }
}
