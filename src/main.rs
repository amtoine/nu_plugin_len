//! A simple plugin that ships the `len` command to `nushell` to compute the length of
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
//! > "this is an example string" | len
//! 25
//! ```
//!
//! [`nu_plugin_example`]: https://github.com/nushell/nushell/tree/b97bfe9297bed4c6063cdd27af0ac4ffe6c065ec/crates/nu_plugin_example
//! [*Create a plugin (in `rust`)*]: https://www.nushell.sh/contributor-book/plugins.html#creating-a-plugin-in-rust
//! [contributors book]: https://www.nushell.sh/contributor-book
use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, Signature, Type, Value};

/// The main structure used by the plugin protocol to communicate with `nushell`.
pub struct StrLen;

/// The implementation of the plugin protocol for our `len`.
impl Plugin for StrLen {
    /// Define the signature of `len`, taking a single string from `stdin`.
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("len")
            .usage("Give the length of the input string")
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::String, Type::Int)])
            .category(Category::Experimental)]
    }

    /// Compute the length of the input `String` value and return as an i64.
    fn run(
        &mut self,
        _name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        match input.get_type() {
            Type::String => (),
            _ => {
                let error_message = format!(
                    "Input is not a string: found {}",
                    input.get_type().to_string()
                );
                return Err(LabeledError {
                    label: error_message.clone(),
                    msg: error_message,
                    span: Some(input.span()?),
                });
            }
        }

        match input.as_string() {
            Ok(s) => Ok(Value::Int {
                val: s.len() as i64,
                span: call.head,
            }),
            Err(e) => {
                return Err(LabeledError {
                    label: "Unable to convert input into a string".to_string(),
                    msg: e.to_string(),
                    span: Some(input.span()?),
                })
            }
        }
    }
}

fn main() {
    serve_plugin(&mut StrLen {}, MsgPackSerializer {});
}
