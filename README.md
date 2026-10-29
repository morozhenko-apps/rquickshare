# rQuickShare

Moving a photo or a file between Linux and Android should be a boring task. It isn't always, which is why I keep working on rQuickShare.

It's a free, open-source Linux desktop app for sharing files, photos and text with compatible nearby Android devices using Quick Share. I maintain this fork to make everyday transfers easier, without adding a cloud account or making you upload files to a server first.

[Product page](https://morozhenko.com/en/rquickshare/) · [Linux installation](https://morozhenko.com/rquickshare/install/linux) · [GitHub issues](https://github.com/morozhenko-apps/rquickshare/issues)

## What it does

You can find compatible Android devices nearby and send them a file, image or text from your Linux desktop. You can also receive transfers, with a clear choice to accept or decline incoming requests. A small tray card shows what's happening, and a separate view helps you follow ongoing and completed transfers.

The idea is to keep sharing simple without hiding important decisions. A rejected transfer should stay rejected, and an interrupted transfer should not look like a success.

## Getting started

The native Linux package targets **Ubuntu 26.04 on x86-64 (amd64)** and comes as a `.deb` file. Use the [Linux installation link](https://morozhenko.com/rquickshare/install/linux), install the package with your usual package manager, and open rQuickShare.

Make sure nearby discovery and the required connection permissions are enabled on both devices. Then select a compatible Android Quick Share device and send your file or text. When receiving something, check which device is sending it before accepting.

This native edition is for Linux. It does not include Windows or macOS installers, and you don't need to install a separate rQuickShare app on Android.

## Privacy and compatibility

The file content travels over supported nearby connections, not through a cloud-storage service operated by me. You don't need an account with the developer to send or receive anything. The devices still exchange information needed for discovery and connection, so this is not a promise of complete anonymity.

Quick Share compatibility depends on the devices, their permissions and the available connections. I can't promise that every Android phone and every network setup will behave the same way. If something doesn't work, I'd rather know exactly what failed than pretend the transfer succeeded.

You can read the [Privacy Policy](https://morozhenko.com/rquickshare/privacy-policy) and [Terms of Use](https://morozhenko.com/rquickshare/terms-of-use) for more detail.

## Bugs and feedback

If you find a bug or an awkward part of the interface, [open an issue](https://github.com/morozhenko-apps/rquickshare/issues). Tell me what you tried, what you expected and what actually happened. The Linux version, Android model and steps to reproduce are more useful than a screenshot of a generic error.

Please remove personal file names, local paths, device identifiers, IP addresses and private data from logs before posting them publicly. Suggestions and bug reports are welcome, but I'm not accepting unsolicited pull requests to this fork.

## Source and license

rQuickShare is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). Credit for the original project and other contributors stays with them.

The code is available under the [GNU General Public License v3.0](LICENSE).
