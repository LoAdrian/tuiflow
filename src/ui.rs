pub(crate) mod body;
pub(crate) mod titlebar_widget;
pub(crate) mod controls_widget;
pub(crate) mod key_control_view_model;
pub(crate) mod main_widget;

pub(crate) trait DomainObserver {
    fn update(&mut self, property_name: &str);
}

pub(crate) trait NotifyPropertyChanged {
    fn register_observer(&mut self, observer: impl DomainObserver);
    fn notify_observers(&mut self, property_name: &str);
}