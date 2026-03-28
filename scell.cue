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
		// install Github CLI via official Debian package repository
		// https://github.com/cli/cli/blob/trunk/docs/install_linux.md
		"(type -p wget >/dev/null || (apt-get update && apt-get install wget -y)) && mkdir -p -m 755 /etc/apt/keyrings && out=$(mktemp) && wget -nv -O$out https://cli.github.com/packages/githubcli-archive-keyring.gpg && cat $out | tee /etc/apt/keyrings/githubcli-archive-keyring.gpg > /dev/null && chmod go+r /etc/apt/keyrings/githubcli-archive-keyring.gpg && mkdir -p -m 755 /etc/apt/sources.list.d && echo \"deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main\" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null && apt-get update && apt-get install gh -y",
		// nushell
		"wget -qO- https://apt.fury.io/nushell/gpg.key | gpg --dearmor -o /etc/apt/keyrings/fury-nushell.gpg",
		"echo \"deb [signed-by=/etc/apt/keyrings/fury-nushell.gpg] https://apt.fury.io/nushell/ /\" | tee /etc/apt/sources.list.d/fury-nushell.list",
		"apt update",
		"apt install nushell",
	]
	workspace: "shell_cell"
	shell:     "/bin/nu"
	hang:      "while true; do sleep 3600; done"
	config: {
		mounts: [
			"./:/shell_cell/",
		]
	}
	// services: {
	// 	layer_1: {
	// 		from_image: "debian:trixie"
	// 		hang:       "while true; do sleep 3600; done"
	// 		services: {
	// 			layer_2: {
	// 				from_image: "debian:trixie"
	// 				hang:       "while true; do sleep 3600; done"
	// 				services: {
	// 					layer_3: {
	// 						from_image: "debian:trixie"
	// 						hang:       "while true; do sleep 3600; done"
	// 					}
	// 				}
	// 			}
	// 		}
	// 	}
	// }
}
