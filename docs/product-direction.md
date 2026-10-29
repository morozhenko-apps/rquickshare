# rQuickShare product direction

rQuickShare is an independently maintained nearby-sharing client based on [Martichou/rquickshare](https://github.com/Martichou/rquickshare). Its purpose is to make direct sharing between a computer and nearby devices comfortable, predictable and safe.

The product name is rQuickShare. Platform packages may use different package identifiers, but should not create competing product identities or overwrite original upstream builds without an explicit migration.

## Everyday sharing

The native Linux desktop client discovers devices, sends files and text, and presents incoming transfers with explicit Accept and Decline controls. Verification information must remain tied to the actual transfer and peer identity. The tray card is small, while a separate view presents ongoing and completed transfers.

Convenience should not come at the cost of trust. A peer's visible name or network address alone is not proof that two routes belong to the same device. Paired identity requires authenticated, session-bound evidence. Unknown peers continue to require appropriate consent.

## Reliability

Transfer work has an identifiable owner and a bounded lifetime. Rejecting or cancelling must not turn into false success. Interrupted transfers must not reveal unrelated files, and partial output must not masquerade as completed data. UI presentation cannot silently change protocol outcomes.

Discovery, Bluetooth, local network transport, filesystem access and desktop presentation are separate failure domains. The app should keep working when one optional integration is unavailable and should explain uncertainty instead of fabricating a successful transfer.

## Desktop and interoperability

The native experience currently centers on Linux and interaction with Android Quick Share devices. Additional platform work is a separate technical and physical acceptance decision, not a promise attached to a roadmap date.

Preserve the upstream project's GPL-3.0 licensing and attribution. Real device compatibility, native UI usability and secure transfer behavior take precedence over expanding feature claims.
