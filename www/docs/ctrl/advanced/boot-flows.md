---
sidebar_position: 3
---

# Boot Flows

A *boot flow* provides the base mechanism to switch between different boot groups, e.g., to realize an A/B update scheme.
To this end, a boot flow needs to be integrated with a bootloader.
Rugix Ctrl provides the following *default boot flows*:

- `uboot`: Boot flow for U-Boot (A/B updates without a dead men's switch).
- `grub`: Boot flow for GRUB (A/B updates without a dead men's switch).

For compatibility with other OTA update solutions, Rugix Ctrl further provides the following boot flows:

- `rauc-uboot`: RAUC-compatible boot flow for U-Boot (priority-based with a dead men's switch).
- `rauc-grub`: RAUC-compatible boot flow for GRUB (priority-based with a dead men's switch).
- `mender-uboot`: Mender-compatible boot flow for U-Boot (A/B updates without a dead men's switch).
- `mender-grub`: Mender-compatible boot flow for GRUB (A/B updates without a dead men's switch).

:::note

Those boot flows allow [safe, in-the-field migrations to Rugix Ctrl from RAUC and Mender](../migrating/index.md), respectively.
This allows users to take advantage of Rugix Ctrl's advanced features, such as highly-efficient static delta updates and streaming from arbitrary sources, in brownfield projects without having to re-provision their devices.
Furthermore, it allows existing bootloader integrations for RAUC and Mender to be re-used with Rugix Ctrl, even in new projects.

:::

In addition to the above generic boot flows, Rugix Ctrl also provides the following Raspberry Pi-specific boot flows:

- `rpi-uboot`: Boot flow for Raspberry Pi using U-Boot.
- `rpi-tryboot`: Boot flow for Raspberry Pi 4 and newer models using the `tryboot` mechanism of Raspberry Pi's firmware.

If none of these boot flows fit your needs, you can also implement your own `custom` boot flow.

Boot flows are configured through the [`boot-flow` section of the system configuration](../advanced/system-configuration.mdx).
If no boot flow is configured, Rugix Ctrl will try to detect it dynamically at runtime by inspecting the config partition (usually the first partition of the root disk):

1. If a file `autoboot.txt` exists, then the boot flow is `rpi-tryboot`.
2. If a file `bootpart.default.env` exists, then the boot flow is `rpi-uboot`.
3. If a file `rugpi/grub.cfg` exists, then the boot flow is `grub`.

In all other cases, automatic runtime detection will fail.


## Boot Flow Interface

Each boot flow must implement at least three operations:

- `set_try_next(group)`: Set the boot group to try first on the next boot (falling back to the current default).
- `get_default()`: Retrieve the current default boot group.
- `commit(group)`: Commit the currently active boot group.

Note that Rugix Ctrl will determine the active boot group itself.
The currently active boot group will be supplied to `commit` and the boot flow's commit operation should fail if it disagrees about what the active boot group is.

In addition, a boot flow may support the following operations:

- `pre_install(group)`: Runs before installing an update to the given group.
- `post_install(group)`: Runs after installing an update to the given group.
- `mark_good(group)`: Mark the given boot group as *good*.
- `mark_bad(group)`: Mark the given boot group as *bad*.

The `mark_good` and `mark_bad` operations are useful for implementing a dead men's switch, where the bootloader triggers a failover to another boot group after a certain number of failed boot attempts.
Rugix Ctrl will **not** automatically mark boot groups as *good* or *bad*, instead an external mechanism is required to monitor the system and trigger the marking through `rugix-ctrl boot`.

:::info

While a dead men's switch sounds like a great idea, it can also have diametral effects.
If boot attempts fail or are considered failed due to external factors (user interactions, fluctuating power, connectivity issues), this may trigger inadvertent failovers until all boot groups are marked bad.
A robust implementation should therefore include an always-operational recovery system that restores the system.
As this is complex and difficult to implement correctly, Rugix Ctrl's default boot flows do not include a dead men's switch.
Instead, a system committed to is assumed to stay good.
As the state management functionality of Rugix Ctrl prevents accidental state from corrupting the system, this is a reasonable assumption in almost all typical cases.

:::

A typical update installation to a boot group will trigger the following operations:

1. `pre_install(group)`
2. Installation of the update.
3. `post_install(group)`
4. `set_try_next(group)`
5. Reboot.

Rebooting with `--boot-group` or `--spare` will trigger the following operations:

1. `set_try_next(group)`
2. Reboot.

Committing an update will trigger the following operations:

1. `get_default()`
2. `commit(group)` only if the default and active group differ.

Note that `set_try_next` may or may not change the default boot group.
In any case, it must guarantee that there is a (transitive) fallback to the current default to make sure that a broken update will not leave the system in an inoperable state.


## Generic Boot Flows

Generic boot flows are independent of any specific device type.

### GRUB

`type = "grub"`

The `grub` boot flow implements an A/B switching mechanism and assumes the following GPT partition layout:

```
1: config    FAT32
2: boot-a    EXT4
3: boot-b    EXT4
4: system-a
5: system-b
```

That is, it requires separate boot and system partitions.
The boot partitions must contain a *second stage boot script* that can be safely updated and used to customize the early boot process.
Furthermore, they typically contain the kernel and initial ramdisk.

To implement the A/B switching Rugix Ctrl uses two boot variables:

- `rugpi_bootpart`: The default partition to boot from (set to `2` or `3`).
- `rugpi_boot_spare`: Indicates whether the spare boot partition should be booted.

Note that those variables still use `rugpi` as the prefix for compatibility reasons, as this is the historic name of Rugix.

Rugix Ctrl sets `rugpi_boot_spare` to `1` to indicate to the bootloader that the spare partition should be booted.
In this case, the bootloader will inspect the `rugpi_bootpart` variable and boot from the respective other partition.
Prior to booting, the bootloader will set the `rugpi_boot_spare` variable back to `0`, falling back to the default partition unless a commit happens.

To communicate the variables to GRUB, Rugix Ctrl uses the following environment files placed on the config partition:

- `rugpi/boot_spare.grubenv`: Contains the value of `rugpi_boot_spare`.
- `rugpi/primary.grubenv`: Contains the value of `rugpi_bootpart` (primary copy).
- `rugpi/secondary.grubenv`: Contains the value of `rugpi_bootpart` (secondary copy).

Rugix Ctrl writes the SHA1 sum of the environment files containing the value of `rugpi_bootpart` to a respective file with the `.sha1` suffix.
This allows GRUB to verify the integrity of the environment files prior to loading them.
Should the primary copy be corrupted, e.g., due to a power failure while writing the file, GRUB will fall back to the secondary copy.

As part of the `post_install` operation, Rugix Ctrl also writes an environment file `boot.grubenv` to the respective boot partition on which the update is installed.
This environment file contains a variable `rugpi_bootargs` specifying boot arguments that may be used.
Those boot arguments have the form `ro init=/usr/bin/rugix-ctrl root=PARTUUID=<...>` where `<...>` is replaced by the partition UUID of the respective system partition.
Those boot arguments may be loaded and used by the second stage boot script, however, they can also be ignored.
In the future we may add additional variables, e.g., for just the partition UUID.

For further details, we refer to the reference [boot scripts](https://github.com/silitics/rugpi/tree/main/boot/grub/cfg) used by Rugix Bakery.

### U-Boot

`type = "uboot"`

The `uboot` boot flow makes no assumptions about the system partition layout.
It uses `fw_setenv` and `fw_printenv` to set and read U-Boot environment variables based on the configuration in `/etc/fw_env.config`.

To implement the A/B switching Rugix Ctrl uses two boot variables:

- `rugix_bootpart`: The default partition to boot from (set to `2` or `3`).
- `rugix_boot_spare`: Indicates whether the spare boot partition should be booted.

Those variables are set analogously to the respective variables of the `grub` boot flow (see above).
In contrast to the `grub` boot flow, there is no additional logic, for instance, to store the partition UUID in the boot arguments.

**Rugix Ctrl does not require any patches to U-Boot.
The required logic can be entirely implemented in U-Boot scripts.**

### Custom

`type = "custom"`

This boot flow allows you to write your own custom logic for controlling the boot process. An example may look as follows:

```toml title="/etc/rugix/system.toml"
[boot-flow]
type = "custom"
controller = "<path to your script>"
```

Your custom boot flow will be called with the name of the operation as the first argument.
If an operation takes a boot group as an argument, then the second argument will be the name of the boot group.
**We may add further, optional arguments in the future, hence, your boot flow should ignore any additional arguments.
We may also add further, optional operations, hence, your script should not do anything (except printing something on stderr), in case it receives an unknown operation as the first argument.**
Following these rules minimizes churn on your end.
The boot flow is expected to produce JSON output on stdout.

For now, all operations except `get_default` should simply return an empty JSON object on stdout and indicate success/failure, as usual, through the return code. The output of `get_default` is expected to have the following form:

```json
{ "group": "<name of the boot group>" }
```

:::info

Custom boot flows can be used to realize a variety of different update setups and integrate with bootloaders not natively supported by Rugix Ctrl.
If you need anything specific, [contact us for commercial support](mailto:hello@silitics.com?subject=Custom%20Boot%20Flow).

:::


### Systemd Boot

:::warning
**Support for Systemd Boot is not implemented yet.**
:::

```
1: EFI       FAT32
2: system-a
3: system-b
```

Support for Systemd Boot would use the [Boot Loader Interface](https://systemd.io/BOOT_LOADER_INTERFACE/) for A/B updates by writing to the following EFI variables:

- `LoaderEntryDefault-4a67b082-0a4c-41cf-b6c7-440b29bb8c4f` (default entry)
- `LoaderEntryOneShot-4a67b082-0a4c-41cf-b6c7-440b29bb8c4f` (oneshot entry)

In contrast to the other boot flows there would be no separate boot partitions.


## RAUC-compatible Boot Flows

The RAUC-compatible boot flows interact with the bootloader in [the same way as RAUC](https://rauc.readthedocs.io/en/latest/reference.html#bootloader-interaction).

See the [RAUC migration guide for details](../migrating/from-rauc.md).


## Mender-compatible Boot Flows

The Mender-compatible boot flows interact with the bootloader in the same way as Mender.

See the [Mender migration guide for details](../migrating/from-mender.md).


## Raspberry Pi Boot Flows

The following boot flows are specific to Raspberry Pi.

### Tryboot

`type = "rpi-tryboot"`

This boot flow is specific to Raspberry Pi 4 and newer models.

The `tryboot` boot flow works almost as described in [Raspberry Pi's documentation on the `tryboot` mechanism](https://www.raspberrypi.com/documentation/computers/config_txt.html#example-update-flow-for-ab-booting).
Instead of reading the device tree `tryboot` flag, it compares the booted partition with the default stored in `autoboot.txt`.

This boot flow typically comes with the following image and system layout:

```
MBR =============================== Image
     1: config    FAT32
     2: boot-a    FAT32
     3: boot-b    FAT32
     5: system-a
    =============================== System
     6: system-b
     7: data
```

This boot flow also allows updating the `config.txt` file as well as the device tree files.

This boot flow also supports a GPT partition table where the system partitions are the 4th and 5th partitions, respectively, and the data partition is the 6th partition. Note that in case of an MBR partition table, the 4th partition is the extended partition.

### Raspberry Pi: U-Boot

`type = "rpi-uboot"`

This boot flow assumes the following image and system layout:

```
MBR =============================== Image
     1: config    FAT32 
     2: boot-a    FAT32
     3: boot-b    FAT32
     5: system-a
    =============================== System
     6: system-b
     7: data
```

In contrast to the generic `uboot` boot flow, the `rpi-uboot` boot flow uses U-Boot environment files stored on the config partition.
This is done, to harmonize the boot flow with the `tryboot` boot flow.
There are two environment files, `bootpart.default.env` and `boot_spare.env`.
The file `bootpart.default.env` sets the `bootpart` variable either to `2` or to `3` indicating the default boot partition (`boot-a` or `boot-b`).
The file `boot_spare.env` sets the `boot_spare` variable either to `1` or to `0` indicating whether the spare or default partition should be booted, respectively.
In addition, there are the files `boot_spare.enabled.env` and `boot_spare.disabled.env` for overwriting the `boot_spare.env` file.

**Note that there is currently no redundancy for the `bootpart` environment file.**

A typical U-Boot boot script would proceed as follows:

1. Load `bootpart.default.env` and `boot_spare.env`.
2. If `boot_spare` is set to `1`, invert `bootpart`.
3. If `boot_spare` is set to `1`, overwrite `boot_spare.env` with `boot_spare.disabled.env`.
4. Proceed booting from the respective partition.

The reference implementation for Raspberry Pi uses two boot scripts, one first stage boot script on the config partition and a second stage boot script on the respective boot partition.
The first stage follows the steps outlined above and then loads the second stage boot script.
This has the advantage that the second stage script can be updated in a fail-safe way.

For further details, we refer to the reference [boot scripts](https://github.com/silitics/rugpi/tree/main/boot/u-boot/scripts) for Raspberry Pi.


## On Atomicity of Commits

Note that commits are the only critical operation because they modify the default boot group.
This is usually done by temporarily remounting the config partition such that it is writeable and then replacing some files.
As the filesystem is FAT32, the atomicity of this operation cannot be guaranteed.
Still, Rugpi Ctrl does its best by first creating a new file and then replacing the old one with the new one by renaming it, and, the Linux kernel does guarantee atomicity for renaming.
However, should the system crash during this process, the FAT32 filesystem may still be corrupted.
We think that this is an acceptable risk as the likelihood of it happening is extremely low and any alternatives, like swapping the MBR, may be problematic for other reasons.[^atomicity-suggestions]

[^atomicity-suggestions]: If you have any suggestions, please share them with us.