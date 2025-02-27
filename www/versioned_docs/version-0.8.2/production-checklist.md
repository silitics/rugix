---
sidebar_position: 44
---

# Production Checklist

Here is a rough checklist of things to consider before deploying anything to production.

:::warning

**This list is provided on a best-effort basis and makes no claim of being comprehensive.** We are not responsible for any issues you may run into. **As Rugix itself, this list comes without any warranties, whether express or implied.**

:::

## System Size

Make sure that you have [configured a system size](./ctrl/bootstrapping.mdx#default-layout) that will be sufficient as your system evolves.
While possible, **growing the system partitions in production is a potentially risky and complex endeavour and not supported by Rugix**. The default size of the system partitions is 4 GiB. To find a good size, consider how your application evolves.

## Testing

Always test any images and update bundles before deploying anything in production. Testing should cover the update workflow itself, **otherwise you risk loosing the ability to update systems in the field**. You can use Rugix's integration testing framework.

## Pre-Commit Hooks

Make sure to have proper [pre-commit hooks](./ctrl/hooks.md) installed to prevent any commits as long as there are issues. In particular, make sure that the system is still able to reach any remote servers necessary for remote control or update installation. Without pre-commit hooks, **you risk rendering your device inoperable, loosing the ability to reach it remotely, and loosing the ability to deploy updates**.