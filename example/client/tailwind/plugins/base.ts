import plugin from 'tailwindcss/plugin';

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
  });
});
