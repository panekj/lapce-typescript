use anyhow::{anyhow, Result};
use lapce_plugin::{
  psp_types::{
    lsp_types::{
      request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, InitializeResult,
      Url,
    },
    Request,
  },
  register_plugin, LapcePlugin, VoltEnvironment, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

macro_rules! string {
  ( $x:expr ) => {
    String::from($x)
  };
}

macro_rules! ok {
  ( $x:expr ) => {
    match ($x) {
      | Ok(v) => v,
      | Err(e) => return Err(anyhow!(e)),
    }
  };
}

type LspParams = (Url, Vec<String>, DocumentSelector, Option<Value>);

fn initialize(params: InitializeParams) -> Result<LspParams> {
  let document_selector: DocumentSelector = vec![
    DocumentFilter {
      language: Some(string!("typescript")),
      pattern: Some(string!("**/*.ts")),
      scheme: None,
    },
    DocumentFilter {
      language: Some(string!("javascript")),
      pattern: Some(string!("**/*.js")),
      scheme: None,
    },
    DocumentFilter {
      language: Some(string!("typescriptreact")),
      pattern: Some(string!("**/*.tsx")),
      scheme: None,
    },
    DocumentFilter {
      language: Some(string!("javascriptreact")),
      pattern: Some(string!("**/*.jsx")),
      scheme: None,
    },
  ];
  let mut server_args = vec![string!("--stdio")];

  if let Some(options) = params.initialization_options.as_ref() {
    if let Some(volt) = options.get("volt") {
      if let Some(args) = volt.get("serverArgs") {
        if let Some(args) = args.as_array() {
          if !args.is_empty() {
            server_args = vec![];
          }
          for arg in args {
            if let Some(arg) = arg.as_str() {
              server_args.push(arg.to_string());
            }
          }
        }
      }

      if let Some(server_path) = volt.get("serverPath") {
        if let Some(server_path) = server_path.as_str() {
          if !server_path.is_empty() {
            let server_uri = ok!(Url::parse(&format!("urn:{server_path}")));
            return Ok((
              server_uri,
              server_args,
              document_selector,
              params.initialization_options,
            ));
          }
        }
      }
    }
  }

  let server_uri = match VoltEnvironment::operating_system().as_deref() {
    | Ok("windows") => ok!(Url::parse("urn:typescript-language-server.cmd")),
    | _ => ok!(Url::parse("urn:typescript-language-server")),
  };

  Ok((
    server_uri,
    server_args,
    document_selector,
    params.initialization_options,
  ))
}

impl LapcePlugin for State {
  fn handle_request(&mut self, id: u64, method: String, params: Value) {
    #[allow(clippy::single_match)]
    match method.as_str() {
      | Initialize::METHOD => {
        let params: InitializeParams = serde_json::from_value(params).unwrap();
        match initialize(params) {
          | Ok((uri, args, filters, options)) => {
            PLUGIN_RPC.start_lsp(uri, args, filters, options).unwrap();
            PLUGIN_RPC
              .host_success(id, InitializeResult::default())
              .unwrap()
          }
          | Err(err) => PLUGIN_RPC.host_error(id, err.to_string()).unwrap(),
        }
      }
      | o => PLUGIN_RPC
        .host_error(id, format!("plugin doesn't understand request '{o}'"))
        .unwrap(),
    }
  }
}
