use nu_plugin::{serve_plugin, MsgPackSerializer};

use nu_plugin_nustart::NuStartPlugin;

fn main() {
    serve_plugin(&NuStartPlugin, MsgPackSerializer);
}
