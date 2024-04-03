use super::interface::{EngineInterfaceManager, Interface, InterfaceManager};
use crate::{
    protocol::{PluginCall, PluginCallId, PluginInput, ProtocolInfo},
    JsonSerializer, PluginEncoder, PluginOutput,
};
use nu_protocol::PluginSignature;
use std::io::Write;

/// An interface to dump out JSON format messages
/// to facilitate writing plugins in unsupported languages.

/// As used by e.g. nu_plugin_bash_env.

pub struct JsonDumper {
    encoder: JsonSerializer,
    manager: EngineInterfaceManager,
}

impl JsonDumper {
    pub fn new() -> Self {
        let stdout = std::io::stdout();
        let encoder = JsonSerializer {};
        let manager = EngineInterfaceManager::new((stdout, encoder));
        Self { encoder, manager }
    }

    pub fn encoder(&self) {
        let mut stdout = std::io::stdout();
        let encoding = self.encoder.name();
        let length = encoding.len() as u8;
        let mut encoding_content: Vec<u8> = encoding.as_bytes().to_vec();
        encoding_content.insert(0, length);
        stdout
            .write_all(&encoding_content)
            .expect("Failed to write encoding");
        stdout.flush().expect("Failed to flush encoding");
    }

    pub fn hello(&self, version: &str) {
        let interface = self.manager.get_interface();
        let hello_msg = PluginOutput::Hello(ProtocolInfo {
            version: version.to_string(),
            ..ProtocolInfo::default()
        });

        interface.write(hello_msg).expect("Failed to write hello");
    }

    pub fn signature(&mut self, version: &str, call_id: usize, sig: Vec<PluginSignature>) {
        // before we can send a signature we have to get the engine into the expected state,
        // that is, having received both hello and signature input
        let hello_input = PluginInput::Hello(ProtocolInfo {
            version: version.to_string(),
            ..ProtocolInfo::default()
        });
        self.manager
            .consume(hello_input)
            .expect("Failed to consume hand-crafted hello input");

        let signature_input = PluginInput::Call(call_id, PluginCall::Signature);
        self.manager
            .consume(signature_input)
            .expect("Failed to consume hand-crafted signature input");

        let interface = self.manager.get_interface();
        interface
            .write_signature(sig)
            .expect("Failed to write signature");
    }
}

impl Default for JsonDumper {
    fn default() -> Self {
        Self::new()
    }
}
