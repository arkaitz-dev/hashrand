import type { I18nTexts } from '$lib/types';

export const es: I18nTexts = {
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
		suffixCannotExceed: 'El sufijo no puede exceder 32 letras',
		yes: 'Sí',
		no: 'No'
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
		securityDescription:
			'Las contraseñas se generan usando generación aleatoria criptográficamente segura. No se almacenan ni registran en ningún lugar.',
		noLookAlikeNote:
			'El alfabeto Sin Confusión excluye letras confundibles. Mínimo {0} letras para seguridad equivalente.',
		fullAlphabetNote:
			'El alfabeto completo con símbolos proporciona máxima entropía. Mínimo {0} letras para seguridad fuerte.',
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
		formatNotice:
			'Todas las claves API se generan con el prefijo "ak_" para fácil identificación. La longitud especificada se refiere solo a las letras aleatorias generadas (prefijo no contado).',
		securityNotice:
			'Almacena las claves API de forma segura y nunca las expongas en código del lado del cliente o control de versiones. Trátalas con el mismo cuidado que las contraseñas.',
		formatPrefix: 'prefijo ak_ +',
		randomCharacters: 'letras aleatorias con',
		noLookAlikeAlphabet: 'alfabeto sin confusión (fácil de escribir)',
		fullAlphanumericAlphabet: 'alfabeto alfanumérico completo',
		failedToGenerateApiKey: 'Error al generar clave API'
	},
	alphabets: {
		base58: 'Base58 (alfabeto Bitcoin)',
		'no-look-alike': 'Sin Confusión',
		full: 'Alfanumérico Completo',
		'full-with-symbols': 'Completo con Símbolos'
	}
};
