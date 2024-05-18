mod imp;

use glib::Object;
use gtk::glib;

glib::wrapper! {
	pub struct CustomButton(ObjectSubclass<imp::CustomButton>)
		@extends gtk::button, gtk::Widget,
		@implements gtk::Accessible, gtk::Actionable, gtk::Buildable,
}

impl CustomButton {
	pub fn new() -> Self {
		Object::builder().build()
	}

	pub fn with_label(label: &str) -> Self {
		Object::builder().property("label", label).build()
	}
}

impl Default for CustomButton {
	fn default() -> Self {
		Self::new()
	}
}

