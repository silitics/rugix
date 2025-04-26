---
sidebar_position: 5
---

# Yocto Integration

We provide open-source [Yocto](https://www.yoctoproject.org/) layers for integrating Rugix Ctrl into a custom, Yocto-based Linux distribution tailored to your embedded device.
The yocto layers are [available here](https://github.com/silitics/meta-rugix/tree/main).

:::warning
These Yocto layers are **work-in-progress and not intended for production use** at this time.
We appreciate any feedback you may have regarding this Yocto integration – feel free to [open an issue](https://github.com/silitics/meta-rugix/issues/new/choose).
:::

## Installing Rugix Ctrl

The `meta-rugix-core` layer provides the `rugix-ctrl` and `rugix-ctrl-bin` recipes for installing Rugix Ctrl.
The `rugix-ctrl-bin` recipes use the pre-build Rugix Ctrl binaries available from Rugix's GitHub releases page.

:::note
At the moment, the `rugix-ctrl` does not work yet due to a compilation issue.
:::

## Bootstrapping Configuration

The `meta-rugix-core` layer provides the `rugix-bootstrapping-conf` recipe for installing a custom [`bootstrapping.toml` configuration file](../bootstrapping.mdx).
You can extend this recipe to replace the default bootstrapping configuration.

## Building Update Bundles

The `meta-rugix-core` layer provides the `rugix-bundle` class for building [Rugix update bundles](./update-bundles.mdx).
The class assumes that you build partitioned images using [WIC](https://docs.yoctoproject.org/5.0.8/dev-manual/wic.html) and allows you to include individual partitions as update payloads.

Here's an example for a typical A/B setup with redundant boot and system partitions:

```bitbake title="update-bundle-minimal.bb"
inherit rugix-bundle

RUGIX_BUNDLE_PAYLOADS = "boot system"

RUGIX_PAYLOAD_boot[image] = "core-image-minimal"
RUGIX_PAYLOAD_boot[partition] = "2"

RUGIX_PAYLOAD_system[image] = "core-image-minimal"
RUGIX_PAYLOAD_system[partition] = "4"
```

The `RUGIX_BUNDLE_PAYLOADS` variable contains a space-separated list of update payloads.
Each payload is then configured through a corresponding `RUGIX_PAYLOAD_` variable with the following flags:

- `image` (required): Image to take the partition from.
- `partition` (required): Image partition to include as the payload.
- `slot` (optional): Update slot for the payload (defaults to payload name).

:::note
In the future, we will extend the class to support additional payload types.
:::

## Raspberry Pi

The `meta-rugix-rpi-tryboot` layer provides support for building Raspberry Pi images with [`tryboot`](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#fail-safe-os-updates-tryboot) support.
For an example, take a look at the [`raspberrypi.yaml`](https://github.com/silitics/meta-rugix/blob/main/examples/raspberrypi.yaml) example [kas](https://github.com/siemens/kas) configuration.