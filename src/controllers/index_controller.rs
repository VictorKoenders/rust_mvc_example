use rust_mvc::view::{ ViewContext, ViewResult };
use models;

#[allow(unused_attributes)]
#[http_url("/")]
pub fn index(_: ViewContext) -> ViewResult {
	view!()
}

#[allow(unused_attributes)]
#[http_url("/index2")]
pub fn index2(_: ViewContext) -> ViewResult {
	view!(models::Model { name: "Test 123".to_string() })
}