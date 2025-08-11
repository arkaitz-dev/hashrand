// Localization configuration in a separate file to avoid circular dependencies
import { configureLocalization } from '@lit/localize';
import { sourceLocale, targetLocales } from './locales/locale-codes.js';

export const { getLocale, setLocale } = configureLocalization({
  sourceLocale,
  targetLocales,
  loadLocale: (locale) => import(`./locales/${locale}.js`),
});