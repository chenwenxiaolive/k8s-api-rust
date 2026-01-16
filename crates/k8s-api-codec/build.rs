use glob::glob;
use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|dir| dir.parent())
        .and_then(|dir| dir.parent())
        .expect("repo root");
    let kube_root = repo_root.join("kubernetes");
    if !kube_root.exists() {
        panic!("kubernetes repo not found at {}", kube_root.display());
    }

    let staging_root = kube_root.join("staging/src");
    let mut protos = Vec::new();

    let patterns = [
        staging_root.join("k8s.io/api/**/generated.proto"),
        staging_root.join("k8s.io/apimachinery/pkg/**/generated.proto"),
        staging_root.join("k8s.io/apiextensions-apiserver/pkg/apis/**/generated.proto"),
        staging_root.join("k8s.io/kube-aggregator/pkg/apis/**/generated.proto"),
    ];

    for pattern in patterns {
        let pattern = pattern.to_string_lossy().to_string();
        for entry in glob(&pattern).expect("glob pattern") {
            let path = entry.expect("glob entry");
            protos.push(path);
        }
    }

    protos.sort();
    if protos.is_empty() {
        panic!("no Kubernetes proto files found under {}", staging_root.display());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let descriptor_path = out_dir.join("k8s_api_descriptor_set.bin");

    let mut config = prost_build::Config::new();
    config.file_descriptor_set_path(&descriptor_path);
    config
        .compile_protos(
            &protos,
            &[
                staging_root,
                kube_root.join("vendor"),
                kube_root.join("third_party/protobuf"),
            ],
        )
        .expect("compile Kubernetes protos");

    println!("cargo:rerun-if-changed={}", kube_root.join("staging/src").display());
    println!("cargo:rerun-if-changed={}", kube_root.join("vendor").display());
    println!(
        "cargo:rerun-if-changed={}",
        kube_root.join("third_party/protobuf").display()
    );
}
