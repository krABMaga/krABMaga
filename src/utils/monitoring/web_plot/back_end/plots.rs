cfg_if! {
  if #[cfg(feature ="plots")]{
      use crossbeam::thread;
      use std::sync::{Arc,Mutex};
      use clap::{App, Arg};
      use lazy_static::*;
  }
}