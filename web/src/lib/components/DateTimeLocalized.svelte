<script lang="ts">
	import { currentLanguage } from '$lib/stores/i18n';

	interface Props {
		timestamp: Date;
		options?: Intl.DateTimeFormatOptions;
		class?: string;
	}

	let { timestamp, options, class: wrapperClass = '' }: Props = $props();

	// Default formatting options
	const defaultOptions: Intl.DateTimeFormatOptions = {
		year: 'numeric',
		month: 'short',
		day: 'numeric',
		hour: '2-digit',
		minute: '2-digit',
		second: '2-digit'
	};

	// Reactive formatted timestamp that updates when language changes
	const formattedTimestamp = $derived.by(() => {
		// Custom format for Basque (euskera): [yyyy]ko [mes en euskera]k [d], hh:mm:ss
		if ($currentLanguage === 'eu') {
			const euskeraMonths = [
				'urtarril',
				'otsail',
				'martxo',
				'apiril',
				'maiatz',
				'ekain',
				'uztail',
				'abuztu',
				'irail',
				'urri',
				'azaro',
				'abendu'
			];

			const year = timestamp.getFullYear();
			const monthName = euskeraMonths[timestamp.getMonth()];
			const day = timestamp.getDate();
			const hours = String(timestamp.getHours()).padStart(2, '0');
			const minutes = String(timestamp.getMinutes()).padStart(2, '0');
			const seconds = String(timestamp.getSeconds()).padStart(2, '0');

			return `${year}ko ${monthName}ak ${day}, ${hours}:${minutes}:${seconds}`;
		}

		// Map language codes to locale identifiers for date formatting
		const localeMap: Record<string, string> = {
			en: 'en-US',
			es: 'es-ES',
			pt: 'pt-PT',
			fr: 'fr-FR',
			de: 'de-DE',
			ru: 'ru-RU',
			zh: 'zh-CN',
			ar: 'ar-SA',
			hi: 'hi-IN',
			ja: 'ja-JP',
			ca: 'ca-ES',
			gl: 'gl-ES'
		};

		const locale = localeMap[$currentLanguage] || 'en-US';
		const formatOptions = options || defaultOptions;

		try {
			return new Intl.DateTimeFormat(locale, formatOptions).format(timestamp);
		} catch {
			// Fallback to English if locale is not supported
			return new Intl.DateTimeFormat('en-US', formatOptions).format(timestamp);
		}
	});
</script>

<span class={wrapperClass}>{formattedTimestamp}</span>
