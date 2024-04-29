use std::collections::HashMap;

use derivative::Derivative;
use serde::Deserialize;

use super::binding_rendered_module::BindingRenderedModule;

#[napi_derive::napi(object)]
#[derive(Deserialize, Default, Derivative)]
#[serde(rename_all = "camelCase")]
#[derive(Debug)]
pub struct RenderedChunk {
  // PreRenderedChunk
  pub is_entry: bool,
  pub is_dynamic_entry: bool,
  pub facade_module_id: Option<String>,
  pub module_ids: Vec<String>,
  pub exports: Vec<String>,
  // RenderedChunk
  pub file_name: String,
  #[serde(skip)]
  pub modules: HashMap<String, BindingRenderedModule>,
  pub imports: Vec<String>,
}

impl From<rolldown_common::RenderedChunk> for RenderedChunk {
  fn from(value: rolldown_common::RenderedChunk) -> Self {
    Self {
      is_entry: value.is_entry,
      is_dynamic_entry: value.is_dynamic_entry,
      facade_module_id: value.facade_module_id.map(|x| x.to_string()),
      module_ids: value.module_ids.into_iter().map(|x| x.to_string()).collect(),
      exports: value.exports,
      file_name: value.file_name.to_string(),
      modules: value
        .modules
        .into_iter()
        .map(|(key, value)| (key.to_string(), value.into()))
        .collect(),
      imports: value.imports.iter().map(|x| x.to_string()).collect(),
    }
  }
}
