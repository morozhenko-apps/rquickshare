# rQuickShare

rQuickShare is a free, open-source app for sharing files, images and text with nearby devices. Its native Linux desktop client works with Android devices using Google Quick Share-compatible discovery and transfer flows.

## Why I work on it

Sharing something between a computer and a phone should be straightforward. I wanted to send a photo, move a document or copy a small piece of text without uploading it somewhere, opening a web service or dealing with cables.

rQuickShare brings that everyday convenience to the Linux desktop. It keeps the sharing controls nearby in a small tray interface while making sure receiving something always stays your decision.

## What it does

The app discovers nearby devices and lets you send files or text to a selected recipient. Incoming requests display a clear choice to accept or decline, including verification information when needed. Transfers continue independently of whether the tray card is visible.

You can see incoming and outgoing transfers, track their progress and open received files or their containing folder. The native client integrates with the desktop to keep the sharing experience accessible without leaving a large window open.

Discovery and transfer take place across the supported local network and nearby connectivity mechanisms rather than through a separate rQuickShare cloud-storage account. Consent, transfer identity and destination safety remain explicit.

## Platform

The native desktop application targets **Linux**. Its current interface uses a tray card and native desktop integration.

## Feedback

Found a bug or a device compatibility problem? Please [open an issue](https://github.com/morozhenko-apps/rquickshare/issues) with the devices involved, steps to reproduce it and the behavior you observed. Suggestions that make nearby sharing easier are welcome there too. Remove identifying device names, private file paths and confidential content before sharing screenshots or logs.

This is an independently maintained fork of [Martichou/rquickshare](https://github.com/Martichou/rquickshare). External bug reports and ideas are welcome, but unsolicited upstream pull requests are not accepted.

## License

The project is distributed under the [GNU General Public License v3.0](LICENSE). Original authorship and licensing obligations remain in force.
