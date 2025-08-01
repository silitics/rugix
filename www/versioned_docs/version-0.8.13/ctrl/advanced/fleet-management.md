---
sidebar_position: 1
---

# Fleet Management

Rugix Ctrl provides a reliable foundation for OTA updates and state management.
To manage a fleet of devices remotely and deliver updates to devices, Rugix Ctrl integrates well with a variety of existing off-the-shelf fleet management solutions.

:::info

As Rugix Ctrl is independent from a fleet management solution, it avoids vendor lock-in.
When using Rugix Ctrl, **you can switch between different fleet management solutions at anytime** and continue updating your existing fleet.
It also allows you to choose a fleet management provider based on the needs and requirements of your application.

:::

Currently, there are ready-made integrations with [Nexigon](https://nexiogn.dev/?utm_source=rugix.org&utm_campaign=rugix-fleet-management), [thin-edge.io](https://thin-edge.io/), [Memfault](https://memfault.com/), and [Mender](https://mender.io/).
For other fleet management solutions, you can develop your own integration or [contract Silitics](/commercial-support), the company behind Rugix, to develop an integration for you.


## Nexigon

Nexigon is a ready-made, modular infrastructure layer for connected devices, designed to seamlessly integrate with your applications and workflows.
Nexigon is developed by the creators of Rugix as a complementary commercial offering to support the ongoing development and maintenance of Rugix as an open-source project.
Nexigon provides a secure, reliable, and scalable foundation you can build upon.
Follow the [Nexigon quickstart guide for Rugix](https://docs.nexigon.dev/agent/build-system-integration/rugix/getting-started?utm_source=rugix.org&utm_campaign=rugix-fleet-management) to onboard your device and connect it to Nexigon within minutes.


## thin-edge.io

[thin-edge.io](https://thin-edge.io/) is an open-source, cloud-agnostic IoT framework designed for resource constraint devices.
It provides an abstraction layer to interface with different providers of IoT management solutions such as [Cumulocity IoT](https://www.cumulocity.com/guides/concepts/introduction/), [Azure IoT](https://azure.microsoft.com/en-us/solutions/iot), and [AWS IoT](https://aws.amazon.com/iot/).
thin-edge.io officially supports Rugix Bakery as well as Rugix Ctrl.
That is, integrating thin-edge.io into your system is straightforward with ready-made recipes for Rugix Bakery.
Those recipes will also include an integration layer for Rugix Ctrl so that you can deploy updates without any further configuration.
To learn more, check out the [thin-edge.io Rugix reference repository](https://github.com/thin-edge/tedge-rugpi-image).


## Memfault

[Memfault](https://memfault.com/) is a fleet management solution with a focus on observability.
We provide a [template repository showcasing how to integrate Rugix with Memfault](https://github.com/silitics/rugix-template-memfault).
Check out the [corresponding Interrupt article for further details](https://interrupt.memfault.com/blog/robust-ota-updates-the-easy-way).


## Mender

Open-source Mender support for Rugix Ctrl and Rugix Bakery is provided by [Silitics](https://silitics.com), check out the [Mender Rugix reference repository](https://github.com/silitics/rugpi-template-mender).
The Mender integration consists in recipes for Rugix Bakery that will install Mender's client as well as a Mender update module to install updates via Mender with Rugix Ctrl.
Note that Mender also offers their own update installation mechanism, which is part of the Mender client.
**When you use Mender with Rugix Ctrl, you will not be using this mechanism but Rugix Ctrl instead.
You can still deploy updates through Mender's fleet management solution as you normally would.**
If you want to know the differences between Mender's own solution and Rugix Ctrl, check out the [Comparison to Other Solutions](../index.md#comparison-to-other-solutions).

:::note
Unfortunately, Mender's fleet management solution is at the moment incompatible with dynamic delta updates.
This is also unlikely to change in the future as delta updates are a key feature of Mender's enterprise offering.
:::

You can also use Rugix Bakery to build images for use with Mender's own OTA solution using [Mender's conversion approach for Debian](https://docs.mender.io/operating-system-updates-debian-family/convert-a-mender-debian-image).
For Debian-based systems, [Mender's documentation recommends](https://web.archive.org/web/20240815210840/https://docs.mender.io/operating-system-updates-debian-family/convert-a-mender-debian-image#recommended-workflow) that you boot an actual system with an image, make changes, and then extract the image from the running system. We strongly recommend not to use this so called _golden image_ workflow as it is a heavily manual process, making it impossible to reproduce and tedious to apply changes. You always have to manually update and integrate your application, which will lead to much less frequent updates with all the (security) implications that brings.
In contrast, with Rugix Bakery, you get a modern, end-to-end workflow for building Debian images that you can also run in CI.

If you are building on Raspberry Pi, note that Rugix Ctrl supports [Raspberry Pi's `tryboot` mechanism](https://www.raspberrypi.com/documentation/computers/config_txt.html#example-update-flow-for-ab-booting), which is the official way to do A/B updates on a Raspberry Pi.
Mender does not support the `tryboot` mechanism but relies on its U-Boot integration instead.
This means that Mender's support for newer Raspberry Pi models will typically be blocked by U-Boot support and therefore lack behind Rugix's.
Furthermore, with the `tryboot` mechanism you can also update the boot partition, including changes to device tree overlays in `config.txt`, which you cannot do when using U-Boot.
Also, Mender's conversion approach so far does not work for 64-bit Raspberry Pi OS.[^mender-64-bit]
Hence, for Raspberry Pi, we definitely recommend using Rugix Ctrl instead of Mender.

[^mender-64-bit]: At the time of writing. For updates, see [this issue in Mender's issue tracker](https://northerntech.atlassian.net/browse/MEN-5634).