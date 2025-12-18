use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_nudo::Nudo;
fn main() {
    serve_plugin(&Nudo, MsgPackSerializer {});
}

