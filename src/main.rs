use nu_plugin::{MsgPackSerializer, serve_plugin};
use nu_plugin_nupm::Nudo;
fn main() {
    serve_plugin(&Nudo, MsgPackSerializer {});
}

