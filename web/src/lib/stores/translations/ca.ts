import type { I18nTexts } from '$lib/types';

export const ca: I18nTexts = {
	common: {
		back: 'Enrere',
		generate: 'Generar',
		copy: 'Copiar',
		copied: 'Copiat!',
		backToMenu: 'Tornar al menú',
		loading: 'Generant...',
		error: "S'ha produït un error",
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
		characters: 'caràcters',
		alphabet: 'Alfabet',
		generatedValue: 'Valor generat',
		clickToSelect:
			"Feu clic a l'àrea de text per seleccionar-ho tot, o utilitzeu el botó de copiar",
		waitGenerating: 'Si us plau, espereu mentre es genera un nou valor...',
		unknownEndpoint: "Tipus d'endpoint desconegut",
		failedToCopy: 'Ha fallat la còpia',
		fallbackCopyFailed: 'Ha fallat la còpia de reserva',
		failedToRegenerate: 'Ha fallat la regeneració',
		failedToLoadVersions: 'Ha fallat carregar les versions',
		mustBeBetween: "ha d'estar entre",
		and: 'i',
		cannotExceed: 'no pot superar',
		optionalPrefix: 'Prefix opcional',
		optionalSuffix: 'Sufix opcional',
		prefixCannotExceed: 'El prefix no pot superar 32 caràcters',
		suffixCannotExceed: 'El sufix no pot superar 32 caràcters',
		seedUsed: 'Llavor Utilitzada',
		copySeed: 'Copiar Llavor',
		optionalSeed: 'Llavor opcional (64 caràcters hex)',
		seedInvalid: 'La llavor ha de tenir exactament 64 caràcters hexadecimals',
		reuseSeedTitle: 'Reutilitzar la mateixa llavor?',
		reuseSeedMessage:
			'Voleu reutilitzar la mateixa llavor per generar el mateix resultat, o preferiu generar una nova llavor aleatòria?',
		keepSameSeed: 'Mantenir la mateixa llavor',
		generateNewSeed: 'Generar nova llavor',
		seed: 'Llavor'
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
		alphabet: "Tipus d'alfabet",
		prefix: 'Prefix',
		suffix: 'Sufix',
		lengthMustBeBetween: "La longitud ha d'estar entre 2 i 128",
		bitcoinDescription: 'Alfabet Bitcoin, exclou caràcters confusos',
		maxReadabilityDescription: 'Màxima llegibilitat, 49 caràcters',
		completeAlphanumericDescription: 'Conjunt alfanumèric complet',
		maxEntropyDescription: 'Màxima entropia amb símbols',
		failedToGenerateHash: 'Ha fallat generar el hash'
	},
	password: {
		title: 'Generador de contrasenyes segures',
		description: 'Genereu contrasenyes segures',
		generatePassword: 'Generar Contrasenya',
		length: 'Longitud',
		alphabet: 'Conjunt de caràcters',
		maxSecurityDescription: 'Màxima seguretat amb símbols (73 caràcters)',
		easyReadDescription: 'Fàcil de llegir i escriure (49 caràcters)',
		securityNote: 'Nota de seguretat:',
		securityDescription:
			"Les contrasenyes es generen utilitzant generació aleatòria criptogràficament segura. No s'emmagatzemen ni es registren enlloc.",
		noLookAlikeNote:
			"L'alfabet sense confusió exclou caràcters confusos. Mínim {0} caràcters per seguretat equivalent.",
		fullAlphabetNote:
			"L'alfabet complet amb símbols proporciona màxima entropia. Mínim {0} caràcters per seguretat forta.",
		failedToGeneratePassword: 'Ha fallat generar la contrasenya'
	},
	apiKey: {
		title: 'Generador de claus API',
		description: 'Genereu claus API amb prefix ak_',
		generateApiKey: 'Generar Clau API',
		length: 'Longitud',
		alphabet: 'Conjunt de caràcters',
		standardAlphanumericDescription: 'Alfanumèric estàndard (62 caràcters)',
		noConfusingDescription: 'Sense caràcters confusos (49 caràcters)',
		formatNotice:
			'Totes les claus API es generen amb el prefix "ak_" per facilitar la identificació. La longitud especificada es refereix només als caràcters aleatoris generats (prefix no comptat).',
		securityNotice:
			'Emmagatzemeu les claus API de forma segura i mai les exposeu en codi del costat del client o control de versions. Tracteu-les amb la mateixa cura que les contrasenyes.',
		formatPrefix: 'prefix ak_ +',
		randomCharacters: 'caràcters aleatoris amb',
		noLookAlikeAlphabet: "alfabet sense confusió (fàcil d'escriure)",
		fullAlphanumericAlphabet: 'alfabet alfanumèric complet',
		fullAlphanumericNote:
			"L'alfabet alfanumèric complet proporciona màxima compatibilitat. Mínim {0} caràcters per a seguretat forta.",
		failedToGenerateApiKey: 'Ha fallat generar la clau API'
	},
	alphabets: {
		base58: 'Base58 (alfabet Bitcoin)',
		'no-look-alike': 'Sense confusió',
		full: 'Alfanumèric complet',
		'full-with-symbols': 'Complet amb símbols'
	}
};
