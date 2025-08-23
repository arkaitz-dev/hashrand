import type { I18nTexts } from '$lib/types';

export const fr: I18nTexts = {
	common: {
		back: 'Retour',
		generate: 'Générer',
		copy: 'Copier',
		copied: 'Copié !',
		backToMenu: 'Retour au Menu',
		loading: 'Génération...',
		error: "Une erreur s'est produite",
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
		clickToSelect:
			'Cliquez sur la zone de texte pour tout sélectionner, ou utilisez le bouton copier',
		waitGenerating: "Veuillez patienter pendant la génération d'une nouvelle valeur...",
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
		suffixCannotExceed: 'Le suffixe ne peut pas dépasser 32 lettres',
		yes: 'Oui',
		no: 'Non'
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
		alphabet: "Type d'Alphabet",
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
		securityDescription:
			'Les mots de passe sont générés en utilisant une génération aléatoire cryptographiquement sécurisée. Ils ne sont stockés ni journalisés nulle part.',
		noLookAlikeNote:
			"L'alphabet Sans Ambiguïté exclut les lettres confuses. Minimum {0} lettres pour une sécurité équivalente.",
		fullAlphabetNote:
			"L'alphabet complet avec symboles fournit une entropie maximale. Minimum {0} lettres pour une sécurité forte.",
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
		formatNotice:
			'Toutes les clés API sont générées avec le préfixe "ak_" pour une identification facile. La longueur spécifiée ne concerne que les caractères aléatoires générés (préfixe non compté).',
		securityNotice:
			'Stockez les clés API en sécurité et ne les exposez jamais dans le code côté client ou le contrôle de version. Traitez-les avec le même soin que les mots de passe.',
		formatPrefix: 'préfixe ak_ +',
		randomCharacters: 'lettres aléatoires avec',
		noLookAlikeAlphabet: 'alphabet sans ambiguïté (facile à taper)',
		fullAlphanumericAlphabet: 'alphabet alphanumérique complet',
		failedToGenerateApiKey: 'Échec de la génération de la clé API'
	},
	alphabets: {
		base58: 'Base58 (alphabet Bitcoin)',
		'no-look-alike': 'Sans Ambiguïté',
		full: 'Alphanumérique Complet',
		'full-with-symbols': 'Complet avec Symboles'
	}
};
