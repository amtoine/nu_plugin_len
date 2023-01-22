use nu_plugin::{serve_plugin, MsgPackSerializer, Plugin, EvaluatedCall, LabeledError};
use nu_protocol::{Signature, Value};

pub struct StrLen;

impl Plugin for StrLen {
    fn signature(&self) -> Vec<Signature> {
        vec![Signature::build("my_str_len")
            .usage("Give the length of the input string")
            .input_type(nu_protocol::Type::String)]
    }

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
