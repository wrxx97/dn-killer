fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=assets/icon.rc");
    embed_resource::compile("assets/icon.rc", embed_resource::NONE)
        .manifest_optional()
        .expect("failed to embed the DN Killer icon");
}
