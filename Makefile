PROJECT           = concept
CONTAINER         = $(PROJECT)-dev

LOCAL_UID         = $(shell id -u)
LOCAL_GID         = $(shell id -g)
LOCAL_USER        = $(shell whoami)
BUILD_ARGS        = --build-arg LOCAL_USER="$(LOCAL_USER)" --build-arg LOCAL_UID="$(LOCAL_UID)" --build-arg LOCAL_GID="$(LOCAL_GID)"

USER_SHARED_DIR   = $(shell pwd)/target
DOCKER_SHARED_DIR = /var/opt/$(PROJECT)/target

OWNER             = $(LOCAL_USER):$(LOCAL_USER)
SHARED_VOLUME     = $(USER_SHARED_DIR):$(DOCKER_SHARED_DIR)


all: docker-build
all: docker-run


.PHONY: docker-container
docker-container:
	docker build -f Dockerfile .  -t "$(CONTAINER)" $(BUILD_ARGS)

.PHONY: docker-build
docker-build:
	docker run -u "$(OWNER)" "$(CONTAINER)" /bin/bash -c ". /home/$(USER)/.profile && make build"

.PHONY: docker-cross-compile
docker-cross-compile:
	docker run -v "$(SHARED_VOLUME)" -u "$(OWNER)" "$(CONTAINER)" /bin/bash -c ". /home/$(USER)/.profile && make build"

.PHONY: docker-run
docker-run: target
	docker run -itv "$(SHARED_VOLUME)" -u "$(OWNER)" "$(CONTAINER)"

.PHONY: clean
clean:
	cargo clean

.PHONY: build
build:
	cargo build --target x86_64-pc-windows-gnu --release


.PHONY: run
run:
	target/x86_64-pc-windows-gnu/release/concept.exe

.PHONY: target
target:
	mkdir -p target
