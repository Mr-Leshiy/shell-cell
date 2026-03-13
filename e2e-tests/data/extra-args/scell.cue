_from_image_arg: string
_workspace_arg:  string
_shell_arg:      string
_hang_arg:       string
_copy_arg_1:     string
_copy_arg_2:     string
_build_arg:      string
_env_arg:        string
_mount_arg:      string
_port_arg:       string

main: {
	from_image: _from_image_arg
	workspace:  _workspace_arg
	shell:      _shell_arg
	hang:       _hang_arg
	copy: [
		_copy_arg_1,
		_copy_arg_2,
	]
	build: [
		_build_arg
	]
	env: [_env_arg]
	config: {
		mounts: [_mount_arg]
		ports: [_port_arg]
	}
}
