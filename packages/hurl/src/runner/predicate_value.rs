/*
 * Hurl (https://hurl.dev)
 * Copyright (C) 2023 Orange
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *          http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use std::collections::HashMap;

use hurl_core::ast::*;

use crate::runner::body::eval_file; // TODO move function out of body module
use crate::runner::error::Error;
use crate::runner::expr::eval_expr;
use crate::runner::multiline::eval_multiline;
use crate::runner::template::eval_template;
use crate::runner::{Number as ValueNumber, Value};
use crate::util::path::ContextDir;

pub fn eval_predicate_value(
    predicate_value: &PredicateValue,
    variables: &HashMap<String, Value>,
    context_dir: &ContextDir,
) -> Result<Value, Error> {
    match predicate_value {
        PredicateValue::String(template) => {
            let s = eval_template(template, variables)?;
            Ok(Value::String(s))
        }
        PredicateValue::MultilineString(value) => {
            let s = eval_multiline(value, variables)?;
            Ok(Value::String(s))
        }
        PredicateValue::Bool(value) => Ok(Value::Bool(*value)),
        PredicateValue::Null => Ok(Value::Null),
        PredicateValue::Number(value) => Ok(Value::Number(eval_number(value))),
        PredicateValue::File(value) => {
            let value = eval_file(&value.filename, context_dir)?;
            Ok(Value::Bytes(value))
        }
        PredicateValue::Hex(value) => Ok(Value::Bytes(value.value.clone())),
        PredicateValue::Base64(value) => Ok(Value::Bytes(value.value.clone())),
        PredicateValue::Expression(expr) => {
            let value = eval_expr(expr, variables)?;
            Ok(value)
        }
        PredicateValue::Regex(regex) => Ok(Value::Regex(regex.inner.clone())),
    }
}

pub fn eval_predicate_value_template(
    predicate_value: &PredicateValue,
    variables: &HashMap<String, Value>,
) -> Result<String, Error> {
    match predicate_value {
        PredicateValue::String(template) => eval_template(template, variables),
        PredicateValue::Regex(regex) => Ok(regex.inner.to_string()),
        // All others value should have failed in parsing:
        _ => panic!("expect a string or a regex predicate value"),
    }
}

fn eval_number(number: &Number) -> ValueNumber {
    match number {
        Number::Float(value) => ValueNumber::Float(value.value),
        Number::Integer(value) => ValueNumber::Integer(*value),
        Number::BigInteger(value) => ValueNumber::BigInteger(value.clone()),
    }
}
