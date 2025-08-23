import type { I18nTexts } from '$lib/types';

export const ja: I18nTexts = {
	common: {
		back: '戻る',
		generate: '生成',
		copy: 'コピー',
		copied: 'コピーしました！',
		backToMenu: 'メニューに戻る',
		loading: '生成中...',
		error: 'エラーが発生しました',
		result: '結果',
		choose: '選択',
		type: 'タイプ',
		length: '文字数',
		generated: '生成済み',
		format: 'フォーマット',
		security: 'セキュリティ',
		loadingVersion: 'バージョン読み込み中...',
		versionsUnavailable: 'バージョン情報が利用できません',
		generationDetails: '生成詳細',
		parametersUsed: '使用したパラメータ',
		generateAnother: 'もう一度生成',
		adjustSettings: '設定を調整',
		navigateTo: '移動する',
		selectLanguage: '言語を選択',
		switchToLightMode: 'ライトモードに切り替え',
		switchToDarkMode: 'ダークモードに切り替え',
		characters: '文字',
		alphabet: 'アルファベット',
		generatedValue: '生成された値',
		clickToSelect: 'テキストエリアをクリックして全選択するか、コピーボタンをお使いください',
		waitGenerating: '新しい値を生成中です。しばらくお待ちください...',
		unknownEndpoint: '不明なエンドポイントタイプ',
		failedToCopy: 'コピーに失敗しました',
		fallbackCopyFailed: 'フォールバックコピーに失敗しました',
		failedToRegenerate: '再生成に失敗しました',
		failedToLoadVersions: 'バージョン読み込みに失敗しました',
		mustBeBetween: 'は次の範囲である必要があります：',
		and: 'から',
		cannotExceed: 'を超えることはできません',
		optionalPrefix: 'プレフィックス（オプション）',
		optionalSuffix: 'サフィックス（オプション）',
		prefixCannotExceed: 'プレフィックスは32文字を超えることはできません',
		suffixCannotExceed: 'サフィックスは32文字を超えることはできません',
		yes: 'はい',
		no: 'いいえ'
	},
	menu: {
		title: 'ハッシュ生成器',
		subtitle: '生成方法を選択してください',
		version: 'バージョン',
		brandName: 'HashRand Spin',
		description: '暗号学的に安全なハッシュ、パスワード、APIキーの生成器'
	},
	custom: {
		title: 'カスタムハッシュ生成器',
		description: 'カスタムランダムハッシュを生成',
		generateHash: 'ハッシュを生成',
		length: '文字数',
		alphabet: 'アルファベットタイプ',
		prefix: 'プレフィックス',
		suffix: 'サフィックス',
		lengthMustBeBetween: '文字数は2から128の間である必要があります',
		bitcoinDescription: 'Bitcoinアルファベット、紛らわしい文字を除外',
		maxReadabilityDescription: '最大の可読性、49文字',
		completeAlphanumericDescription: '完全な英数字セット',
		maxEntropyDescription: '記号を含む最大エントロピー',
		failedToGenerateHash: 'ハッシュの生成に失敗しました'
	},
	password: {
		title: '安全なパスワード生成器',
		description: '安全なパスワードを生成',
		generatePassword: 'パスワードを生成',
		length: '文字数',
		alphabet: '文字セット',
		maxSecurityDescription: '記号を含む最大セキュリティ（73文字）',
		easyReadDescription: '読みやすく入力しやすい（49文字）',
		securityNote: 'セキュリティに関する注意：',
		securityDescription:
			'パスワードは暗号学的に安全な乱数生成を使用して作成されます。どこにも保存やログ記録されません。',
		noLookAlikeNote:
			'紛らわしくないアルファベットは混同しやすい文字を除外します。同等のセキュリティには最低{0}文字が必要です。',
		fullAlphabetNote:
			'記号を含む完全なアルファベットは最大エントロピーを提供します。強力なセキュリティには最低{0}文字が必要です。',
		failedToGeneratePassword: 'パスワードの生成に失敗しました'
	},
	apiKey: {
		title: 'APIキー生成器',
		description: 'ak_プレフィックス付きAPIキーを生成',
		generateApiKey: 'APIキーを生成',
		length: '文字数',
		alphabet: '文字セット',
		standardAlphanumericDescription: '標準英数字（62文字）',
		noConfusingDescription: '紛らわしい文字なし（49文字）',
		formatNotice:
			'すべてのAPIキーは識別しやすくするために「ak_」プレフィックスが付加されます。指定した文字数は生成されるランダム文字のみを指します（プレフィックスは含みません）。',
		securityNotice:
			'APIキーは安全に保存し、クライアント側コードやバージョン管理システムに絶対に公開しないでください。パスワードと同じように慎重に扱ってください。',
		formatPrefix: 'ak_プレフィックス +',
		randomCharacters: 'ランダム文字を使用：',
		noLookAlikeAlphabet: '紛らわしくないアルファベット（入力しやすい）',
		fullAlphanumericAlphabet: '完全英数字アルファベット',
		fullAlphanumericNote: '完全英数字アルファベットは最大の互換性を提供します。強力なセキュリティには最低{0}文字が必要です。',
		failedToGenerateApiKey: 'APIキーの生成に失敗しました'
	},
	alphabets: {
		base58: 'Base58（ビットコインアルファベット）',
		'no-look-alike': '紛らわしくない',
		full: '完全英数字',
		'full-with-symbols': '記号を含む完全'
	}
};
