PROJECT = concept

.PHONY: docker-container
docker-container:
	docker build --no-cache -f "Dockerfile" -t "$(PROJECT)-dev" .

.PHONY: docker-build
docker-build:
	docker run -v "$(shell pwd):/var/$(PROJECT)-dev/$(PROJECT)" -u "root:root" "$(PROJECT)-dev" /bin/bash -c "cargo clean && cargo build --target x86_64-pc-windows-gnu"

.PHONY: docker-run
docker-run:
	docker run -itv "$(shell pwd):/var/$(PROJECT)-dev/$(PROJECT)" -u "root:root" "$(PROJECT)-dev"
