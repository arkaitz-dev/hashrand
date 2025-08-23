import type { I18nTexts } from '$lib/types';

export const de: I18nTexts = {
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
		clickToSelect:
			'Klicken Sie auf das Textfeld, um alles auszuwählen, oder verwenden Sie die Schaltfläche Kopieren',
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
		securityDescription:
			'Passwörter werden mit kryptographisch sicherer Zufallsgenerierung erzeugt. Sie werden nirgends gespeichert oder protokolliert.',
		noLookAlikeNote:
			'Das Alphabet ohne Verwechslung schließt verwechselbare Zeichen aus. Mindestens {0} Zeichen für gleichwertige Sicherheit.',
		fullAlphabetNote:
			'Das vollständige Alphabet mit Symbolen bietet maximale Entropie. Mindestens {0} Zeichen für starke Sicherheit.',
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
		formatNotice:
			'Alle API-Schlüssel werden mit dem Präfix "ak_" zur leichten Identifizierung generiert. Die angegebene Länge bezieht sich nur auf die generierten Zufallszeichen (Präfix nicht mitgezählt).',
		securityNotice:
			'Speichern Sie API-Schlüssel sicher und setzen Sie sie niemals in clientseitigem Code oder der Versionskontrolle frei. Behandeln Sie sie mit derselben Sorgfalt wie Passwörter.',
		formatPrefix: 'ak_-Präfix +',
		randomCharacters: 'Zufallszeichen mit',
		noLookAlikeAlphabet: 'Alphabet ohne Verwechslung (leicht zu tippen)',
		fullAlphanumericAlphabet: 'vollständiges alphanumerisches Alphabet',
		failedToGenerateApiKey: 'API-Schlüssel-Generierung fehlgeschlagen'
	},
	alphabets: {
		base58: 'Base58 (Bitcoin-Alphabet)',
		'no-look-alike': 'Ohne Verwechslung',
		full: 'Vollständig Alphanumerisch',
		'full-with-symbols': 'Vollständig mit Symbolen'
	}
};
