mod client;
use client::ClientRenderer;

fn main() {
    let renderer = ClientRenderer::new();

    renderer.render();
}
