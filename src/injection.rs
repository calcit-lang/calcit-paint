/// reused code from
/// https://github.com/calcit-lang/calcit_runner.rs/blob/main/src/bin/injection/mod.rs
use cirru_edn::Edn;
use std::sync::RwLock;

lazy_static! {

  // TODO need better soution for immediate calls
    /// to be read by external logics and used as FFI
    static ref PROGRAM_FFI_MESSAGES: RwLock<Vec<(String, CalcitItems)>> = RwLock::new(vec![]);
}

use calcit_runner::{
  builtins,
  data::edn::{calcit_to_edn, edn_to_calcit},
  primes::{Calcit, CalcitErr, CalcitItems, CrListWrap},
};

/// FFI protocol types
type EdnFfi = fn(args: Vec<Edn>) -> Result<Edn, String>;

pub fn inject_platform_apis() {
  builtins::register_import_proc("&call-dylib-edn", call_dylib_edn);
  builtins::register_import_proc("echo", echo);
  builtins::register_import_proc("println", echo);
  builtins::register_import_proc("&ffi-message", ffi_message);
}

// &call-dylib-edn
pub fn call_dylib_edn(xs: &CalcitItems) -> Result<Calcit, CalcitErr> {
  if xs.is_empty() {
    return Err(CalcitErr::use_string(format!(
      "&call-dylib-edn expected >2 arguments, got {}",
      CrListWrap(xs.to_owned())
    )));
  }
  let lib_name = if let Calcit::Str(s) = &xs[0] {
    s.to_owned()
  } else {
    return Err(CalcitErr::use_string(format!(
      "&call-dylib-edn expected a lib_name, got {}",
      xs[0]
    )));
  };

  let method: String = if let Calcit::Str(s) = &xs[1] {
    s.to_owned()
  } else {
    return Err(CalcitErr::use_string(format!(
      "&call-dylib-edn expected a method name, got {}",
      xs[1]
    )));
  };
  let mut ys: Vec<Edn> = vec![];
  for (idx, v) in xs.iter().enumerate() {
    if idx > 1 {
      ys.push(calcit_to_edn(v).map_err(CalcitErr::use_string)?);
    }
  }

  unsafe {
    let lib = libloading::Library::new(&lib_name).expect("dylib not found");
    let func: libloading::Symbol<EdnFfi> = lib.get(method.as_bytes()).expect("dy function not found");
    let ret = func(ys.to_owned()).map_err(CalcitErr::use_string)?;
    Ok(edn_to_calcit(&ret))
  }
}

pub fn echo(xs: &CalcitItems) -> Result<Calcit, CalcitErr> {
  let mut s = String::from("");
  for (idx, x) in xs.iter().enumerate() {
    if idx > 0 {
      s.push(' ');
    }
    s.push_str(&x.turn_string());
  }
  println!("{}", s);
  Ok(Calcit::Nil)
}

fn send_ffi_message(op: String, items: CalcitItems) {
  let mut ref_messages = PROGRAM_FFI_MESSAGES.write().unwrap();
  (*ref_messages).push((op, items))
}

pub fn take_ffi_messages() -> Result<Vec<(String, CalcitItems)>, String> {
  let mut messages: Vec<(String, CalcitItems)> = vec![];
  let mut ref_messages = PROGRAM_FFI_MESSAGES.write().unwrap();
  for m in (*ref_messages).iter() {
    messages.push(m.to_owned())
  }
  (*ref_messages).clear();
  Ok(messages)
}

fn ffi_message(xs: &CalcitItems) -> Result<Calcit, CalcitErr> {
  if !xs.is_empty() {
    match &xs[0] {
      Calcit::Str(s) | Calcit::Symbol(s, ..) => {
        let items = xs.to_owned().slice(1..);
        send_ffi_message(s.to_owned(), items);
        Ok(Calcit::Nil)
      }
      a => Err(CalcitErr::use_string(format!(
        "&ffi-message expected string, got {}",
        a
      ))),
    }
  } else {
    Err(CalcitErr::use_str("&ffi-message expected arguments but got empty"))
  }
}
