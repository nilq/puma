use std::fmt;
use colored::Colorize;

pub enum Response<T: fmt::Display> {
  Wrong(T),
  Weird(T),
  Note(T),
}

use self::Response::*;

#[macro_export]
macro_rules! response {
  ( $( $r:expr ),+ ) => {{
    $(
      print!("{}", $r);
    )*
    println!();
  }};
}

impl<T: fmt::Display> fmt::Display for Response<T> {
  fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
    let (color, message_type, message) = match *self {
      Wrong(ref m) => ("red",    "error", m),
      Weird(ref m) => ("yellow", "warning", m),
      Note(ref m)  => ("blue",   "help",  m),
    };

    let message_type = format!("\n{}", message_type).color(color).bold();
    let message      = format!("{}", message);

    let message      = format!("{}: {}", message_type, message);

    write!(f, "{}", message)
  }
}