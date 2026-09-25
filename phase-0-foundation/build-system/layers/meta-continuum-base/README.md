# meta-continuum-base

Base system layer: init system (per ADR 0002), filesystem setup (ext4
root per ADR 0003), and integrates the boot-chain components from
`../../../boot/` (Secure Boot signing, dm-verity image generation, A/B
update client) into the built image. Recipe implementation deferred.
