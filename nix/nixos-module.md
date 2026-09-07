# Configure Rugix on NixOS

`rugix.nixosModules.rugix` provides the Rugix Ctrl service integration, privileged
operation daemon, and app recovery services. Device configuration remains under
`services.rugix.settings`; the module does not define disk layouts or boot groups.

## Enable Rugix

In a flake with `nixpkgs` and `rugix` inputs, add the following output alongside
your device's `configuration.nix`, which supplies the platform, storage, and Rugix
settings:

```nix
nixosConfigurations.device = nixpkgs.lib.nixosSystem {
  modules = [
    rugix.nixosModules.rugix
    ({ pkgs, ... }: {
      nixpkgs.overlays = [ rugix.overlays.default ];
      services.rugix = {
        enable = true;
        package = pkgs.rugix-ctrl;
        apps.dockerCompose.enable = true;
        daemon = {
          enable = true;
          features.appLifecycle = true;
        };
      };
    })
    ./configuration.nix
  ];
};
```

Supply the device's Rugix settings and storage configuration in
`configuration.nix`. Set `rugix.inputs.nixpkgs.follows = "nixpkgs"` in the flake
inputs to share the package set. The package can also be supplied directly through
`services.rugix.package` without installing the overlay.

## Service Options

All options are under `services.rugix`.

| Option                      | Purpose                                                                                              |
| --------------------------- | ---------------------------------------------------------------------------------------------------- |
| `enable`                    | Install Rugix Ctrl and generate `/etc/rugix/system.toml`.                                            |
| `package`                   | Rugix Ctrl package; defaults to `pkgs.rugix-ctrl`.                                                   |
| `settings`                  | Rugix system configuration as a TOML-compatible attribute set.                                       |
| `apps.enable`               | Restore app units during boot and recover interrupted transitions.                                   |
| `apps.dockerCompose.enable` | Enable Rugix, app recovery, Docker, and Compose runtime component metadata.                          |
| `daemon.enable`             | Start the privileged operation daemon with the `rugix-daemon` group.                                 |
| `daemon.features`           | Opt into `factoryReset`, `systemCommit`, `systemReboot`, and `appLifecycle` operations individually. |

Enable flags default to false. App recovery and daemon options require Rugix to
be enabled. The module leaves the decision to commit system updates to the device
configuration or an authorized daemon client.
