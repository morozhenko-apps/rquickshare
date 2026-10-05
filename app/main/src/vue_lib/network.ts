export interface ListeningPortParseResult {
	port: number | null;
	error: string | null;
}

export function parseListeningPort(raw: string): ListeningPortParseResult {
	const value = raw.trim();

	if (value === '') {
		return { port: null, error: null };
	}

	const port = Number(value);
	if (!Number.isInteger(port) || port < 1024 || port > 65535) {
		return {
			port: null,
			error: 'Use a port from 1024 to 65535.',
		};
	}

	return { port, error: null };
}
