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

      '--main-300': '87 87 88',
      '--main-400': '70 70 72',
      '--main-500': '55 55 58',
      '--main-600': '42 42 46',
      '--main-700': '31 31 36',

      // bg colors
      '--main-800': '24 23 31',
      '--main-900': '18 18 23',
      '--main-1000': '11 10 16',

      // accent colors for traits' stats etc
      '--accent-1': '255 104 140',
      '--accent-2': '67 186 100',
      '--accent-3': '123 112 255',

      '--accents-opacity': '0.2',
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

      // 0-1000
      main: Array(11).fill(null).reduce((scale, _, i) => {
        const index = i * 100;
        scale[index] = rgb(`--main-${index}`);
        return scale;
      }, {} as Record<number, string>),

      // 1-3
      accent: Array(3).fill(null).reduce((scale, _, i) => {
        const index = i + 1;
        scale[index] = rgb(`--accent-${index}`);
        return scale;
      }, {} as Record<number, string>),
    },
    opacity: {
      accents: 'var(--accents-opacity)',
    },
  },
});
