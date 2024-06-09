import plugin from 'tailwindcss/plugin';
import { BG_OPACITY, TEXT_OPACITY, rgbBg, rgbText } from '../utils';

export const baseStylesPlugin = plugin(({ addBase, theme }) => {
  addBase({
    '*': {
      // Disable auto-replacements of chars' sequences (e.g. 0x1 -> 0×1)
      fontVariantLigatures: 'no-contextual',
    },
    'html': {
      fontFamily: theme('fontFamily.mono'),
      fontWeight: theme('fontWeight.normal'),
      scrollBehavior: 'smooth',
    },

    // typography
    'h1': { fontSize: '2.5rem', lineHeight: '1.15' },
    'h2': { fontSize: '2rem', lineHeight: '1.15' },
    'h3': { fontSize: '1.625rem', lineHeight: '1.15' },
    'h4': { fontSize: '1.25rem', lineHeight: '1.15' },
    'h5': { fontSize: '1.125rem', lineHeight: '1.15' },
    'h6': { fontSize: '1rem', lineHeight: '1.15' },

    'code': {
      [BG_OPACITY]: '1',
      [TEXT_OPACITY]: '1',
      color: rgbText('--main-200'),
      backgroundColor: rgbBg('--main-700'),
      borderRadius: theme('borderRadius.md'),
      padding: '0.25ch 0.5ch 0.0625ch 0.5ch',
    },
  });
});
