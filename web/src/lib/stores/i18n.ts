import { writable, derived } from 'svelte/store';
import type { I18nTexts } from '$lib/types';

// Language detection function
function detectBrowserLanguage(): string {
	if (typeof window === 'undefined') {
		return 'en'; // SSR fallback
	}
	
	// Get browser language preferences
	const browserLanguages = [
		navigator.language,
		...(navigator.languages || [])
	];
	
	// Map of supported languages
	const supportedLanguages = new Set([
		'en', 'es', 'pt', 'fr', 'de', 'ru', 'zh', 'ar', 'eu', 'ca', 'gl'
	]);
	
	// Check each browser language preference
	for (const browserLang of browserLanguages) {
		// Extract language code (e.g., 'es-ES' -> 'es')
		const langCode = browserLang.split('-')[0].toLowerCase();
		
		if (supportedLanguages.has(langCode)) {
			return langCode;
		}
	}
	
	// Default fallback to English
	return 'en';
}

// Initialize with detected language or fallback to English
function initializeLanguage(): string {
	if (typeof window === 'undefined') {
		return 'en'; // SSR fallback
	}
	
	// First check localStorage for user preference
	const storedLang = localStorage.getItem('preferred-language');
	if (storedLang && ['en', 'es', 'pt', 'fr', 'de', 'ru', 'zh', 'ar', 'eu', 'ca', 'gl'].includes(storedLang)) {
		return storedLang;
	}
	
	// If no stored preference, detect browser language
	const detectedLang = detectBrowserLanguage();
	
	// Store the detected language as user preference
	localStorage.setItem('preferred-language', detectedLang);
	
	return detectedLang;
}

// Current language store with automatic detection
export const currentLanguage = writable<string>(initializeLanguage());

// Subscribe to language changes and persist them
currentLanguage.subscribe((language) => {
	if (typeof window !== 'undefined') {
		localStorage.setItem('preferred-language', language);
		
		// Debug information in development
		if (import.meta.env.DEV) {
			console.log(`[i18n] Language changed to: ${language}`);
		}
	}
});

// Debug functions for browser console (development only)
if (typeof window !== 'undefined' && import.meta.env.DEV) {
	// @ts-ignore - Adding to window for debugging
	window.debugI18n = {
		getCurrentLanguage: () => {
			let current;
			const unsubscribe = currentLanguage.subscribe(lang => current = lang);
			unsubscribe();
			return current;
		},
		getBrowserLanguages: () => {
			if (typeof navigator !== 'undefined') {
				return {
					language: navigator.language,
					languages: navigator.languages
				};
			}
			return null;
		},
		getStoredLanguage: () => localStorage.getItem('preferred-language'),
		detectLanguage: () => detectBrowserLanguage(),
		getSupportedLanguages: () => ['en', 'es', 'pt', 'fr', 'de', 'ru', 'zh', 'ar', 'eu', 'ca', 'gl'],
		resetLanguage: () => {
			localStorage.removeItem('preferred-language');
			const detected = detectBrowserLanguage();
			currentLanguage.set(detected);
			return detected;
		},
		setLanguage: (lang: string) => {
			if (['en', 'es', 'pt', 'fr', 'de', 'ru', 'zh', 'ar', 'eu', 'ca', 'gl'].includes(lang)) {
				currentLanguage.set(lang);
				return lang;
			}
			console.error('[i18n] Unsupported language:', lang);
			return null;
		},
		testRTL: () => {
			console.log('[i18n] Testing RTL - switching to Arabic');
			currentLanguage.set('ar');
		},
		testLTR: () => {
			console.log('[i18n] Testing LTR - switching to English');
			currentLanguage.set('en');
		}
	};
	
	console.log('[i18n] Debug functions available at window.debugI18n');
	console.log('[i18n] Try: debugI18n.getBrowserLanguages(), debugI18n.getCurrentLanguage(), etc.');
}

// Comprehensive translation texts with proper grammar for all languages
export const translations: Record<string, I18nTexts> = {
	// English (base)
	en: {
		common: {
			back: 'Back',
			generate: 'Generate',
			copy: 'Copy',
			copied: 'Copied!',
			backToMenu: 'Back to Menu',
			loading: 'Generating...',
			error: 'Error occurred',
			result: 'Result',
			choose: 'Choose',
			type: 'Type',
			length: 'Length',
			generated: 'Generated',
			format: 'Format',
			security: 'Security',
			loadingVersion: 'Loading version...',
			versionsUnavailable: 'Versions unavailable',
			generationDetails: 'Generation Details',
			parametersUsed: 'Parameters Used',
			generateAnother: 'Generate Another',
			adjustSettings: 'Adjust Settings',
			navigateTo: 'Navigate to',
			selectLanguage: 'Select language',
			switchToLightMode: 'Switch to light mode',
			switchToDarkMode: 'Switch to dark mode',
			characters: 'letters',
			alphabet: 'Alphabet',
			generatedValue: 'Generated Value',
			clickToSelect: 'Click the text area to select all, or use the copy button',
			waitGenerating: 'Please wait while generating new value...',
			unknownEndpoint: 'Unknown endpoint type',
			failedToCopy: 'Failed to copy',
			fallbackCopyFailed: 'Fallback copy failed',
			failedToRegenerate: 'Failed to regenerate',
			failedToLoadVersions: 'Failed to load versions',
			mustBeBetween: 'must be between',
			and: 'and',
			cannotExceed: 'cannot exceed',
			optionalPrefix: 'Optional prefix',
			optionalSuffix: 'Optional suffix',
			prefixCannotExceed: 'Prefix cannot exceed 32 letters',
			suffixCannotExceed: 'Suffix cannot exceed 32 letters'
		},
		menu: {
			title: 'Hash Generator',
			subtitle: 'Choose a generation method',
			version: 'Version',
			brandName: 'HashRand Spin',
			description: 'Cryptographically secure hash, password, and API key generator'
		},
		custom: {
			title: 'Custom Hash Generator',
			description: 'Generate customized random hashes',
			generateHash: 'Generate Hash',
			length: 'Length',
			alphabet: 'Alphabet Type',
			prefix: 'Prefix',
			suffix: 'Suffix',
			lengthMustBeBetween: 'Length must be between 2 and 128',
			bitcoinDescription: 'Bitcoin alphabet, excludes confusing letters',
			maxReadabilityDescription: 'Maximum readability, 49 letters',
			completeAlphanumericDescription: 'Complete alphanumeric set',
			maxEntropyDescription: 'Maximum entropy with symbols',
			failedToGenerateHash: 'Failed to generate hash'
		},
		password: {
			title: 'Secure Password Generator',
			description: 'Generate secure passwords',
			generatePassword: 'Generate Password',
			length: 'Length',
			alphabet: 'Character Set',
			maxSecurityDescription: 'Maximum security with symbols (73 chars)',
			easyReadDescription: 'Easy to read and type (49 chars)',
			securityNote: 'Security Note:',
			securityDescription: 'Passwords are generated using cryptographically secure random generation. They are not stored or logged anywhere.',
			noLookAlikeNote: 'No Look-alike alphabet excludes confusable letters. Minimum {0} letters for equivalent security.',
			fullAlphabetNote: 'Full alphabet with symbols provides maximum entropy. Minimum {0} letters for strong security.',
			failedToGeneratePassword: 'Failed to generate password'
		},
		apiKey: {
			title: 'API Key Generator',
			description: 'Generate API keys with ak_ prefix',
			generateApiKey: 'Generate API Key',
			length: 'Length',
			alphabet: 'Character Set',
			standardAlphanumericDescription: 'Standard alphanumeric (62 chars)',
			noConfusingDescription: 'No confusing letters (49 chars)',
			formatNote: 'All API keys are generated with the "ak_" prefix for easy identification. The specified length refers only to the random letters generated (prefix not counted).',
			securityWarning: 'Store API keys securely and never expose them in client-side code or version control. Treat them with the same care as passwords.',
			randomCharacters: 'random characters using',
			noLookAlikeAlphabet: '(easy to type)',
			fullAlphanumericAlphabet: '(maximum compatibility)',
			noLookAlikeNote: 'No Look-alike excludes confusing characters. Minimum {0} characters for equivalent security.',
			fullAlphanumericNote: 'Full alphanumeric provides maximum compatibility. Minimum {0} characters for strong security.',
			formatNotice: 'All API keys are generated with the "ak_" prefix for easy identification. The specified length refers only to the random characters generated (prefix not counted).',
			securityNotice: 'Store API keys securely and never expose them in client-side code or version control. Treat them with the same care as passwords.',
			failedToGenerateApiKey: 'Failed to generate API key'
		},
		alphabets: {
			'base58': 'Base58 (58 chars)',
			'no-look-alike': 'No Look-alike (49 chars)',
			'full': 'Full Alphanumeric (62 chars)',
			'full-with-symbols': 'Full with Symbols (73 chars)'
		}
	},

	// Español (Spain)
	es: {
		common: {
			back: 'Volver',
			generate: 'Generar',
			copy: 'Copiar',
			copied: '¡Copiado!',
			backToMenu: 'Volver al Menú',
			loading: 'Generando...',
			error: 'Ha ocurrido un error',
			result: 'Resultado',
			choose: 'Elegir',
			type: 'Tipo',
			length: 'Longitud',
			generated: 'Generado',
			format: 'Formato',
			security: 'Seguridad',
			loadingVersion: 'Cargando versión...',
			versionsUnavailable: 'Versiones no disponibles',
			generationDetails: 'Detalles de Generación',
			parametersUsed: 'Parámetros Utilizados',
			generateAnother: 'Generar Otro',
			adjustSettings: 'Ajustar Configuración',
			navigateTo: 'Navegar a',
			selectLanguage: 'Seleccionar idioma',
			switchToLightMode: 'Cambiar a modo claro',
			switchToDarkMode: 'Cambiar a modo oscuro',
			characters: 'caracteres',
			alphabet: 'Alfabeto',
			generatedValue: 'Valor Generado',
			clickToSelect: 'Haz clic en el área de texto para seleccionar todo, o usa el botón de copiar',
			waitGenerating: 'Por favor, espera mientras se genera un nuevo valor...',
			unknownEndpoint: 'Tipo de endpoint desconocido',
			failedToCopy: 'Error al copiar',
			fallbackCopyFailed: 'Error en copia de respaldo',
			failedToRegenerate: 'Error al regenerar',
			failedToLoadVersions: 'Error al cargar versiones',
			mustBeBetween: 'debe estar entre',
			and: 'y',
			cannotExceed: 'no puede exceder',
			optionalPrefix: 'Prefijo opcional',
			optionalSuffix: 'Sufijo opcional',
			prefixCannotExceed: 'El prefijo no puede exceder 32 letras',
			suffixCannotExceed: 'El sufijo no puede exceder 32 letras'
		},
		menu: {
			title: 'Generador de Hash',
			subtitle: 'Elige un método de generación',
			version: 'Versión',
			brandName: 'HashRand Spin',
			description: 'Generador criptográficamente seguro de hashes, contraseñas y claves API'
		},
		custom: {
			title: 'Generador de Hash Personalizado',
			description: 'Genera hashes aleatorios personalizados',
			generateHash: 'Generar Hash',
			length: 'Longitud',
			alphabet: 'Tipo de Alfabeto',
			prefix: 'Prefijo',
			suffix: 'Sufijo',
			lengthMustBeBetween: 'La longitud debe estar entre 2 y 128',
			bitcoinDescription: 'Alfabeto Bitcoin, excluye letras confusas',
			maxReadabilityDescription: 'Máxima legibilidad, 49 letras',
			completeAlphanumericDescription: 'Conjunto alfanumérico completo',
			maxEntropyDescription: 'Máxima entropía con símbolos',
			failedToGenerateHash: 'Error al generar hash'
		},
		password: {
			title: 'Generador de Contraseñas Seguras',
			description: 'Genera contraseñas seguras',
			generatePassword: 'Generar Contraseña',
			length: 'Longitud',
			alphabet: 'Conjunto de Caracteres',
			maxSecurityDescription: 'Máxima seguridad con símbolos (73 chars)',
			easyReadDescription: 'Fácil de leer y escribir (49 chars)',
			securityNote: 'Nota de Seguridad:',
			securityDescription: 'Las contraseñas se generan usando generación aleatoria criptográficamente segura. No se almacenan ni registran en ningún lugar.',
			noLookAlikeNote: 'El alfabeto Sin Confusión excluye letras confundibles. Mínimo {0} letras para seguridad equivalente.',
			fullAlphabetNote: 'El alfabeto completo con símbolos proporciona máxima entropía. Mínimo {0} letras para seguridad fuerte.',
			failedToGeneratePassword: 'Error al generar contraseña'
		},
		apiKey: {
			title: 'Generador de Claves API',
			description: 'Genera claves API con prefijo ak_',
			generateApiKey: 'Generar Clave API',
			length: 'Longitud',
			alphabet: 'Conjunto de Caracteres',
			standardAlphanumericDescription: 'Alfanumérico estándar (62 chars)',
			noConfusingDescription: 'Sin letras confusas (49 chars)',
			formatNote: 'Todas las claves API se generan con el prefijo "ak_" para fácil identificación. La longitud especificada se refiere solo a las letras aleatorias generadas (prefijo no contado).',
			securityWarning: 'Almacena las claves API de forma segura y nunca las expongas en código del lado del cliente o control de versiones. Trátalas con el mismo cuidado que las contraseñas.',
			formatPrefix: 'prefijo ak_ +',
			randomCharacters: 'letras aleatorias con',
			noLookAlikeAlphabet: 'alfabeto sin confusión (fácil de escribir)',
			fullAlphanumericAlphabet: 'alfabeto alfanumérico completo',
			failedToGenerateApiKey: 'Error al generar clave API'
		},
		alphabets: {
			'base58': 'Base58 (58 chars)',
			'no-look-alike': 'Sin Confusión (49 chars)',
			'full': 'Alfanumérico Completo (62 chars)',
			'full-with-symbols': 'Completo con Símbolos (73 chars)'
		}
	},

	// Português
	pt: {
		common: {
			back: 'Voltar',
			generate: 'Gerar',
			copy: 'Copiar',
			copied: 'Copiado!',
			backToMenu: 'Voltar ao Menu',
			loading: 'Gerando...',
			error: 'Ocorreu um erro',
			result: 'Resultado',
			choose: 'Escolher',
			type: 'Tipo',
			length: 'Comprimento',
			generated: 'Gerado',
			format: 'Formato',
			security: 'Segurança',
			loadingVersion: 'Carregando versão...',
			versionsUnavailable: 'Versões indisponíveis',
			generationDetails: 'Detalhes da Geração',
			parametersUsed: 'Parâmetros Utilizados',
			generateAnother: 'Gerar Outro',
			adjustSettings: 'Ajustar Configurações',
			navigateTo: 'Navegar para',
			selectLanguage: 'Selecionar idioma',
			switchToLightMode: 'Mudar para modo claro',
			switchToDarkMode: 'Mudar para modo escuro',
			characters: 'caracteres',
			alphabet: 'Alfabeto',
			generatedValue: 'Valor Gerado',
			clickToSelect: 'Clique na área de texto para selecionar tudo, ou use o botão copiar',
			waitGenerating: 'Por favor, aguarde enquanto um novo valor é gerado...',
			unknownEndpoint: 'Tipo de endpoint desconhecido',
			failedToCopy: 'Falha ao copiar',
			fallbackCopyFailed: 'Falha na cópia de backup',
			failedToRegenerate: 'Falha ao regenerar',
			failedToLoadVersions: 'Falha ao carregar versões',
			mustBeBetween: 'deve estar entre',
			and: 'e',
			cannotExceed: 'não pode exceder',
			optionalPrefix: 'Prefixo opcional',
			optionalSuffix: 'Sufixo opcional',
			prefixCannotExceed: 'O prefixo não pode exceder 32 letras',
			suffixCannotExceed: 'O sufixo não pode exceder 32 letras'
		},
		menu: {
			title: 'Gerador de Hash',
			subtitle: 'Escolha um método de geração',
			version: 'Versão',
			brandName: 'HashRand Spin',
			description: 'Gerador criptograficamente seguro de hashes, senhas e chaves API'
		},
		custom: {
			title: 'Gerador de Hash Personalizado',
			description: 'Gere hashes aleatórios personalizados',
			generateHash: 'Gerar Hash',
			length: 'Comprimento',
			alphabet: 'Tipo de Alfabeto',
			prefix: 'Prefixo',
			suffix: 'Sufixo',
			lengthMustBeBetween: 'O comprimento deve estar entre 2 e 128',
			bitcoinDescription: 'Alfabeto Bitcoin, exclui letras confusas',
			maxReadabilityDescription: 'Máxima legibilidade, 49 letras',
			completeAlphanumericDescription: 'Conjunto alfanumérico completo',
			maxEntropyDescription: 'Máxima entropia com símbolos',
			failedToGenerateHash: 'Falha ao gerar hash'
		},
		password: {
			title: 'Gerador de Senhas Seguras',
			description: 'Gere senhas seguras',
			generatePassword: 'Gerar Senha',
			length: 'Comprimento',
			alphabet: 'Conjunto de Caracteres',
			maxSecurityDescription: 'Máxima segurança com símbolos (73 chars)',
			easyReadDescription: 'Fácil de ler e digitar (49 chars)',
			securityNote: 'Nota de Segurança:',
			securityDescription: 'As senhas são geradas usando geração aleatória criptograficamente segura. Não são armazenadas nem registadas em qualquer lugar.',
			noLookAlikeNote: 'O alfabeto Sem Confusão exclui letras confundíveis. Mínimo {0} letras para segurança equivalente.',
			fullAlphabetNote: 'O alfabeto completo com símbolos fornece máxima entropia. Mínimo {0} letras para segurança forte.',
			failedToGeneratePassword: 'Falha ao gerar senha'
		},
		apiKey: {
			title: 'Gerador de Chaves API',
			description: 'Gere chaves API com prefixo ak_',
			generateApiKey: 'Gerar Chave API',
			length: 'Comprimento',
			alphabet: 'Conjunto de Caracteres',
			standardAlphanumericDescription: 'Alfanumérico padrão (62 chars)',
			noConfusingDescription: 'Sem letras confusas (49 chars)',
			formatNote: 'Todas as chaves API são geradas com o prefixo "ak_" para fácil identificação. O comprimento especificado refere-se apenas às letras aleatórias geradas (prefixo não contado).',
			securityWarning: 'Armazene as chaves API com segurança e nunca as exponha em código do lado do cliente ou controlo de versões. Trate-as com o mesmo cuidado que as senhas.',
			formatPrefix: 'prefixo ak_ +',
			randomCharacters: 'caracteres aleatórios com',
			noLookAlikeAlphabet: 'alfabeto sem confusão (fácil de digitar)',
			fullAlphanumericAlphabet: 'alfabeto alfanumérico completo',
			failedToGenerateApiKey: 'Falha ao gerar chave API'
		},
		alphabets: {
			'base58': 'Base58 (58 chars)',
			'no-look-alike': 'Sem Confusão (49 chars)',
			'full': 'Alfanumérico Completo (62 chars)',
			'full-with-symbols': 'Completo com Símbolos (73 chars)'
		}
	},

	// Français
	fr: {
		common: {
			back: 'Retour',
			generate: 'Générer',
			copy: 'Copier',
			copied: 'Copié !',
			backToMenu: 'Retour au Menu',
			loading: 'Génération...',
			error: 'Une erreur s\'est produite',
			result: 'Résultat',
			choose: 'Choisir',
			type: 'Type',
			length: 'Longueur',
			generated: 'Généré',
			format: 'Format',
			security: 'Sécurité',
			loadingVersion: 'Chargement de la version...',
			versionsUnavailable: 'Versions indisponibles',
			generationDetails: 'Détails de Génération',
			parametersUsed: 'Paramètres Utilisés',
			generateAnother: 'Générer un Autre',
			adjustSettings: 'Ajuster les Paramètres',
			navigateTo: 'Naviguer vers',
			selectLanguage: 'Sélectionner la langue',
			switchToLightMode: 'Passer en mode clair',
			switchToDarkMode: 'Passer en mode sombre',
			characters: 'lettres',
			alphabet: 'Alphabet',
			generatedValue: 'Valeur Générée',
			clickToSelect: 'Cliquez sur la zone de texte pour tout sélectionner, ou utilisez le bouton copier',
			waitGenerating: 'Veuillez patienter pendant la génération d\'une nouvelle valeur...',
			unknownEndpoint: 'Type de point de terminaison inconnu',
			failedToCopy: 'Échec de la copie',
			fallbackCopyFailed: 'Échec de la copie de secours',
			failedToRegenerate: 'Échec de la régénération',
			failedToLoadVersions: 'Échec du chargement des versions',
			mustBeBetween: 'doit être entre',
			and: 'et',
			cannotExceed: 'ne peut pas dépasser',
			optionalPrefix: 'Préfixe optionnel',
			optionalSuffix: 'Suffixe optionnel',
			prefixCannotExceed: 'Le préfixe ne peut pas dépasser 32 lettres',
			suffixCannotExceed: 'Le suffixe ne peut pas dépasser 32 lettres'
		},
		menu: {
			title: 'Générateur de Hash',
			subtitle: 'Choisissez une méthode de génération',
			version: 'Version',
			brandName: 'HashRand Spin',
			description: 'Générateur cryptographiquement sécurisé de hashs, mots de passe et clés API'
		},
		custom: {
			title: 'Générateur de Hash Personnalisé',
			description: 'Générez des hashs aléatoires personnalisés',
			generateHash: 'Générer Hash',
			length: 'Longueur',
			alphabet: 'Type d\'Alphabet',
			prefix: 'Préfixe',
			suffix: 'Suffixe',
			lengthMustBeBetween: 'La longueur doit être entre 2 et 128',
			bitcoinDescription: 'Alphabet Bitcoin, exclut les lettres ambigues',
			maxReadabilityDescription: 'Lisibilité maximale, 49 lettres',
			completeAlphanumericDescription: 'Ensemble alphanumérique complet',
			maxEntropyDescription: 'Entropie maximale avec symboles',
			failedToGenerateHash: 'Échec de la génération du hash'
		},
		password: {
			title: 'Générateur de Mots de Passe Sécurisés',
			description: 'Générez des mots de passe sécurisés',
			generatePassword: 'Générer Mot de Passe',
			length: 'Longueur',
			alphabet: 'Jeu de Caractères',
			maxSecurityDescription: 'Sécurité maximale avec symboles (73 chars)',
			easyReadDescription: 'Facile à lire et taper (49 chars)',
			securityNote: 'Note de Sécurité :',
			securityDescription: 'Les mots de passe sont générés en utilisant une génération aléatoire cryptographiquement sécurisée. Ils ne sont stockés ni journalisés nulle part.',
			noLookAlikeNote: 'L\'alphabet Sans Ambiguïté exclut les lettres confuses. Minimum {0} lettres pour une sécurité équivalente.',
			fullAlphabetNote: 'L\'alphabet complet avec symboles fournit une entropie maximale. Minimum {0} lettres pour une sécurité forte.',
			failedToGeneratePassword: 'Échec de la génération du mot de passe'
		},
		apiKey: {
			title: 'Générateur de Clés API',
			description: 'Générez des clés API avec le préfixe ak_',
			generateApiKey: 'Générer Clé API',
			length: 'Longueur',
			alphabet: 'Jeu de Caractères',
			standardAlphanumericDescription: 'Alphanumérique standard (62 chars)',
			noConfusingDescription: 'Aucune lettre confuse (49 chars)',
			formatNote: 'Toutes les clés API sont générées avec le préfixe "ak_" pour une identification facile. La longueur spécifiée ne concerne que les caractères aléatoires générés (préfixe non compté).',
			securityWarning: 'Stockez les clés API en sécurité et ne les exposez jamais dans le code côté client ou le contrôle de version. Traitez-les avec le même soin que les mots de passe.',
			formatPrefix: 'préfixe ak_ +',
			randomCharacters: 'lettres aléatoires avec',
			noLookAlikeAlphabet: 'alphabet sans ambiguïté (facile à taper)',
			fullAlphanumericAlphabet: 'alphabet alphanumérique complet',
			failedToGenerateApiKey: 'Échec de la génération de la clé API'
		},
		alphabets: {
			'base58': 'Base58 (58 chars)',
			'no-look-alike': 'Sans Ambiguïté (49 chars)',
			'full': 'Alphanumérique Complet (62 chars)',
			'full-with-symbols': 'Complet avec Symboles (73 chars)'
		}
	},

	// Deutsch (with cases)
	de: {
		common: {
			back: 'Zurück',
			generate: 'Generieren',
			copy: 'Kopieren',
			copied: 'Kopiert!',
			backToMenu: 'Zurück zum Menü',
			loading: 'Generiert...',
			error: 'Ein Fehler ist aufgetreten',
			result: 'Ergebnis',
			choose: 'Wählen',
			type: 'Typ',
			length: 'Länge',
			generated: 'Generiert',
			format: 'Format',
			security: 'Sicherheit',
			loadingVersion: 'Version wird geladen...',
			versionsUnavailable: 'Versionen nicht verfügbar',
			generationDetails: 'Generierungsdetails',
			parametersUsed: 'Verwendete Parameter',
			generateAnother: 'Einen Anderen Generieren',
			adjustSettings: 'Einstellungen Anpassen',
			navigateTo: 'Navigieren zu',
			selectLanguage: 'Sprache auswählen',
			switchToLightMode: 'Zu hellem Modus wechseln',
			switchToDarkMode: 'Zu dunklem Modus wechseln',
			characters: 'Zeichen',
			alphabet: 'Alphabet',
			generatedValue: 'Generierter Wert',
			clickToSelect: 'Klicken Sie auf das Textfeld, um alles auszuwählen, oder verwenden Sie die Schaltfläche Kopieren',
			waitGenerating: 'Bitte warten Sie, während ein neuer Wert generiert wird...',
			unknownEndpoint: 'Unbekannter Endpunkt-Typ',
			failedToCopy: 'Kopieren fehlgeschlagen',
			fallbackCopyFailed: 'Ersatzkopie fehlgeschlagen',
			failedToRegenerate: 'Regenerierung fehlgeschlagen',
			failedToLoadVersions: 'Laden der Versionen fehlgeschlagen',
			mustBeBetween: 'muss zwischen',
			and: 'und',
			cannotExceed: 'kann nicht überschreiten',
			optionalPrefix: 'Optionales Präfix',
			optionalSuffix: 'Optionales Suffix',
			prefixCannotExceed: 'Das Präfix kann 32 Zeichen nicht überschreiten',
			suffixCannotExceed: 'Das Suffix kann 32 Zeichen nicht überschreiten'
		},
		menu: {
			title: 'Hash-Generator',
			subtitle: 'Wählen Sie eine Generierungsmethode',
			version: 'Version',
			brandName: 'HashRand Spin',
			description: 'Kryptographisch sicherer Generator für Hashes, Passwörter und API-Schlüssel'
		},
		custom: {
			title: 'Benutzerdefinierter Hash-Generator',
			description: 'Generieren Sie benutzerdefinierte zufällige Hashes',
			generateHash: 'Hash Generieren',
			length: 'Länge',
			alphabet: 'Alphabet-Typ',
			prefix: 'Präfix',
			suffix: 'Suffix',
			lengthMustBeBetween: 'Die Länge muss zwischen 2 und 128 liegen',
			bitcoinDescription: 'Bitcoin-Alphabet, schließt verwirrende Zeichen aus',
			maxReadabilityDescription: 'Maximale Lesbarkeit, 49 Zeichen',
			completeAlphanumericDescription: 'Vollständiger alphanumerischer Satz',
			maxEntropyDescription: 'Maximale Entropie mit Symbolen',
			failedToGenerateHash: 'Hash-Generierung fehlgeschlagen'
		},
		password: {
			title: 'Sicherer Passwort-Generator',
			description: 'Generieren Sie sichere Passwörter',
			generatePassword: 'Passwort Generieren',
			length: 'Länge',
			alphabet: 'Zeichensatz',
			maxSecurityDescription: 'Maximale Sicherheit mit Symbolen (73 Zeichen)',
			easyReadDescription: 'Leicht zu lesen und zu tippen (49 Zeichen)',
			securityNote: 'Sicherheitshinweis:',
			securityDescription: 'Passwörter werden mit kryptographisch sicherer Zufallsgenerierung erzeugt. Sie werden nirgends gespeichert oder protokolliert.',
			noLookAlikeNote: 'Das Alphabet ohne Verwechslung schließt verwechselbare Zeichen aus. Mindestens {0} Zeichen für gleichwertige Sicherheit.',
			fullAlphabetNote: 'Das vollständige Alphabet mit Symbolen bietet maximale Entropie. Mindestens {0} Zeichen für starke Sicherheit.',
			failedToGeneratePassword: 'Passwort-Generierung fehlgeschlagen'
		},
		apiKey: {
			title: 'API-Schlüssel-Generator',
			description: 'Generieren Sie API-Schlüssel mit ak_-Präfix',
			generateApiKey: 'API-Schlüssel Generieren',
			length: 'Länge',
			alphabet: 'Zeichensatz',
			standardAlphanumericDescription: 'Standard-Alphanumerisch (62 Zeichen)',
			noConfusingDescription: 'Keine verwirrenden Zeichen (49 Zeichen)',
			formatNote: 'Alle API-Schlüssel werden mit dem Präfix "ak_" zur leichten Identifizierung generiert. Die angegebene Länge bezieht sich nur auf die generierten Zufallszeichen (Präfix nicht mitgezählt).',
			securityWarning: 'Speichern Sie API-Schlüssel sicher und setzen Sie sie niemals in clientseitigem Code oder der Versionskontrolle frei. Behandeln Sie sie mit derselben Sorgfalt wie Passwörter.',
			formatPrefix: 'ak_-Präfix +',
			randomCharacters: 'Zufallszeichen mit',
			noLookAlikeAlphabet: 'Alphabet ohne Verwechslung (leicht zu tippen)',
			fullAlphanumericAlphabet: 'vollständiges alphanumerisches Alphabet',
			failedToGenerateApiKey: 'API-Schlüssel-Generierung fehlgeschlagen'
		},
		alphabets: {
			'base58': 'Base58 (58 Zeichen)',
			'no-look-alike': 'Ohne Verwechslung (49 Zeichen)',
			'full': 'Vollständig Alphanumerisch (62 Zeichen)',
			'full-with-symbols': 'Vollständig mit Symbolen (73 Zeichen)'
		}
	},

	// Русский (Russian with cases)
	ru: {
		common: {
			back: 'Назад',
			generate: 'Генерировать',
			copy: 'Копировать',
			copied: 'Скопировано!',
			backToMenu: 'Вернуться в меню',
			loading: 'Генерация...',
			error: 'Произошла ошибка',
			result: 'Результат',
			choose: 'Выбрать',
			type: 'Тип',
			length: 'Длина',
			generated: 'Сгенерирован',
			format: 'Формат',
			security: 'Безопасность',
			loadingVersion: 'Загрузка версии...',
			versionsUnavailable: 'Версии недоступны',
			generationDetails: 'Детали генерации',
			parametersUsed: 'Используемые параметры',
			generateAnother: 'Сгенерировать ещё',
			adjustSettings: 'Настроить параметры',
			navigateTo: 'Перейти к',
			selectLanguage: 'Выбрать язык',
			switchToLightMode: 'Переключиться на светлый режим',
			switchToDarkMode: 'Переключиться на тёмный режим',
			characters: 'символов',
			alphabet: 'Алфавит',
			generatedValue: 'Сгенерированное значение',
			clickToSelect: 'Щёлкните по текстовому полю, чтобы выделить всё, или используйте кнопку копирования',
			waitGenerating: 'Пожалуйста, подождите, пока генерируется новое значение...',
			unknownEndpoint: 'Неизвестный тип конечной точки',
			failedToCopy: 'Не удалось скопировать',
			fallbackCopyFailed: 'Резервное копирование не удалось',
			failedToRegenerate: 'Не удалось перегенерировать',
			failedToLoadVersions: 'Не удалось загрузить версии',
			mustBeBetween: 'должна быть между',
			and: 'и',
			cannotExceed: 'не может превышать',
			optionalPrefix: 'Необязательный префикс',
			optionalSuffix: 'Необязательный суффикс',
			prefixCannotExceed: 'Префикс не может превышать 32 символа',
			suffixCannotExceed: 'Суффикс не может превышать 32 символа'
		},
		menu: {
			title: 'Генератор хешей',
			subtitle: 'Выберите метод генерации',
			version: 'Версия',
			brandName: 'HashRand Spin',
			description: 'Криптографически безопасный генератор хешей, паролей и API-ключей'
		},
		custom: {
			title: 'Настраиваемый генератор хешей',
			description: 'Генерируйте настраиваемые случайные хеши',
			generateHash: 'Генерировать хеш',
			length: 'Длина',
			alphabet: 'Тип алфавита',
			prefix: 'Префикс',
			suffix: 'Суффикс',
			lengthMustBeBetween: 'Длина должна быть между 2 и 128',
			bitcoinDescription: 'Биткойн алфавит, исключает путающие символы',
			maxReadabilityDescription: 'Максимальная читаемость, 49 символов',
			completeAlphanumericDescription: 'Полный алфавитно-цифровой набор',
			maxEntropyDescription: 'Максимальная энтропия с символами',
			failedToGenerateHash: 'Не удалось сгенерировать хеш'
		},
		password: {
			title: 'Генератор безопасных паролей',
			description: 'Генерируйте безопасные пароли',
			generatePassword: 'Генерировать пароль',
			length: 'Длина',
			alphabet: 'Набор символов',
			maxSecurityDescription: 'Максимальная безопасность с символами (73 символа)',
			easyReadDescription: 'Легко читать и набирать (49 символов)',
			securityNote: 'Примечание по безопасности:',
			securityDescription: 'Пароли генерируются с использованием криптографически безопасной случайной генерации. Они нигде не хранятся и не записываются.',
			noLookAlikeNote: 'Алфавит без путаницы исключает путающие символы. Минимум {0} символов для эквивалентной безопасности.',
			fullAlphabetNote: 'Полный алфавит с символами обеспечивает максимальную энтропию. Минимум {0} символов для надёжной безопасности.',
			failedToGeneratePassword: 'Не удалось сгенерировать пароль'
		},
		apiKey: {
			title: 'Генератор API-ключей',
			description: 'Генерируйте API-ключи с префиксом ak_',
			generateApiKey: 'Генерировать API-ключ',
			length: 'Длина',
			alphabet: 'Набор символов',
			standardAlphanumericDescription: 'Стандартный алфавитно-цифровой (62 символа)',
			noConfusingDescription: 'Без путающих символов (49 символов)',
			formatNote: 'Все API-ключи генерируются с префиксом "ak_" для лёгкой идентификации. Указанная длина относится только к генерируемым случайным символам (префикс не считается).',
			securityWarning: 'Храните API-ключи безопасно и никогда не выставляйте их в клиентском коде или системе контроля версий. Обращайтесь с ними так же осторожно, как с паролями.',
			formatPrefix: 'префикс ak_ +',
			randomCharacters: 'случайных символов с',
			noLookAlikeAlphabet: 'алфавит без путаницы (легко набирать)',
			fullAlphanumericAlphabet: 'полный алфавитно-цифровой алфавит',
			failedToGenerateApiKey: 'Не удалось сгенерировать API-ключ'
		},
		alphabets: {
			'base58': 'Base58 (58 символов)',
			'no-look-alike': 'Без путаницы (49 символов)',
			'full': 'Полный алфавитно-цифровой (62 символа)',
			'full-with-symbols': 'Полный с символами (73 символа)'
		}
	},

	// 中文 (Chinese)
	zh: {
		common: {
			back: '返回',
			generate: '生成',
			copy: '复制',
			copied: '已复制！',
			backToMenu: '返回菜单',
			loading: '生成中...',
			error: '发生错误',
			result: '结果',
			choose: '选择',
			type: '类型',
			length: '长度',
			generated: '已生成',
			format: '格式',
			security: '安全性',
			loadingVersion: '加载版本中...',
			versionsUnavailable: '版本不可用',
			generationDetails: '生成详情',
			parametersUsed: '使用的参数',
			generateAnother: '再生成一个',
			adjustSettings: '调整设置',
			navigateTo: '导航到',
			selectLanguage: '选择语言',
			switchToLightMode: '切换到浅色模式',
			switchToDarkMode: '切换到深色模式',
			characters: '个字符',
			alphabet: '字母表',
			generatedValue: '生成的值',
			clickToSelect: '点击文本区域全选，或使用复制按钮',
			waitGenerating: '请等待生成新值...',
			unknownEndpoint: '未知的端点类型',
			failedToCopy: '复制失败',
			fallbackCopyFailed: '备用复制失败',
			failedToRegenerate: '重新生成失败',
			failedToLoadVersions: '加载版本失败',
			mustBeBetween: '必须在',
			and: '和',
			cannotExceed: '不能超过',
			optionalPrefix: '可选前缀',
			optionalSuffix: '可选后缀',
			prefixCannotExceed: '前缀不能超过32个字符',
			suffixCannotExceed: '后缀不能超过32个字符'
		},
		menu: {
			title: '哈希生成器',
			subtitle: '选择生成方法',
			version: '版本',
			brandName: 'HashRand Spin',
			description: '密码学安全的哈希、密码和API密钥生成器'
		},
		custom: {
			title: '自定义哈希生成器',
			description: '生成自定义随机哈希',
			generateHash: '生成哈希',
			length: '长度',
			alphabet: '字母表类型',
			prefix: '前缀',
			suffix: '后缀',
			lengthMustBeBetween: '长度必须在2到128之间',
			bitcoinDescription: '比特币字母表，排除混淆字符',
			maxReadabilityDescription: '最大可读性，49个字符',
			completeAlphanumericDescription: '完整字母数字集合',
			maxEntropyDescription: '带符号的最大熵',
			failedToGenerateHash: '生成哈希失败'
		},
		password: {
			title: '安全密码生成器',
			description: '生成安全密码',
			generatePassword: '生成密码',
			length: '长度',
			alphabet: '字符集',
			maxSecurityDescription: '带符号的最大安全性（73个字符）',
			easyReadDescription: '易读易输入（49个字符）',
			securityNote: '安全提示：',
			securityDescription: '密码使用密码学安全的随机生成。不会在任何地方存储或记录。',
			noLookAlikeNote: '无相似字母表排除易混淆字符。等效安全性需要最少{0}个字符。',
			fullAlphabetNote: '带符号的完整字母表提供最大熵。强安全性需要最少{0}个字符。',
			failedToGeneratePassword: '生成密码失败'
		},
		apiKey: {
			title: 'API密钥生成器',
			description: '生成带ak_前缀的API密钥',
			generateApiKey: '生成API密钥',
			length: '长度',
			alphabet: '字符集',
			standardAlphanumericDescription: '标准字母数字（62个字符）',
			noConfusingDescription: '无混淆字符（49个字符）',
			formatNote: '所有API密钥都生成带"ak_"前缀以便识别。指定长度仅指生成的随机字符（不计算前缀）。',
			securityWarning: '安全存储API密钥，永远不要在客户端代码或版本控制中暴露它们。像对待密码一样谨慎对待它们。',
			formatPrefix: 'ak_前缀 +',
			randomCharacters: '个随机字符，采用',
			noLookAlikeAlphabet: '无相似字母表（易输入）',
			fullAlphanumericAlphabet: '完整字母数字字母表',
			failedToGenerateApiKey: '生成API密钥失败'
		},
		alphabets: {
			'base58': 'Base58（58个字符）',
			'no-look-alike': '无相似（49个字符）',
			'full': '完整字母数字（62个字符）',
			'full-with-symbols': '带符号完整（73个字符）'
		}
	},

	// العربية (Arabic - RTL consideration)
	ar: {
		common: {
			back: 'رجوع',
			generate: 'توليد',
			copy: 'نسخ',
			copied: 'تم النسخ!',
			backToMenu: 'العودة إلى القائمة',
			loading: 'جاري التوليد...',
			error: 'حدث خطأ',
			result: 'النتيجة',
			choose: 'اختر',
			type: 'النوع',
			length: 'الطول',
			generated: 'تم التوليد',
			format: 'التنسيق',
			security: 'الأمان',
			loadingVersion: 'تحميل الإصدار...',
			versionsUnavailable: 'الإصدارات غير متاحة',
			generationDetails: 'تفاصيل التوليد',
			parametersUsed: 'المعاملات المستخدمة',
			generateAnother: 'توليد آخر',
			adjustSettings: 'تعديل الإعدادات',
			navigateTo: 'الانتقال إلى',
			selectLanguage: 'اختر اللغة',
			switchToLightMode: 'التبديل إلى الوضع الفاتح',
			switchToDarkMode: 'التبديل إلى الوضع المظلم',
			characters: 'حرف',
			alphabet: 'الأبجدية',
			generatedValue: 'القيمة المولدة',
			clickToSelect: 'انقر على منطقة النص لتحديد الكل، أو استخدم زر النسخ',
			waitGenerating: 'يرجى الانتظار أثناء توليد قيمة جديدة...',
			unknownEndpoint: 'نوع نقطة نهاية غير معروف',
			failedToCopy: 'فشل في النسخ',
			fallbackCopyFailed: 'فشل في النسخ الاحتياطي',
			failedToRegenerate: 'فشل في إعادة التوليد',
			failedToLoadVersions: 'فشل في تحميل الإصدارات',
			mustBeBetween: 'يجب أن يكون بين',
			and: 'و',
			cannotExceed: 'لا يمكن أن يتجاوز',
			optionalPrefix: 'البادئة اختيارية',
			optionalSuffix: 'اللاحقة اختيارية',
			prefixCannotExceed: 'البادئة لا يمكن أن تتجاوز 32 حرفاً',
			suffixCannotExceed: 'اللاحقة لا يمكن أن تتجاوز 32 حرفاً'
		},
		menu: {
			title: 'مولد الهاش',
			subtitle: 'اختر طريقة التوليد',
			version: 'الإصدار',
			brandName: 'HashRand Spin',
			description: 'مولد آمن تشفيرياً للهاش وكلمات المرور ومفاتيح API'
		},
		custom: {
			title: 'مولد الهاش المخصص',
			description: 'ولد هاش عشوائي مخصص',
			generateHash: 'توليد هاش',
			length: 'الطول',
			alphabet: 'نوع الأبجدية',
			prefix: 'البادئة',
			suffix: 'اللاحقة',
			lengthMustBeBetween: 'الطول يجب أن يكون بين 2 و 128',
			bitcoinDescription: 'أبجدية البيتكوين، تستبعد الأحرف المربكة',
			maxReadabilityDescription: 'أقصى قابلية قراءة، 49 حرف',
			completeAlphanumericDescription: 'مجموعة أبجدية رقمية كاملة',
			maxEntropyDescription: 'أقصى عشوائية مع الرموز',
			failedToGenerateHash: 'فشل في توليد الهاش'
		},
		password: {
			title: 'مولد كلمات المرور الآمنة',
			description: 'ولد كلمات مرور آمنة',
			generatePassword: 'توليد كلمة مرور',
			length: 'الطول',
			alphabet: 'مجموعة الأحرف',
			maxSecurityDescription: 'أقصى أمان مع الرموز (73 حرف)',
			easyReadDescription: 'سهل القراءة والكتابة (49 حرف)',
			securityNote: 'ملاحظة أمنية:',
			securityDescription: 'يتم توليد كلمات المرور باستخدام توليد عشوائي آمن تشفيرياً. لا يتم تخزينها أو تسجيلها في أي مكان.',
			noLookAlikeNote: 'أبجدية عدم التشابه تستبعد الأحرف المربكة. الحد الأدنى {0} حرف للأمان المكافئ.',
			fullAlphabetNote: 'الأبجدية الكاملة مع الرموز توفر أقصى عشوائية. الحد الأدنى {0} حرف للأمان القوي.',
			failedToGeneratePassword: 'فشل في توليد كلمة المرور'
		},
		apiKey: {
			title: 'مولد مفاتيح API',
			description: 'ولد مفاتيح API مع البادئة ak_',
			generateApiKey: 'توليد مفتاح API',
			length: 'الطول',
			alphabet: 'مجموعة الأحرف',
			standardAlphanumericDescription: 'أبجدية رقمية قياسية (62 حرف)',
			noConfusingDescription: 'بدون أحرف مربكة (49 حرف)',
			formatNote: 'جميع مفاتيح API تُولد مع البادئة "ak_" للتعرف السهل. الطول المحدد يشير فقط إلى الأحرف العشوائية المولدة (البادئة غير محسوبة).',
			securityWarning: 'احفظ مفاتيح API بأمان ولا تعرضها أبداً في كود العميل أو التحكم في الإصدارات. عاملها بنفس عناية كلمات المرور.',
			formatPrefix: 'بادئة ak_ +',
			randomCharacters: 'حرف عشوائي مع',
			noLookAlikeAlphabet: 'أبجدية عدم التشابه (سهل الكتابة)',
			fullAlphanumericAlphabet: 'أبجدية أبجدية رقمية كاملة',
			failedToGenerateApiKey: 'فشل في توليد مفتاح API'
		},
		alphabets: {
			'base58': 'Base58 (58 حرف)',
			'no-look-alike': 'عدم التشابه (49 حرف)',
			'full': 'أبجدية رقمية كاملة (62 حرف)',
			'full-with-symbols': 'كاملة مع الرموز (73 حرف)'
		}
	},

	// Euskera (Basque - with proper declension cases)
	eu: {
		common: {
			back: 'Atzera',
			generate: 'Sortu',
			copy: 'Kopiatu',
			copied: 'Kopiatuta!',
			backToMenu: 'Menura itzuli',
			loading: 'Sortzen...',
			error: 'Errorea gertatu da',
			result: 'Emaitza',
			choose: 'Aukeratu',
			type: 'Mota',
			length: 'Luzeera',
			generated: 'Sortutakoa',
			format: 'Formatua',
			security: 'Segurtasuna',
			loadingVersion: 'Bertsioa kargatzen...',
			versionsUnavailable: 'Bertsioak ez daude eskuragarri',
			generationDetails: 'Sorkuntzaren Xehetasunak',
			parametersUsed: 'Erabilitako Parametroak',
			generateAnother: 'Beste bat sortu',
			adjustSettings: 'Ezarpenak aldatu',
			navigateTo: 'Hona nabigatu',
			selectLanguage: 'Hizkuntza aukeratu',
			switchToLightMode: 'Argi modura aldatu',
			switchToDarkMode: 'Ilun modura aldatu',
			characters: 'hizki',
			alphabet: 'Alfabetoa',
			generatedValue: 'Sortutako Balioa',
			clickToSelect: 'Testu-eremuan klik egin guztia hautatzeko, edo kopiatzeko botoia erabili',
			waitGenerating: 'Mesedez, itxaron balio berria sortzen den bitartean...',
			unknownEndpoint: 'Amaiera-puntu mota ezezaguna',
			failedToCopy: 'Kopiatzeak huts egin du',
			fallbackCopyFailed: 'Ordezko kopiaketa-sistemak huts egin du',
			failedToRegenerate: 'Berriz sortzeak huts egin du',
			failedToLoadVersions: 'Bertsioak kargatzeak huts egin du',
			mustBeBetween: 'tartean egon behar du',
			and: 'eta',
			cannotExceed: 'ezin du gainditu',
			optionalPrefix: 'Aukerako aurrizkia',
			optionalSuffix: 'Aukerako atzizkia',
			prefixCannotExceed: 'Aurrizkiak 32 hizki ezin ditu gainditu',
			suffixCannotExceed: 'Atzizkiak 32 hizki ezin ditu gainditu'
		},
		menu: {
			title: 'Hash-Sortzailea',
			subtitle: 'Sorkuntzako metodoa aukeratu',
			version: 'Bertsioa',
			brandName: 'HashRand Spin',
			description: 'Hash, pasahitz eta API gako kriptografikoki seguruak sortzeko tresna'
		},
		custom: {
			title: 'Hash-Sortzaile Pertsonalizatua',
			description: 'Nahi bezalako ausazko hash-ak sortu',
			generateHash: 'Hash-a sortu',
			length: 'Luzeera',
			alphabet: 'Alfabeto Mota',
			prefix: 'Aurrizkia',
			suffix: 'Atzizkia',
			lengthMustBeBetween: 'Luzerak 2 eta 128 artean egon behar du',
			bitcoinDescription: 'Bitcoin-alfabetoa, hizki nahasgarriak kanpoan uzten ditu',
			maxReadabilityDescription: 'Irakurgarritasun handiena, 49 hizki',
			completeAlphanumericDescription: 'Alfabeto alfanumeriko osoa',
			maxEntropyDescription: 'Entropia handiena sinboloekin',
			failedToGenerateHash: 'Hash-a sortzeak huts egin du'
		},
		password: {
			title: 'Pasahitz Seguruen Sortzailea',
			description: 'Pasahitz seguruak sortu',
			generatePassword: 'Pasahitza sortu',
			length: 'Luzeera',
			alphabet: 'Hizki-multzoa',
			maxSecurityDescription: 'Segurtasun handiena sinboloekin (73 hizki)',
			easyReadDescription: 'Erraz irakurri eta idazteko (49 hizki)',
			securityNote: 'Segurtasunari buruzko oharra:',
			securityDescription: 'Pasahitzak kriptografikoki segurua den ausazko sorkuntzaren bidez sortzen dira. Ez dira inon gordetzen edo erregistratzen.',
			noLookAlikeNote: 'Hizki nahasgarririk gabeko alfabetoak hizki antzekoak kanpoan uzten ditu. Gutxieneko {0} hizki behar dira segurtasun baliokidea lortzeko.',
			fullAlphabetNote: 'Sinboloak dituen alfabeto osoak entropia handiena ematen du. Gutxieneko {0} hizki behar dira segurtasun sendoa lortzeko.',
			failedToGeneratePassword: 'Pasahitza sortzeak huts egin du'
		},
		apiKey: {
			title: 'API Gakoen Sortzailea',
			description: 'ak_ aurrizkidun API gakoak sortu',
			generateApiKey: 'API gakoa sortu',
			length: 'Luzeera',
			alphabet: 'Hizki-multzoa',
			standardAlphanumericDescription: 'Alfabeto alfanumeriko estandarra (62 hizki)',
			noConfusingDescription: 'Hizki nahasgarririk gabe (49 hizki)',
			formatNote: 'API gako guztiak "ak_" aurrizkiarekin sortzen dira identifikazioa errazagoa izateko. Zehaztutako luzerak soilik sortutako ausazko hizkiak hartzen ditu kontuan (aurrizkirik gabe).',
			securityWarning: 'API gakoak modu seguruan gorde eta inoiz ez jarri bezero-aldeko kodean edo bertsio-kontrolean. Pasahitzekin bezalako arretaz tratatu.',
			formatPrefix: 'ak_ aurrizkia +',
			randomCharacters: 'ausazko hizki hauek erabiliz',
			noLookAlikeAlphabet: 'hizki nahasgarririk gabeko alfabetoa (erraz idazteko)',
			fullAlphanumericAlphabet: 'alfabeto alfanumeriko osoa',
			failedToGenerateApiKey: 'API gakoa sortzeak huts egin du'
		},
		alphabets: {
			'base58': 'Base58 (58 hizki)',
			'no-look-alike': 'Nahasgarririk gabe (49 hizki)',
			'full': 'Alfabeto Alfanumeriko Osoa (62 hizki)',
			'full-with-symbols': 'Osoa Sinboloekin (73 hizki)'
		}
	},

	// Català
	ca: {
		common: {
			back: 'Enrere',
			generate: 'Generar',
			copy: 'Copiar',
			copied: 'Copiat!',
			backToMenu: 'Tornar al menú',
			loading: 'Generant...',
			error: 'S\'ha produït un error',
			result: 'Resultat',
			choose: 'Triar',
			type: 'Tipus',
			length: 'Longitud',
			generated: 'Generat',
			format: 'Format',
			security: 'Seguretat',
			loadingVersion: 'Carregant versió...',
			versionsUnavailable: 'Versions no disponibles',
			generationDetails: 'Detalls de generació',
			parametersUsed: 'Paràmetres utilitzats',
			generateAnother: 'Generar un altre',
			adjustSettings: 'Ajustar configuració',
			navigateTo: 'Navegar a',
			selectLanguage: 'Seleccionar idioma',
			switchToLightMode: 'Canviar al mode clar',
			switchToDarkMode: 'Canviar al mode fosc',
			characters: 'lletres',
			alphabet: 'Alfabet',
			generatedValue: 'Valor generat',
			clickToSelect: 'Feu clic a l\'àrea de text per seleccionar-ho tot, o utilitzeu el botó de copiar',
			waitGenerating: 'Si us plau, espereu mentre es genera un nou valor...',
			unknownEndpoint: 'Tipus d\'endpoint desconegut',
			failedToCopy: 'Ha fallat la còpia',
			fallbackCopyFailed: 'Ha fallat la còpia de reserva',
			failedToRegenerate: 'Ha fallat la regeneració',
			failedToLoadVersions: 'Ha fallat carregar les versions',
			mustBeBetween: 'ha d\'estar entre',
			and: 'i',
			cannotExceed: 'no pot superar',
			optionalPrefix: 'Prefix opcional',
			optionalSuffix: 'Sufix opcional',
			prefixCannotExceed: 'El prefix no pot superar 32 lletres',
			suffixCannotExceed: 'El sufix no pot superar 32 lletres'
		},
		menu: {
			title: 'Generador de Hash',
			subtitle: 'Trieu un mètode de generació',
			version: 'Versió',
			brandName: 'HashRand Spin',
			description: 'Generador criptogràficament segur de hashs, contrasenyes i claus API'
		},
		custom: {
			title: 'Generador de Hash personalitzat',
			description: 'Genereu hashs aleatoris personalitzats',
			generateHash: 'Generar Hash',
			length: 'Longitud',
			alphabet: 'Tipus d\'alfabet',
			prefix: 'Prefix',
			suffix: 'Sufix',
			lengthMustBeBetween: 'La longitud ha d\'estar entre 2 i 128',
			bitcoinDescription: 'Alfabet Bitcoin, exclou lletres confuses',
			maxReadabilityDescription: 'Màxima llegibilitat, 49 lletres',
			completeAlphanumericDescription: 'Conjunt alfanumèric complet',
			maxEntropyDescription: 'Màxima entropia amb símbols',
			failedToGenerateHash: 'Ha fallat generar el hash'
		},
		password: {
			title: 'Generador de contrasenyes segures',
			description: 'Genereu contrasenyes segures',
			generatePassword: 'Generar Contrasenya',
			length: 'Longitud',
			alphabet: 'Conjunt de lletres',
			maxSecurityDescription: 'Màxima seguretat amb símbols (73 caràcters)',
			easyReadDescription: 'Fàcil de llegir i escriure (49 caràcters)',
			securityNote: 'Nota de seguretat:',
			securityDescription: 'Les contrasenyes es generen utilitzant generació aleatòria criptogràficament segura. No s\'emmagatzemen ni es registren enlloc.',
			noLookAlikeNote: 'L\'alfabet sense confusió exclou caràcters confusos. Mínim {0} caràcters per seguretat equivalent.',
			fullAlphabetNote: 'L\'alfabet complet amb símbols proporciona màxima entropia. Mínim {0} caràcters per seguretat forta.',
			failedToGeneratePassword: 'Ha fallat generar la contrasenya'
		},
		apiKey: {
			title: 'Generador de claus API',
			description: 'Genereu claus API amb prefix ak_',
			generateApiKey: 'Generar Clau API',
			length: 'Longitud',
			alphabet: 'Conjunt de lletres',
			standardAlphanumericDescription: 'Alfanumèric estàndard (62 caràcters)',
			noConfusingDescription: 'Sense caràcters confusos (49 caràcters)',
			formatNote: 'Totes les claus API es generen amb el prefix "ak_" per facilitar la identificació. La longitud especificada es refereix només als caràcters aleatoris generats (prefix no comptat).',
			securityWarning: 'Emmagatzemeu les claus API de forma segura i mai les exposeu en codi del costat del client o control de versions. Tracteu-les amb la mateixa cura que les contrasenyes.',
			formatPrefix: 'prefix ak_ +',
			randomCharacters: 'lletres aleatòries amb',
			noLookAlikeAlphabet: 'alfabet sense confusió (fàcil d\'escriure)',
			fullAlphanumericAlphabet: 'alfabet alfanumèric complet',
			failedToGenerateApiKey: 'Ha fallat generar la clau API'
		},
		alphabets: {
			'base58': 'Base58 (58 caràcters)',
			'no-look-alike': 'Sense confusió (49 caràcters)',
			'full': 'Alfanumèric complet (62 caràcters)',
			'full-with-symbols': 'Complet amb símbols (73 caràcters)'
		}
	},

	// Galego
	gl: {
		common: {
			back: 'Atrás',
			generate: 'Xerar',
			copy: 'Copiar',
			copied: 'Copiado!',
			backToMenu: 'Volver ao menú',
			loading: 'Xerando...',
			error: 'Produciuse un erro',
			result: 'Resultado',
			choose: 'Escoller',
			type: 'Tipo',
			length: 'Lonxitude',
			generated: 'Xerado',
			format: 'Formato',
			security: 'Seguridade',
			loadingVersion: 'Cargando versión...',
			versionsUnavailable: 'Versións non dispoñibles',
			generationDetails: 'Detalles de xeración',
			parametersUsed: 'Parámetros utilizados',
			generateAnother: 'Xerar outro',
			adjustSettings: 'Axustar configuración',
			navigateTo: 'Navegar a',
			selectLanguage: 'Seleccionar idioma',
			switchToLightMode: 'Cambiar ao modo claro',
			switchToDarkMode: 'Cambiar ao modo escuro',
			characters: 'caracteres',
			alphabet: 'Alfabeto',
			generatedValue: 'Valor xerado',
			clickToSelect: 'Fai clic na área de texto para seleccionar todo, ou usa o botón copiar',
			waitGenerating: 'Por favor, agarda mentres se xera un novo valor...',
			unknownEndpoint: 'Tipo de endpoint descoñecido',
			failedToCopy: 'Fallou ao copiar',
			fallbackCopyFailed: 'Fallou a copia de respaldo',
			failedToRegenerate: 'Fallou ao rexerar',
			failedToLoadVersions: 'Fallou cargar as versións',
			mustBeBetween: 'debe estar entre',
			and: 'e',
			cannotExceed: 'non pode superar',
			optionalPrefix: 'Prefixo opcional',
			optionalSuffix: 'Sufixo opcional',
			prefixCannotExceed: 'O prefixo non pode superar 32 caracteres',
			suffixCannotExceed: 'O sufixo non pode superar 32 caracteres'
		},
		menu: {
			title: 'Xerador de Hash',
			subtitle: 'Escolle un método de xeración',
			version: 'Versión',
			brandName: 'HashRand Spin',
			description: 'Xerador criptograficamente seguro de hashes, contrasinais e chaves API'
		},
		custom: {
			title: 'Xerador de Hash personalizado',
			description: 'Xera hashes aleatorios personalizados',
			generateHash: 'Xerar Hash',
			length: 'Lonxitude',
			alphabet: 'Tipo de alfabeto',
			prefix: 'Prefixo',
			suffix: 'Sufixo',
			lengthMustBeBetween: 'A lonxitude debe estar entre 2 e 128',
			bitcoinDescription: 'Alfabeto Bitcoin, exclúe caracteres confusos',
			maxReadabilityDescription: 'Máxima lexibilidade, 49 caracteres',
			completeAlphanumericDescription: 'Conxunto alfanumérico completo',
			maxEntropyDescription: 'Máxima entropía con símbolos',
			failedToGenerateHash: 'Fallou xerar o hash'
		},
		password: {
			title: 'Xerador de contrasinais seguros',
			description: 'Xera contrasinais seguros',
			generatePassword: 'Xerar Contrasinal',
			length: 'Lonxitude',
			alphabet: 'Conxunto de caracteres',
			maxSecurityDescription: 'Máxima seguridade con símbolos (73 caracteres)',
			easyReadDescription: 'Fácil de ler e escribir (49 caracteres)',
			securityNote: 'Nota de seguridade:',
			securityDescription: 'Os contrasinais xéranse usando xeración aleatoria criptograficamente segura. Non se almacenan nin se rexistran en ningures.',
			noLookAlikeNote: 'O alfabeto sen confusión exclúe letras confundibles. Mínimo {0} letras para seguridade equivalente.',
			fullAlphabetNote: 'O alfabeto completo con símbolos proporciona máxima entropía. Mínimo {0} letras para seguridade forte.',
			failedToGeneratePassword: 'Fallou xerar o contrasinal'
		},
		apiKey: {
			title: 'Xerador de chaves API',
			description: 'Xera chaves API con prefixo ak_',
			generateApiKey: 'Xerar Chave API',
			length: 'Lonxitude',
			alphabet: 'Conxunto de caracteres',
			standardAlphanumericDescription: 'Alfanumérico estándar (62 caracteres)',
			noConfusingDescription: 'Sen letras confusas (49 letras)',
			formatNote: 'Todas as chaves API xéranse co prefixo "ak_" para facilitar a identificación. A lonxitude especificada refírese só ás letras aleatorias xeradas (prefixo non contado).',
			securityWarning: 'Almacena as chaves API de forma segura e nunca as exponñas en código do lado do cliente ou control de versións. Trátalas co mesmo coidado que os contrasinais.',
			formatPrefix: 'prefixo ak_ +',
			randomCharacters: 'letras aleatorias con',
			noLookAlikeAlphabet: 'alfabeto sen confusión (fácil de escribir)',
			fullAlphanumericAlphabet: 'alfabeto alfanumérico completo',
			failedToGenerateApiKey: 'Fallou xerar a chave API'
		},
		alphabets: {
			'base58': 'Base58 (58 caracteres)',
			'no-look-alike': 'Sen confusión (49 caracteres)',
			'full': 'Alfanumérico completo (62 caracteres)',
			'full-with-symbols': 'Completo con símbolos (73 caracteres)'
		}
	}
};

// Translation function
export function t(key: string, lang: string = 'en'): string {
	const keys = key.split('.');
	let value: any = translations[lang];
	
	for (const k of keys) {
		if (value && typeof value === 'object' && k in value) {
			value = value[k];
		} else {
			return key; // Return key if translation not found
		}
	}
	
	return typeof value === 'string' ? value : key;
}

// Reactive translation function that works with the store
export const _ = derived(currentLanguage, (lang) => {
	return (key: string) => t(key, lang);
});

// Get current translations object
export const currentTexts = derived(currentLanguage, (lang) => {
	return translations[lang] || translations.en;
});