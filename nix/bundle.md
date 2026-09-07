# Build Update Bundles

`rugix.lib.mkBundle` packages files into a Rugix update bundle. Each file is a
payload; the bundle manifest specifies where and how Rugix delivers it. The caller
supplies the files and manifest, keeping image formats and device layouts under
the caller's control.

## Create a Bundle

In a flake with a `rugix` input, use the build machine's package set as `pkgs`.
Here, `systemImage` is the payload file: a derivation output or a file inside one.
The destination slot `system` must be defined in the device's Rugix configuration.

```nix
let
  mkBundle = rugix.lib.mkBundle { inherit pkgs; };
in
mkBundle {
  name = "update";
  version = "1";
  manifest = {
    update-type = "full";
    payloads = [
      {
        filename = "system.img";
        delivery = {
          type = "slot";
          slot = "system";
        };
        block-encoding = {
          chunker = "casync-64";
          deduplicate = true;
        };
      }
    ];
  };
  payloads."system.img" = systemImage;
}
```

The result contains `update.rugixb` and `update.rugixb-hash`.

## Helper Arguments

First call `rugix.lib.mkBundle { pkgs = buildPkgs; }`. Use the build machine's
package set; payloads may target another architecture. An optional `rugixBundler`
argument overrides the Bundler package at this step.

| Argument   | Purpose                                                                                               |
| ---------- | ----------------------------------------------------------------------------------------------------- |
| `name`     | Derivation name and output filename stem.                                                             |
| `version`  | Nix derivation version.                                                                               |
| `manifest` | Attribute set in the existing [bundle manifest format](../schemas/rugix-bundle-manifest.schema.json). |
| `payloads` | Source files keyed by their manifest filenames.                                                       |

Each manifest payload must have exactly one source. Filenames may contain
subdirectories, but must be relative paths without empty, `.` or `..` segments.
The helper preserves the manifest's payload order and delivery settings. Bundler
validates the manifest when building. `manifestFile` and `payloads` are exposed as
passthru attributes for inspection.

## Check Bundle Packaging

```console
nix build .#checks.x86_64-linux.bundle
```

The check bundles and unpacks multiple files, then compares their contents,
manifest, and verification hash.
