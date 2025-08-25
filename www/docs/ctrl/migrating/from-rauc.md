# Migrating from RAUC

:::warning

**The functionality described here is still in development and not fully mature and battle-tested yet.**

:::

RAUC supports U-Boot, GRUB, Barebox, and EFI.
**At the time of writing, Rugix Ctrl only supports migrating from U-Boot and GRUB.**
Furthermore, Rugix Ctrl only supports installing updates to block devices.
Raw flash devices requiring UBI are not supported yet.
If you need support for any of the above, please [open an issue](https://github.com/silitics/rugix/issues/new/choose) or [reach out for commercial support](mailto:hello@silitics.com?subject=Migrating%20to%20Rugix%20Ctrl).

Rugix Ctrl implements two RAUC-compatible boot flows, `rauc-uboot` and `rauc-grub`, which are compatible with RAUC's U-Boot and GRUB integration, respectively.
Migrating from RAUC to Rugix Ctrl requires a [system configuration file](../advanced/system-configuration.mdx) that specifies the correct RAUC-compatible boot flow, boot groups, and slots.
Here is an example for U-Boot with typical A/B boot partitions:

```toml title="/etc/rugix/system.toml"
#:schema https://raw.githubusercontent.com/silitics/rugix/refs/heads/main/schemas/rugix-ctrl-system.schema.json

[boot-flow]
type = "rauc-uboot"
# group-names = ["A", "B"]

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

Like RAUC, Rugix Ctrl also supports other setups than A/B.
To migrate to Rugix Ctrl, you need to translate your RAUC configuration to a Rugix Ctrl system configuration, declaring all slots and boot groups.
By default, Rugix Ctrl will convert the names of the boot groups to uppercase and use those as the names for the boot variables required by RAUC.
The option `group-names` can be used to override this behavior and specify different names for the boot groups.

:::info

While it is possible to use Rugix Ctrl and RAUC on the same device at the same time, special care must be taken to ensure that RAUC doesn't cause the state tracked by Rugix to become out of sync.
For instance, Rugix tracks what has been installed to the respective slots in order to enable (static) delta updates.
We therefore recommend a full and clean migration.

:::
