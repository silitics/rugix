# Migrating from Mender

:::warning

**The functionality described here is still in development and not fully mature and battle-tested yet.**

:::

Mender supports two bootloaders, U-Boot and GRUB.
Rugix Ctrl implements two Mender-compatible boot flows, `mender-uboot` and `mender-grub`, which are compatible with Mender's U-Boot and GRUB integration, respectively.

Migrating from Mender to Rugix Ctrl requires a [system configuration file](../advanced/system-configuration.mdx) that specifies the correct Mender-compatible boot flow, boot groups, and slots.
Here is an example for GRUB with the default Mender boot partitions:

```toml title="/etc/rugix/system.toml"
#:schema https://raw.githubusercontent.com/silitics/rugix/refs/heads/main/schemas/rugix-ctrl-system.schema.json

[boot-flow]
type = "mender-grub"
# boot-dir = "/boot"
# boot-part-a = 2
# boot-part-b = 3

[boot-groups.a]
slots = { system = "system-a" }

[boot-groups.b]
slots = { system = "system-b" }

[slots.system-a]
type = "block"
partition = 2

[slots.system-b]
type = "block"
partition = 3
```

This is all that is required to enable Rugix Ctrl to take over update installation from Mender.
Any custom integrations such as state scripts need to be adapted to work with Rugix Ctrl, e.g., by using [hooks](../hooks.md).
Furthermore, if you want to use Rugix's state management functionality, there needs to be a migration of the data partition to the structure required by Rugix.

:::info

Mender tracks the state of updates and installed software within its own database.
While it is possible to use Rugix Ctrl and Mender on the same device at the same time, special care must be taken to ensure that the two do not cause the state tracked by the other to diverge from the actual state of the system.
We therefore recommend a full and clean migration.

:::