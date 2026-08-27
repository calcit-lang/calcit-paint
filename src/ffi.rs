use std::panic::{catch_unwind, AssertUnwindSafe};
use std::{mem::size_of, ptr, slice};

use cirru_edn::Edn;

pub const FFI_PROTOCOL_VERSION: u32 = 1;
pub const FFI_STATUS_OK: i32 = 0;
pub const FFI_STATUS_CALLBACK_ERROR: i32 = 10;
pub const FFI_STATUS_INVALID_PAYLOAD: i32 = 8;
pub const FFI_STATUS_INTERNAL_ERROR: i32 = 9;
const MAX_BUFFER_BYTES: usize = 256 * 1024 * 1024;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalcitFfiBuffer {
  pub ptr: *mut u8,
  pub len: usize,
  pub cap: usize,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalcitFfiAsyncTaskV1 {
  pub protocol_version: u32,
  pub struct_size: u32,
  pub handle: u64,
  pub kind: u32,
  pub flags: u32,
}

pub type BlockingHostInvoke = unsafe extern "C" fn(u64, u64, *const u8, usize, *mut CalcitFfiBuffer) -> i32;
pub type BlockingHostFinish = unsafe extern "C" fn(u64, u64) -> i32;
pub type BlockingHostFreeBuffer = unsafe extern "C" fn(u64, u64, CalcitFfiBuffer) -> i32;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CalcitFfiBlockingHostV1 {
  pub protocol_version: u32,
  pub struct_size: u32,
  pub context: u64,
  pub invoke: Option<BlockingHostInvoke>,
  pub finish: Option<BlockingHostFinish>,
  pub free_buffer: Option<BlockingHostFreeBuffer>,
}

unsafe fn read_abi_header<T>(value: *const T) -> Result<(u32, u32), i32> {
  if value.is_null() {
    return Err(FFI_STATUS_INVALID_PAYLOAD);
  }
  let bytes = value.cast::<u8>();
  // SAFETY: every versioned descriptor starts with two readable u32 fields.
  let protocol_version = unsafe { ptr::read_unaligned(bytes.cast::<u32>()) };
  // SAFETY: the second header field starts four bytes after the first.
  let struct_size = unsafe { ptr::read_unaligned(bytes.add(size_of::<u32>()).cast::<u32>()) };
  Ok((protocol_version, struct_size))
}

unsafe fn copy_descriptor<T: Copy>(value: *const T) -> Result<T, i32> {
  // SAFETY: the exported adapter receives a versioned descriptor pointer.
  let (version, size) = unsafe { read_abi_header(value) }?;
  if version != FFI_PROTOCOL_VERSION || size < size_of::<T>() as u32 {
    return Err(FFI_STATUS_INVALID_PAYLOAD);
  }
  // SAFETY: the validated size covers every v1 field.
  Ok(unsafe { ptr::read_unaligned(value) })
}

#[no_mangle]
pub extern "C" fn calcit_ffi_buffer_version() -> u32 {
  FFI_PROTOCOL_VERSION
}

#[no_mangle]
pub extern "C" fn calcit_ffi_async_version() -> u32 {
  FFI_PROTOCOL_VERSION
}

/// Release a module-owned buffer returned through buffer protocol v1.
///
/// # Safety
///
/// The buffer metadata must be the exact value returned by this module.
#[no_mangle]
pub unsafe extern "C" fn calcit_ffi_buffer_free(buffer: CalcitFfiBuffer) {
  if buffer.ptr.is_null() {
    return;
  }
  // SAFETY: Calcit returns exactly the metadata produced by `write_output`.
  drop(unsafe { Vec::from_raw_parts(buffer.ptr, buffer.len, buffer.cap) });
}

unsafe fn decode_request(request_ptr: *const u8, request_len: usize) -> Result<Vec<Edn>, String> {
  if request_ptr.is_null() && request_len != 0 {
    return Err("FFI request pointer is null".to_owned());
  }
  if request_len > MAX_BUFFER_BYTES {
    return Err(format!("FFI request exceeds {MAX_BUFFER_BYTES} bytes"));
  }
  let bytes = if request_len == 0 {
    &[]
  } else {
    // SAFETY: the host keeps request bytes readable for this call.
    unsafe { slice::from_raw_parts(request_ptr, request_len) }
  };
  let source = std::str::from_utf8(bytes).map_err(|error| format!("FFI request is not UTF-8: {error}"))?;
  let data = cirru_edn::parse(source).map_err(|error| format!("FFI request is not valid Cirru EDN: {error}"))?;
  let Edn::List(args) = data else {
    return Err("FFI request must be a Cirru EDN list".to_owned());
  };
  Ok(args)
}

fn encode_edn(value: &Edn) -> Result<Vec<u8>, String> {
  cirru_edn::format(value, true)
    .map(String::into_bytes)
    .map_err(|error| format!("failed to encode Cirru EDN: {error}"))
}

unsafe fn write_output(output: *mut CalcitFfiBuffer, bytes: Vec<u8>) -> i32 {
  if output.is_null() {
    return FFI_STATUS_INVALID_PAYLOAD;
  }
  let mut bytes = std::mem::ManuallyDrop::new(bytes);
  let buffer = CalcitFfiBuffer {
    ptr: bytes.as_mut_ptr(),
    len: bytes.len(),
    cap: bytes.capacity(),
  };
  // SAFETY: the caller supplied a writable output slot.
  unsafe { output.write(buffer) };
  FFI_STATUS_OK
}

fn copy_host_buffer(buffer: CalcitFfiBuffer) -> Result<Vec<u8>, String> {
  if buffer.len > buffer.cap || buffer.len > MAX_BUFFER_BYTES || (buffer.ptr.is_null() && buffer.len != 0) {
    return Err("Calcit callback returned invalid buffer metadata".to_owned());
  }
  if buffer.len == 0 {
    Ok(vec![])
  } else {
    // SAFETY: the host keeps the allocation alive until `free_buffer`.
    Ok(unsafe { slice::from_raw_parts(buffer.ptr, buffer.len) }.to_vec())
  }
}

pub fn invoke_blocking_callback(
  host: CalcitFfiBlockingHostV1,
  task: CalcitFfiAsyncTaskV1,
  args: Vec<Edn>,
) -> Result<Edn, String> {
  let invoke = host
    .invoke
    .ok_or_else(|| "blocking host is missing invoke".to_owned())?;
  let free_buffer = host
    .free_buffer
    .ok_or_else(|| "blocking host is missing free_buffer".to_owned())?;
  let payload = encode_edn(&Edn::List(args))?;
  let mut output = CalcitFfiBuffer {
    ptr: ptr::null_mut(),
    len: 0,
    cap: 0,
  };
  // SAFETY: copied host function pointers are valid for the blocking call.
  let status = unsafe { invoke(host.context, task.handle, payload.as_ptr(), payload.len(), &mut output) };
  let has_output = !output.ptr.is_null() || output.len != 0 || output.cap != 0;
  if !has_output {
    return Err(format!("Calcit callback returned no output buffer, status {status}"));
  }
  let copied = copy_host_buffer(output);
  // SAFETY: this is the exact host-owned buffer returned by `invoke`.
  let free_status = unsafe { free_buffer(host.context, task.handle, output) };
  if free_status != FFI_STATUS_OK {
    return Err(format!(
      "Calcit host rejected callback buffer release with status {free_status}"
    ));
  }
  let bytes = copied?;
  if status == FFI_STATUS_OK {
    let source =
      std::str::from_utf8(&bytes).map_err(|error| format!("Calcit callback result is not UTF-8: {error}"))?;
    cirru_edn::parse(source).map_err(|error| format!("Calcit callback result is not valid Cirru EDN: {error}"))
  } else if status == FFI_STATUS_CALLBACK_ERROR {
    Err(String::from_utf8_lossy(&bytes).into_owned())
  } else {
    Err(format!("Calcit host rejected blocking callback with status {status}"))
  }
}

pub unsafe fn run_buffer_adapter(
  request_ptr: *const u8,
  request_len: usize,
  output: *mut CalcitFfiBuffer,
  method: fn(Vec<Edn>) -> Result<Edn, String>,
) -> i32 {
  match catch_unwind(AssertUnwindSafe(|| {
    // SAFETY: forwarded from the exported buffer ABI contract.
    let args = unsafe { decode_request(request_ptr, request_len) }?;
    method(args).and_then(|value| encode_edn(&value))
  })) {
    Ok(Ok(bytes)) => unsafe { write_output(output, bytes) },
    Ok(Err(error)) => {
      let _ = unsafe { write_output(output, error.into_bytes()) };
      1
    }
    Err(_) => {
      let _ = unsafe { write_output(output, b"calcit-paint buffer adapter panicked".to_vec()) };
      FFI_STATUS_INTERNAL_ERROR
    }
  }
}

pub unsafe fn run_blocking_adapter(
  request_ptr: *const u8,
  request_len: usize,
  task: *const CalcitFfiAsyncTaskV1,
  host: *const CalcitFfiBlockingHostV1,
  output: *mut CalcitFfiBuffer,
  method: impl FnOnce(Vec<Edn>, CalcitFfiAsyncTaskV1, CalcitFfiBlockingHostV1) -> Result<Edn, String>,
) -> i32 {
  match catch_unwind(AssertUnwindSafe(|| {
    // SAFETY: descriptors and request bytes follow blocking protocol v1.
    let task = unsafe { copy_descriptor(task) }.map_err(|status| format!("invalid task descriptor: {status}"))?;
    // SAFETY: the same versioned descriptor contract applies to the host table.
    let host = unsafe { copy_descriptor(host) }.map_err(|status| format!("invalid host descriptor: {status}"))?;
    if host.invoke.is_none() || host.free_buffer.is_none() {
      return Err("blocking host is missing required operations".to_owned());
    }
    // SAFETY: request memory is copied before the event loop starts.
    let args = unsafe { decode_request(request_ptr, request_len) }?;
    method(args, task, host).and_then(|value| encode_edn(&value))
  })) {
    Ok(Ok(bytes)) => unsafe { write_output(output, bytes) },
    Ok(Err(error)) => {
      let _ = unsafe { write_output(output, error.into_bytes()) };
      1
    }
    Err(_) => {
      let _ = unsafe { write_output(output, b"calcit-paint blocking adapter panicked".to_vec()) };
      FFI_STATUS_INTERNAL_ERROR
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn echo(args: Vec<Edn>) -> Result<Edn, String> {
    args.into_iter().next().ok_or_else(|| "missing argument".to_owned())
  }

  unsafe extern "C" fn echo_invoke(
    _context: u64,
    _task_handle: u64,
    payload_ptr: *const u8,
    payload_len: usize,
    output: *mut CalcitFfiBuffer,
  ) -> i32 {
    let value = unsafe { decode_request(payload_ptr, payload_len) }
      .ok()
      .and_then(|args| args.into_iter().next());
    match value.and_then(|value| encode_edn(&value).ok()) {
      Some(bytes) => unsafe { write_output(output, bytes) },
      None => FFI_STATUS_INVALID_PAYLOAD,
    }
  }

  unsafe extern "C" fn echo_free(_context: u64, _task_handle: u64, buffer: CalcitFfiBuffer) -> i32 {
    unsafe { calcit_ffi_buffer_free(buffer) };
    FFI_STATUS_OK
  }

  #[test]
  fn buffer_adapter_round_trips() {
    let request = cirru_edn::format(&Edn::List(vec![Edn::Number(7.0)]), true).unwrap();
    let mut output = CalcitFfiBuffer {
      ptr: ptr::null_mut(),
      len: 0,
      cap: 0,
    };
    assert_eq!(
      unsafe { run_buffer_adapter(request.as_ptr(), request.len(), &mut output, echo) },
      FFI_STATUS_OK
    );
    let bytes = unsafe { slice::from_raw_parts(output.ptr, output.len) };
    assert_eq!(
      cirru_edn::parse(std::str::from_utf8(bytes).unwrap()).unwrap(),
      Edn::Number(7.0)
    );
    unsafe { calcit_ffi_buffer_free(output) };
  }

  #[test]
  fn buffer_adapter_reports_malformed_request_without_unwinding() {
    let mut output = CalcitFfiBuffer {
      ptr: ptr::null_mut(),
      len: 0,
      cap: 0,
    };
    assert_eq!(unsafe { run_buffer_adapter(ptr::null(), 1, &mut output, echo) }, 1);
    let diagnostic = unsafe { slice::from_raw_parts(output.ptr, output.len) };
    assert!(String::from_utf8_lossy(diagnostic).contains("pointer is null"));
    unsafe { calcit_ffi_buffer_free(output) };
  }

  #[test]
  fn blocking_callback_round_trips_and_releases_host_output() {
    let task = CalcitFfiAsyncTaskV1 {
      protocol_version: FFI_PROTOCOL_VERSION,
      struct_size: size_of::<CalcitFfiAsyncTaskV1>() as u32,
      handle: 7,
      kind: 1,
      flags: 1,
    };
    let host = CalcitFfiBlockingHostV1 {
      protocol_version: FFI_PROTOCOL_VERSION,
      struct_size: size_of::<CalcitFfiBlockingHostV1>() as u32,
      context: 11,
      invoke: Some(echo_invoke),
      finish: None,
      free_buffer: Some(echo_free),
    };
    assert_eq!(
      invoke_blocking_callback(host, task, vec![Edn::Number(9.0)]).unwrap(),
      Edn::Number(9.0)
    );
  }
}
