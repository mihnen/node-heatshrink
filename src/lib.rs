#![deny(clippy::all)]

use heatshrink::Config;
use napi::JsBuffer;
use napi::Task;
use napi::bindgen_prelude::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn encode_sync(input: Buffer, window_size: u8, lookahead_size: u8) -> Result<Buffer> {
  let input_ref: &[u8] = input.as_ref();
  let config = match Config::new(window_size, lookahead_size) {
    Ok(cfg) => cfg,
    Err(err) => {
      return Err(Error::new(Status::GenericFailure, err));
    }
  };

  let mut output_buf: Vec<u8> = vec![0; input.len() * 2];
  match heatshrink::encode(input_ref, output_buf.as_mut_slice(), &config) {
    Ok(out) => {
      return Ok(Vec::from(out).into());
    },
    Err(_) => {
      return Err(Error::new(Status::GenericFailure, format!("Output buffer is too small, length: {}", output_buf.len())));
    }
  };
}

#[napi]
pub fn decode_sync(input: Buffer, window_size: u8, lookahead_size: u8) -> Result<Buffer> {
  let input_ref: &[u8] = input.as_ref();
  let config = match Config::new(window_size, lookahead_size) {
    Ok(cfg) => cfg,
    Err(err) => {
      return Err(Error::new(Status::GenericFailure, err));
    }
  };

  let mut output_buf: Vec<u8> = vec![0; input.len() * 2];
  match heatshrink::decode(input_ref, output_buf.as_mut_slice(), &config) {
    Ok(out) => {
      return Ok(Vec::from(out).into());
    },
    Err(_) => {
      return Err(Error::new(Status::GenericFailure, format!("Output buffer is too small, length: {}", output_buf.len())));
    }
  };
}

pub struct EncodeTask {
  pub(crate) input: Buffer,
  pub(crate) window_size: u8,
  pub(crate) lookahead_size: u8,
}

impl Task for EncodeTask {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input_ref: &[u8] = self.input.as_ref();
    let config = match Config::new(self.window_size, self.lookahead_size) {
      Ok(cfg) => cfg,
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, err));
      }
    };
  
    let mut output_buf: Vec<u8> = vec![0; self.input.len() * 2];
    match heatshrink::encode(input_ref, output_buf.as_mut_slice(), &config) {
      Ok(out) => {
        return Ok(Vec::from(out));
      },
      Err(_) => {
        return Err(Error::new(Status::GenericFailure, format!("Output buffer is too small, length: {}", output_buf.len())));
      }
    };
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    match env.create_buffer_with_data(output) {
      Ok(buf_val) => {
        return Ok(buf_val.into_raw());
      },
      Err(err) => {
        return Err(err);
      }
    }
  }
}

pub struct DecodeTask {
  pub(crate) input: Buffer,
  pub(crate) window_size: u8,
  pub(crate) lookahead_size: u8,
}

impl Task for DecodeTask {
  type Output = Vec<u8>;
  type JsValue = JsBuffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input_ref: &[u8] = self.input.as_ref();
    let config = match Config::new(self.window_size, self.lookahead_size) {
      Ok(cfg) => cfg,
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, err));
      }
    };
  
    let mut output_buf: Vec<u8> = vec![0; self.input.len() * 2];
    match heatshrink::decode(input_ref, output_buf.as_mut_slice(), &config) {
      Ok(out) => {
        return Ok(Vec::from(out));
      },
      Err(_) => {
        return Err(Error::new(Status::GenericFailure, format!("Output buffer is too small, length: {}", output_buf.len())));
      }
    };
  }

  fn resolve(&mut self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    match env.create_buffer_with_data(output) {
      Ok(buf_val) => {
        return Ok(buf_val.into_raw());
      },
      Err(err) => {
        return Err(err);
      }
    }
  }
}

#[napi]
pub fn encode(input: Buffer, window_size: u8, lookahead_size: u8, signal: Option<AbortSignal>) -> AsyncTask<EncodeTask> {
  AsyncTask::with_optional_signal(EncodeTask { input, window_size, lookahead_size }, signal)
}

#[napi]
pub fn decode(input: Buffer, window_size: u8, lookahead_size: u8, signal: Option<AbortSignal>) -> AsyncTask<DecodeTask> {
  AsyncTask::with_optional_signal(DecodeTask { input, window_size, lookahead_size }, signal)
}
