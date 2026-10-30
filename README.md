# rQuickShare

**Share files, photos and text with nearby devices.** rQuickShare is a free, open-source desktop app built around the idea that moving something between your computer and your phone should be simple.

[Product page](https://morozhenko.com/en/rquickshare/) · [Download](https://morozhenko.com/rquickshare/install/linux) · [Report an issue](https://github.com/morozhenko-apps/rquickshare/issues)

## Why this project exists

I originally wanted a native way to share files between Linux and Android. I liked the idea behind [Martichou's rQuickShare](https://github.com/Martichou/rquickshare), but I wasn't happy with the interface or some parts of the implementation. Development also seemed to have slowed down, so I decided to take the existing project as a starting point and work on it myself.

It made more sense to improve an open-source implementation than to start from zero just to solve the same problem. The goal is a straightforward desktop experience, with clear transfer status, sensible controls and fewer surprises.

Linux and Android were the first priority, not the final limit. I'd like to bring rQuickShare to Windows and macOS as well. Those are plans, not supported downloads yet.

## Features

- Discover compatible nearby Android devices and choose where to send files, photos or text.
- Review incoming transfers and explicitly accept or decline them.
- Track incoming and outgoing transfers from the desktop, including a compact tray interface.
- Open received files or their containing folder.
- Use supported nearby connections without uploading your files to a developer-operated cloud-storage account.

Compatibility depends on the other device, its Quick Share implementation, permissions and available connections. Not every device or network behaves the same way.

## Install

The current native build is available as a **DEB package for Ubuntu 26.04 on x86-64 (amd64)**.

**[Download rQuickShare for Linux](https://morozhenko.com/rquickshare/install/linux)**

After installation, open rQuickShare and make sure the required nearby discovery and connectivity permissions are enabled on both devices. Choose a compatible Android device and send a file or some text. Check the sender before accepting an incoming request.

Windows and macOS versions are planned, but there are no installers for them in this release. Android is currently the other device in the transfer, not a separate rQuickShare app you need to install.

## Privacy and security

Transfers use supported nearby and local-network connections. File contents are not uploaded to a cloud-storage service operated by this project. Devices still exchange information needed to discover and connect to each other, so local sharing does not mean complete anonymity.

For more detail, read the [Privacy Policy](https://morozhenko.com/rquickshare/privacy-policy) and [Terms of Use](https://morozhenko.com/rquickshare/terms-of-use).

## Feedback and contributing

If a device is not discovered, a transfer fails or part of the interface is confusing, [open an issue](https://github.com/morozhenko-apps/rquickshare/issues). Include what you tried, what you expected, what happened instead and the device/OS versions. Please remove personal file paths, device identifiers, IP addresses and private data before sharing logs or screenshots.

Bug reports and suggestions are welcome. I am not accepting unsolicited pull requests to this fork.

## Source and license

rQuickShare is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). Credit to the original author and other contributors remains in place.

The source is available under the [GNU General Public License v3.0](LICENSE).
