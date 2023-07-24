#![deny(clippy::all)]

use heatshrink::Config;
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

  let mut output_buf: Vec<u8> = vec![0; 16 + (2 * (2 ^ input.len()))];
  match heatshrink::encode(input_ref, output_buf.as_mut_slice(), &config) {
    Ok(out) => {
      return Ok(Vec::from(out).into());
    },
    Err(err) => {
      return Err(Error::new(Status::GenericFailure, format!("{:?}", err)));
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

  let mut output_buf: Vec<u8> = vec![0; 16 + (2 * (2 ^ input.len()))];
  match heatshrink::decode(input_ref, output_buf.as_mut_slice(), &config) {
    Ok(out) => {
      return Ok(Vec::from(out).into());
    },
    Err(err) => {
      return Err(Error::new(Status::GenericFailure, format!("{:?}", err)));
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
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input_ref: &[u8] = self.input.as_ref();
    let config = match Config::new(self.window_size, self.lookahead_size) {
      Ok(cfg) => cfg,
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, err));
      }
    };
  
    let mut output_buf: Vec<u8> = vec![0; 16 + (2 * (2 ^ self.input.len()))];
    match heatshrink::encode(input_ref, output_buf.as_mut_slice(), &config) {
      Ok(out) => {
        return Ok(Vec::from(out));
      },
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, format!("{:?}", err)));
      }
    };
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

pub struct DecodeTask {
  pub(crate) input: Buffer,
  pub(crate) window_size: u8,
  pub(crate) lookahead_size: u8,
}

impl Task for DecodeTask {
  type Output = Vec<u8>;
  type JsValue = Buffer;

  fn compute(&mut self) -> Result<Self::Output> {
    let input_ref: &[u8] = self.input.as_ref();
    let config = match Config::new(self.window_size, self.lookahead_size) {
      Ok(cfg) => cfg,
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, err));
      }
    };
  
    let mut output_buf: Vec<u8> = vec![0; 16 + (2 * (2 ^ self.input.len()))];
    match heatshrink::decode(input_ref, output_buf.as_mut_slice(), &config) {
      Ok(out) => {
        return Ok(Vec::from(out));
      },
      Err(err) => {
        return Err(Error::new(Status::GenericFailure, format!("{:?}", err)));
      }
    };
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
    Ok(output.into())
  }
}

#[napi(ts_return_type = "Promise<Buffer>")]
pub fn encode(input: Buffer, window_size: u8, lookahead_size: u8, signal: Option<AbortSignal>) -> Result<AsyncTask<EncodeTask>> {
  Ok(AsyncTask::with_optional_signal(EncodeTask { input, window_size, lookahead_size }, signal))
}

#[napi(ts_return_type = "Promise<Buffer>")]
pub fn decode(input: Buffer, window_size: u8, lookahead_size: u8, signal: Option<AbortSignal>) -> Result<AsyncTask<DecodeTask>> {
  Ok(AsyncTask::with_optional_signal(DecodeTask { input, window_size, lookahead_size }, signal))
}
