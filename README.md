## THIS IS NOT READY YET AND WON'T BE FOR A WHILE

Please use [vzvol](https://github.com/RainbowHackerHorse/vzvol) until it is.

This is an active effort on my part to learn Rust, and eventually implement features I never finished in `vzvol`

# vzvol-rs
vzvol rewrite in Rust

![VZvolLogo](https://github.com/RainbowHackerHorse/vzvol/raw/master/HorseDrive_vector_border.png)

vzvol is a tool to assist in the creation of ZFS zvols as storage for various virtualization providers.

## What
Creates a ZFS zvol, and configures permissions, and creates and registers a VirtualBox VMDK shim for the zvol if you ask nicely. 

## Why
This allows you to use the zvol to back a disk for VirtualBox, bhyve, or other virtualization providers.
`vzvol` also allows you to format your zvol with many filesystems, including:
- zfs
- ufs2
- fat32
- ext2,3,4
- xfs

## Contributing
Fork and open a PR with your changes.
If you've contributed, please ensure you edit CONTRIBUTORS and add your GitHub username
to the bottom if it isn't already listed!

## Package Status
#### FreeBSD Ports
Unsupported

#### Debian Linux / Ubuntu Linux
Unsupported

#### Arch Linux
Unsupported

#### Slackware Linux
Idk, I'll probably shove this into `pkgsrc` eventually, use that.

#### Windows
No.



