use leptos::prelude::*;

pub type ToastId = u64;

#[derive(Debug, Clone)]
pub enum ToastLevel {
    Info,
    Error,
    Warning,
    Success,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToastPosition {
    TopLeft,
    Top,
    TopRight,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone)]
pub struct ToastData {
    pub id: ToastId,

    pub level: ToastLevel,
    pub expiry: Option<u32>,
    pub position: ToastPosition,

    pub clear_signal: RwSignal<bool>,
}
