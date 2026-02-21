#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![forbid(unsafe_code)]

use crate::graph::Node;
use serde_json::Value;

pub struct ExpressionContext<'a> {
    pub nodes: &'a [Node],
}

impl<'a> ExpressionContext<'a> {
    #[must_use]
    pub const fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    #[must_use]
    pub fn resolve(&self, expr: &str) -> Value {
        let trimmed = expr.trim();

        // 1. Path Resolution: $node["Name"].json.path
        if let Some(node_part) = trimmed.strip_prefix("$node[\"") {
            if let Some((node_name, path_part)) = node_part.split_once("\"]") {
                let path = path_part
                    .strip_prefix(".json.")
                    .map_or(path_part, |prefix| prefix);
                let resolved = self
                    .nodes
                    .iter()
                    .find(|n| n.name == node_name)
                    .and_then(|n| n.last_output.as_ref())
                    .and_then(|out| out.pointer(&format!("/{}", path.replace('.', "/"))));

                return resolved.map_or(Value::Null, std::clone::Clone::clone);
            }
        }

        // 2. Constant Math (Simple regex-free split)
        if let Some((left, right)) = trimmed.split_once(" + ") {
            return self.eval_binary_op(left, right, |a, b| Value::from(a + b));
        }
        if let Some((left, right)) = trimmed.split_once(" - ") {
            return self.eval_binary_op(left, right, |a, b| Value::from(a - b));
        }

        // 3. String Methods
        if let Some(base) = trimmed.strip_suffix(".to_uppercase()") {
            let val = self.resolve(base);
            if let Some(s) = val.as_str() {
                return Value::String(s.to_uppercase());
            }
        }
        if let Some(base) = trimmed.strip_suffix(".len()") {
            let val = self.resolve(base);
            if let Some(s) = val.as_str() {
                return Value::from(s.len());
            }
            if let Some(a) = val.as_array() {
                return Value::from(a.len());
            }
        }

        // 4. Literals
        if let Ok(n) = trimmed.parse::<f64>() {
            return Value::from(n);
        }
        if (trimmed.starts_with('"') && trimmed.ends_with('"'))
            || (trimmed.starts_with('\'') && trimmed.ends_with('\''))
        {
            return Value::String(trimmed[1..trimmed.len() - 1].to_string());
        }
        if trimmed == "true" {
            return Value::Bool(true);
        }
        if trimmed == "false" {
            return Value::Bool(false);
        }

        Value::String(trimmed.to_string())
    }

    fn eval_binary_op<F>(&self, left: &str, right: &str, op: F) -> Value
    where
        F: Fn(f64, f64) -> Value,
    {
        let lv = self.resolve(left);
        let rv = self.resolve(right);
        if let (Some(l), Some(r)) = (lv.as_f64(), rv.as_f64()) {
            return op(l, r);
        }
        Value::Null
    }
}
