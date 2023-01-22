//! A simple plugin that ships the `my_str_len` command to `nushell` to compute the length of
//! strings.
//!
//! This `nu_plugin_len` is greatly inspired by
//! - the [`nu_plugin_example`] official example plugin from the `nushell` source itself
//! - the [*Create a plugin (in `rust`)*] section of the old [contributors book] of `nushell`
//!
//! > **Note**  
//! > this plugin uses the `MsgPackSerializer` of `nu-plugin`
//!
//! # Examples
//! ```bash
//! > "this is an example string" | my_str_len
//! 25
//! ```
//!
//! [`nu_plugin_example`]: https://github.com/nushell/nushell/tree/b97bfe9297bed4c6063cdd27af0ac4ffe6c065ec/crates/nu_plugin_example
//! [*Create a plugin (in `rust`)*]: https://www.nushell.sh/contributor-book/plugins.html#creating-a-plugin-in-rust
//! [contributors book]: https://www.nushell.sh/contributor-book
use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, EvaluatedCall, LabeledError};
use nu_protocol::{Signature, Value};

/// The main structure used by the plugin protocol to communicate with `nushell`.
pub struct StrLen;

/// The implementation of the plugin protocol for our `my_str_len`.
impl Plugin for StrLen {
    /// Define the signature of `my_str_len`, taking a single string from `stdin`.
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("my_str_len")
            .usage("Give the length of the input string")
            .input_type(nu_protocol::Type::String)]
    }

    /// Compute the length of the input `String` value and return as an i64.
    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        Ok(Value::Int {
            val: input.as_string().unwrap().len() as i64,
            span: call.head,
        })
    }
}

fn main() {
    serve_plugin(&mut StrLen {}, MsgPackSerializer {});
}
