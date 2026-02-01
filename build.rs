use std::path::{Path, PathBuf};

const PROTO_DIR: &str = "proto";

// A simplified struct for input
struct ProtoSource {
    github: &'static str,
    version: &'static str,
    files: &'static [&'static str],
}

impl ProtoSource {
    /// Returns a Vec of (Download URL, Local Destination Path)
    fn get_download_map(&self) -> Vec<(String, String)> {
        self.files
            .iter()
            .map(|&src| {
                let url = format!(
                    "https://raw.githubusercontent.com/{}/{}/{}",
                    self.github, self.version, src
                );

                let dest = match self.github {
                    "moby/buildkit" | "tonistiigi/fsutil" => {
                        format!("github.com/{}/{}", self.github, src)
                    },
                    "planetscale/vtprotobuf" => src.trim_start_matches("include/").to_string(),
                    "protocolbuffers/protobuf" => src.trim_start_matches("src/").to_string(),
                    _ => src.to_string(), // googleapis case
                };

                (url, dest)
            })
            .collect()
    }
}

const BUILDKIT_PROTOS: ProtoSource = ProtoSource {
    github: "moby/buildkit",
    version: "v0.27.1",
    files: &[
        // only this one we want to compile
        "frontend/gateway/pb/gateway.proto",
        "solver/pb/ops.proto",
        "sourcepolicy/pb/policy.proto",
        "util/apicaps/pb/caps.proto",
        "api/types/worker.proto",
    ],
};

const FSUTILS_PROTOS: ProtoSource = ProtoSource {
    github: "tonistiigi/fsutil",
    version: "a2aa163d723fe2c00105350a49e9e2b02242f472",
    files: &["types/stat.proto"],
};

const PLANETSCALE_PROTOS: ProtoSource = ProtoSource {
    github: "planetscale/vtprotobuf",
    version: "0393e58bdf106fe0347e554d272a8f2c84d12461",
    files: &["include/github.com/planetscale/vtprotobuf/vtproto/ext.proto"],
};

const GOOGLE_RPC_PROTOS: ProtoSource = ProtoSource {
    github: "googleapis/googleapis",
    version: "2af421884dd468d565137215c946ebe4e245ae26",
    files: &["google/rpc/status.proto"],
};

const GOOGLE_PROTOS: ProtoSource = ProtoSource {
    github: "protocolbuffers/protobuf",
    version: "v3.11.4",
    files: &[
        "src/google/protobuf/any.proto",
        "src/google/protobuf/timestamp.proto",
        "src/google/protobuf/descriptor.proto",
    ],
};

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR")?);
    let proto_dir = out_dir.join(PROTO_DIR);
    // Create proto directory
    std::fs::create_dir_all(&proto_dir)?;

    let buildkit_protos = download_protos(&proto_dir, &BUILDKIT_PROTOS)?;
    let _fsutils_protos = download_protos(&proto_dir, &FSUTILS_PROTOS)?;
    let _planetscale_protos = download_protos(&proto_dir, &PLANETSCALE_PROTOS)?;
    let _google_rpc_protos = download_protos(&proto_dir, &GOOGLE_RPC_PROTOS)?;
    let _google_protos = download_protos(&proto_dir, &GOOGLE_PROTOS)?;

    // compile only "frontend/gateway/pb/gateway.proto"
    let to_compile = &[buildkit_protos[0].clone()];

    compile_protos(to_compile, &[proto_dir], &out_dir)?;
    Ok(())
}

fn compile_protos(
    protos: &[PathBuf],
    includes: &[PathBuf],
    out_dir: &Path,
) -> anyhow::Result<()> {
    println!("\nCompiling proto files with tonic-build...");

    tonic_prost_build::configure()
        .build_client(true)
        .build_server(false)
        .out_dir(out_dir)
        .compile_protos(&protos, includes)?;
    Ok(())
}

fn download_protos(
    proto_dir: &Path,
    loc: &ProtoSource,
) -> anyhow::Result<Vec<PathBuf>> {
    println!(
        "Downloading proto files from '{}' of '{}' version...",
        loc.github, loc.version,
    );

    loc.get_download_map()
        .iter()
        .map(|(url, dest)| {
            let dest_path = proto_dir.join(dest);
            download_file(&url, &dest_path)?;
            Ok(dest_path)
        })
        .collect()
}

/// Download a file from URL to destination path using reqwest
fn download_file(
    url: &str,
    dest: &Path,
) -> anyhow::Result<()> {
    const USER_AGENT: &str = "shell-cell-build-script";

    // Create parent directory if needed
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    // Use reqwest to download the file
    let client = reqwest::blocking::Client::builder()
        .user_agent(USER_AGENT)
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    let response = client.get(url).send()?;

    anyhow::ensure!(
        response.status().is_success(),
        "Failed to download {} - HTTP status: {}",
        url,
        response.status()
    );

    let content = response.bytes()?;
    anyhow::ensure!(!content.is_empty(), "Empty response from {}", url);
    std::fs::write(dest, content)?;

    Ok(())
}
