use core::fmt;
use std::error::Error;

use crossterm::event::{self, Event};
use ratatui::{text::Text, widgets::Paragraph, Frame};
use regex::Regex;

mod builders;
mod model;

fn main() {}

struct W {
    s: Box<dyn T>,
}

/*
impl W {
    fn new(s: impl T) -> W{
        //This does not work, the compiler looses information about the (implicit) lifetime of s
        //because it is dynamic; s could contain something that might not life as long as W
        W {s: Box::new(s) }
    }
}

struct W2<V: T> {
    s: V
}

impl<V: T> W2<V> {
    //this works because event tough we store something behind a trait it will be expanded at compile time including lifetimes
    fn new(s: V) -> W2<V>{
        W2 { s }
    }
}
*/

struct S<'a> {
    x: &'a i32,
}

trait T {
    fn get(&self) -> i32;
}

impl<'a> T for S<'a> {
    fn get(&self) -> i32 {
        *self.x
    }
}

fn F(s: S) {
    print!("{}", s.x);
}

fn F2(s: impl T) {
    print!("{}", s.get());
}

fn F3(s: impl T + 'static) {
    print!("{}", s.get());
}
