export type ClipboardWriter = (text: string) => Promise<void>;

export type ClipboardWriteMethod = 'primary' | 'fallback' | null;

export async function writeClipboardWithFallback(
	text: string,
	primary: ClipboardWriter,
	fallback?: ClipboardWriter,
): Promise<ClipboardWriteMethod> {
	try {
		await primary(text);
		return 'primary';
	} catch {
		if (!fallback) return null;
	}

	try {
		await fallback(text);
		return 'fallback';
	} catch {
		return null;
	}
}
