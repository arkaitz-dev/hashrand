import type { I18nTexts } from '$lib/types';

export const de: I18nTexts = {
	common: {
		back: 'Zurück',
		generate: 'Generieren',
		copy: 'Kopieren',
		copied: 'Kopiert!',
		backToMenu: 'Zurück zum Menü',
		loading: 'Wird generiert...',
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
		generateAnother: 'Neuen generieren',
		adjustSettings: 'Einstellungen anpassen',
		navigateTo: 'Gehe zu',
		selectLanguage: 'Sprache auswählen',
		switchToLightMode: 'Zum hellen Modus wechseln',
		switchToDarkMode: 'Zum dunklen Modus wechseln',
		characters: 'Zeichen',
		alphabet: 'Alphabet',
		generatedValue: 'Generierter Wert',
		clickToSelect:
			'Klicken Sie auf das Textfeld, um alles auszuwählen, oder verwenden Sie den Kopieren-Button',
		waitGenerating: 'Bitte warten Sie, während ein neuer Wert generiert wird...',
		unknownEndpoint: 'Unbekannter Endpoint-Typ',
		failedToCopy: 'Kopieren fehlgeschlagen',
		fallbackCopyFailed: 'Ersatzkopie fehlgeschlagen',
		failedToRegenerate: 'Neugenerierung fehlgeschlagen',
		failedToLoadVersions: 'Laden der Versionen fehlgeschlagen',
		mustBeBetween: 'muss zwischen',
		and: 'und',
		cannotExceed: 'darf nicht überschreiten',
		optionalPrefix: 'Optionales Präfix',
		optionalSuffix: 'Optionales Suffix',
		prefixCannotExceed: 'Das Präfix darf 32 Zeichen nicht überschreiten',
		suffixCannotExceed: 'Das Suffix darf 32 Zeichen nicht überschreiten'
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
		generatePassword: 'Passwort generieren',
		length: 'Länge',
		alphabet: 'Zeichensatz',
		maxSecurityDescription: 'Maximale Sicherheit mit Symbolen (73 Zeichen)',
		easyReadDescription: 'Leicht zu lesen und einzugeben (49 Zeichen)',
		securityNote: 'Sicherheitshinweis:',
		securityDescription:
			'Passwörter werden mittels kryptographisch sicherer Zufallsgenerierung erstellt. Sie werden nirgendwo gespeichert oder protokolliert.',
		noLookAlikeNote:
			'Das eindeutige Alphabet schließt verwechselbare Zeichen aus. Mindestens {0} Zeichen für gleichwertige Sicherheit.',
		fullAlphabetNote:
			'Das vollständige Alphabet mit Symbolen bietet maximale Entropie. Mindestens {0} Zeichen für starke Sicherheit.',
		failedToGeneratePassword: 'Passwort-Generierung fehlgeschlagen'
	},
	apiKey: {
		title: 'API-Schlüssel-Generator',
		description: 'Generieren Sie API-Schlüssel mit ak_-Präfix',
		generateApiKey: 'API-Schlüssel generieren',
		length: 'Länge',
		alphabet: 'Zeichensatz',
		standardAlphanumericDescription: 'Standard-Alphanumerisch (62 Zeichen)',
		noConfusingDescription: 'Keine verwechselbaren Zeichen (49 Zeichen)',
		formatNotice:
			'Alle API-Schlüssel werden zur einfachen Identifizierung mit dem Präfix "ak_" generiert. Die angegebene Länge bezieht sich nur auf die generierten Zufallszeichen (Präfix nicht mitgezählt).',
		securityNotice:
			'Bewahren Sie API-Schlüssel sicher auf und geben Sie sie niemals in clientseitigem Code oder der Versionskontrolle preis. Behandeln Sie sie mit derselben Sorgfalt wie Passwörter.',
		formatPrefix: 'ak_-Präfix +',
		randomCharacters: 'Zufallszeichen mit',
		noLookAlikeAlphabet: 'eindeutigem Alphabet (leicht einzugeben)',
		fullAlphanumericAlphabet: 'vollständigem alphanumerischem Alphabet',
		fullAlphanumericNote: 'Das vollständige alphanumerische Alphabet bietet maximale Kompatibilität. Mindestens {0} Zeichen für starke Sicherheit.',
		failedToGenerateApiKey: 'API-Schlüssel-Generierung fehlgeschlagen'
	},
	alphabets: {
		base58: 'Base58 (Bitcoin-Alphabet)',
		'no-look-alike': 'Eindeutig',
		full: 'Vollständig Alphanumerisch',
		'full-with-symbols': 'Vollständig mit Symbolen'
	}
};
