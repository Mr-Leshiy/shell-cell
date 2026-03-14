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
			"./:/shell_cell/",
		]
	}
}
