#![feature(attr_literals)]
#![feature(custom_attribute)]

#[macro_use]
extern crate rust_mvc;

mod models;
mod run;
mod controllers;
mod views;

fn main(){
	run::run();
}