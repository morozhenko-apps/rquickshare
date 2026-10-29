# rQuickShare

**Share files, photos and text between Linux and compatible nearby Android Quick Share devices.** Free, open source and designed for straightforward desktop-to-phone transfers.

[Product page](https://morozhenko.com/en/rquickshare/) · [Download for Linux](https://morozhenko.com/rquickshare/install/linux) · [Report an issue](https://github.com/morozhenko-apps/rquickshare/issues)

## Features

- Discover compatible nearby Android devices and choose who receives your files, images or text.
- Review incoming transfer requests and explicitly accept or decline them.
- Follow outgoing and incoming transfer progress from a compact native Linux desktop interface.
- Open received files or their containing folder.
- Use nearby connectivity supported by participating devices without a developer-operated cloud file-storage account.

The recipient, device capabilities, network conditions and operating-system permissions determine whether a particular discovery or transfer path is available. rQuickShare does not claim compatibility with every phone or sharing implementation.

## Install

The native rQuickShare 1.0.0 release targets **Ubuntu 26.04, x86-64 (amd64)** and is packaged as a Debian `.deb` file.

**[Download the Ubuntu 26.04 DEB](https://morozhenko.com/rquickshare/install/linux)**

You can also find release assets and source history in the [GitHub repository](https://github.com/morozhenko-apps/rquickshare). The native release is provided for Linux; this edition does not include Windows or macOS installers. Android is the compatible peer device, not a separate rQuickShare Android application.

After installing, launch rQuickShare, enable the connectivity and discovery permissions needed on both devices, select the recipient and send your content. Accept any incoming transfer only from a device you recognize.

## Privacy and security

Transfers use supported nearby and local-network connections. File contents are not uploaded to a developer-operated cloud-storage service. Discovery and transfer can exchange device identifiers and connection metadata with other participating devices; use trusted recipients and review every incoming request.

For the full data-processing description, see the [Privacy Policy](https://morozhenko.com/rquickshare/privacy-policy) and [Terms of Use](https://morozhenko.com/rquickshare/terms-of-use).

## Feedback and contributing

For device compatibility issues, crashes or transfer problems, [open a GitHub issue](https://github.com/morozhenko-apps/rquickshare/issues) with reproducible steps, the operating systems and the expected versus observed behavior. Before sharing screenshots or logs, remove private file paths, device identifiers, IP addresses, confidential file names and other personal information.

This is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). Bug reports and proposals are welcome; unsolicited upstream pull requests are not accepted.

## License

rQuickShare is distributed under the [GNU General Public License v3.0](LICENSE). Original authorship, attribution and licensing obligations remain in force.
