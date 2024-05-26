import plugin from 'tailwindcss/plugin';
import { rgb } from '../utils';

export const palettePlugin = plugin(({ addBase }) => {
  addBase({
    'html': {
      colorScheme: 'dark',
      color: 'rgb(var(--main-0))',
      backgroundColor: 'rgb(var(--main-1000))',
    },
    ':root': {
      // (r g b) channels
      '--black': '0 0 0',
      '--white': '255 255 255',

      // text colors
      '--main-0': '255 255 255',
      '--main-100': '194 194 194',
      '--main-200': '106 106 106',

      // bg colors
      '--main-800': '22 21 28',
      '--main-900': '16 16 21',
      '--main-1000': '11 10 16',

      // accent colors for traits' stats etc
      '--accent-1': '255 104 140',
      '--accent-2': '67 186 100',
      '--accent-3': '123 112 255',

      '--accents-opacity': '0.1',
    },
    '::selection': {
      color: 'rgb(var(--black))',
      backgroundColor: 'rgb(var(--white))',
    },
  });
}, {
  theme: {
    colors: {
      black: rgb('--black'),
      white: rgb('--white'),
      main: {
        0: rgb('--main-0'),
        100: rgb('--main-100'),
        200: rgb('--main-200'),
        // ...
        800: rgb('--main-800'),
        900: rgb('--main-900'),
        1000: rgb('--main-1000'),

      },
      accent: {
        1: rgb('--accent-1'),
        2: rgb('--accent-2'),
        3: rgb('--accent-3'),
      },
    },
    opacity: {
      accents: 'var(--accents-opacity)',
    },
  },
});
