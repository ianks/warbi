use wit_bindgen_host_wasmtime_rust::wasmtime::Trap;

use crate::bindings::ruby_value_resource;

/// The host context for the `ruby-value` module.
#[derive(Debug)]
pub struct RubyValue {}

impl ruby_value_resource::RubyValueResource for RubyValue {
    type RubyValue = magnus::Value;
    type Error = magnus::Error;

    fn error_to_trap(&mut self, err: Self::Error) -> Trap {
        Trap::new(err.to_string())
    }

    fn ruby_value_from_str(&mut self, input: &str) -> Result<Self::RubyValue, Self::Error> {
        Ok(input.into())
    }

    fn ruby_value_from_i64(&mut self, input: i64) -> Result<Self::RubyValue, Self::Error> {
        Ok(input.into())
    }

    fn ruby_value_from_f64(&mut self, input: f64) -> Result<Self::RubyValue, Self::Error> {
        Ok(input.into())
    }

    fn ruby_value_from_bool(&mut self, input: bool) -> Result<Self::RubyValue, Self::Error> {
        Ok(input.into())
    }

    fn ruby_value_to_string(&mut self, rb_self: &Self::RubyValue) -> Result<String, Self::Error> {
        Ok(rb_self.to_string())
    }
}
