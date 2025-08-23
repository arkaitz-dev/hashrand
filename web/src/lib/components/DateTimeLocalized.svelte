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
		// Helper function for time formatting
		const getTimeString = (date: Date) => {
			const hours = String(date.getHours()).padStart(2, '0');
			const minutes = String(date.getMinutes()).padStart(2, '0');
			const seconds = String(date.getSeconds()).padStart(2, '0');
			return `${hours}:${minutes}:${seconds}`;
		};

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
			const timeString = getTimeString(timestamp);

			return `${year}ko ${monthName}ak ${day}, ${timeString}`;
		}

		// Custom format for Galician (gallego) as fallback if Intl fails
		if ($currentLanguage === 'gl') {
			const galegoMonths = [
				'xan.', 'feb.', 'mar.', 'abr.', 'mai.', 'xuñ.',
				'xul.', 'ago.', 'set.', 'out.', 'nov.', 'dec.'
			];
			
			// Try Intl first, fall back to custom if it fails
			try {
				const result = new Intl.DateTimeFormat('gl-ES', options || defaultOptions).format(timestamp);
				// Test if the formatting actually worked (some browsers return English month names)
				if (result.includes('Jan') || result.includes('Feb') || result.includes('Mar')) {
					throw new Error('Intl fallback needed');
				}
				return result;
			} catch {
				const year = timestamp.getFullYear();
				const monthName = galegoMonths[timestamp.getMonth()];
				const day = timestamp.getDate();
				const timeString = getTimeString(timestamp);
				return `${day} ${monthName} ${year}, ${timeString}`;
			}
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
			const formatter = new Intl.DateTimeFormat(locale, formatOptions);
			const result = formatter.format(timestamp);
			
			// Additional validation: check if the result contains expected localized content
			// If we get English month names when expecting another language, try fallback
			if (locale !== 'en-US' && locale !== 'en-GB') {
				const englishMonths = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 
									  'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec',
									  'January', 'February', 'March', 'April', 'June',
									  'July', 'August', 'September', 'October', 'November', 'December'];
				
				const hasEnglishMonth = englishMonths.some(month => result.includes(month));
				if (hasEnglishMonth) {
					throw new Error('Locale not properly supported, falling back');
				}
			}
			
			return result;
		} catch {
			// Fallback to English if locale is not supported
			try {
				return new Intl.DateTimeFormat('en-US', formatOptions).format(timestamp);
			} catch {
				// Ultimate fallback: manual formatting
				const year = timestamp.getFullYear();
				const month = String(timestamp.getMonth() + 1).padStart(2, '0');
				const day = String(timestamp.getDate()).padStart(2, '0');
				const timeString = getTimeString(timestamp);
				return `${year}-${month}-${day}, ${timeString}`;
			}
		}
	});
</script>

<span class={wrapperClass}>{formattedTimestamp}</span>
