import type { I18nTexts } from '$lib/types';

export const eu: I18nTexts = {
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
		mustBeBetween: 'artean egon behar du',
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
		securityDescription:
			'Pasahitzak kriptografikoki segurua den ausazko sorkuntzaren bidez sortzen dira. Ez dira inon gordetzen edo erregistratzen.',
		noLookAlikeNote:
			'Hizki nahasgarririk gabeko alfabetoak hizki antzekoak kanpoan uzten ditu. Segurtasun baliokiderako gutxienez {0} hizki behar dira.',
		fullAlphabetNote:
			'Sinboloak dituen alfabeto osoak entropia handiena ematen du. Segurtasun sendorako gutxienez {0} hizki behar dira.',
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
		formatNotice:
			'API gako guztiak "ak_" aurrizkiarekin sortzen dira identifikazioa errazagoa izateko. Zehaztutako luzerak soilik sortutako ausazko hizkiak hartzen ditu kontuan (aurrizkirik gabe).',
		securityNotice:
			'API gakoak modu seguruan gorde eta inoiz ez jarri bezero-aldeko kodean edo bertsio-kontrolean. Pasahitzekin bezalako arretaz tratatu.',
		formatPrefix: 'ak_ aurrizkia +',
		randomCharacters: 'ausazko hizki hauek erabiliz',
		noLookAlikeAlphabet: 'hizki nahasgarririk gabeko alfabetoa (erraz idazteko)',
		fullAlphanumericAlphabet: 'alfabeto alfanumeriko osoa',
		fullAlphanumericNote: 'Alfabeto alfanumeriko osoak bateragarritasun maximoa eskaintzen du. Segurtasun sendorako gutxienez {0} hizki behar.',
		failedToGenerateApiKey: 'API gakoa sortzeak huts egin du'
	},
	alphabets: {
		base58: 'Base58 (Bitcoin alfabetoa)',
		'no-look-alike': 'Nahasgarririk gabe',
		full: 'Alfabeto Alfanumeriko Osoa',
		'full-with-symbols': 'Osoa Sinboloekin'
	}
};
