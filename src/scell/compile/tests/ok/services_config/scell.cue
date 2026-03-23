main: {
	from_image: "from"
	shell:      "shell"
	hang:       "hang"
	services: {
		db: {
			from_image: "postgres:16"
			shell:      "/bin/sh"
			hang:       "sleep infinity"
		}
		cache: {
			from_image: "redis:7"
			shell:      "/bin/sh"
			hang:       "sleep infinity"
		}
	}
}
