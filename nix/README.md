# Nix Integration

The [Rugix flake](../flake.nix) exposes command packages, runnable apps, a
nixpkgs overlay, a NixOS service module, and library helpers. This directory
contains supporting Nix expressions and documentation.

- [Run Rugix with Nix](../README.md#run-directly-from-git-with-nix): use the
  `rugix-ctrl`, `rugix-bundler`, and `rugix-util` packages and apps.
- [`overlays.default`](../flake.nix): add the three Rugix command packages to
  a nixpkgs package set.
- [Build Container Apps with Nix](compose-bundle.md): package Compose apps and
  Nix-built container images with `lib.mkComposeBundle`.
- [Configure Rugix on NixOS](nixos-module.md): configure Rugix Ctrl, its daemon,
  and app recovery with `nixosModules.rugix`.
- [Build Update Bundles](bundle.md): package explicit payloads and delivery
  settings with `lib.mkBundle`.
