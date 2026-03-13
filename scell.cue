_gh_token: string

main: {
	from_image: "rust:1.93-trixie"
	env: [
		// claude code instalation path
		"PATH=\"/root/.local/bin:$PATH\"",
		"GH_TOKEN=\"\(_gh_token)\""
	]
	build: [
		// Prepare Rust
		"rustup component add clippy",
		"rustup component add rustfmt",
		"apt-get update --fix-missing",
		"apt-get -y install git curl wget",
		// claude code
		"curl -fsSL https://claude.ai/install.sh | bash",
		// install Github Cli
		"apt install -y gh",
		// zsh
		"apt install -y zsh",
	]
	workspace: "shell_cell"
	shell:     "/bin/zsh"
	hang:      "while true; do sleep 3600; done"
	config: {
		mounts: [
			"./.claude:/shell_cell/.claude",
			"./.github:/shell_cell/.github",
			"./docs:/shell_cell/docs",
			"./e2e-tests:/shell_cell/e2e-tests",
			"./src:/shell_cell/src",
			"./build.rs:/shell_cell/build.rs",
			"./README.md:/shell_cell/README.md",
			"./CLAUDE.md:/shell_cell/CLAUDE.md",
			"./Cargo.toml:/shell_cell/Cargo.toml",
			"./Cargo.lock:/shell_cell/Cargo.lock",
		]
	}
}
