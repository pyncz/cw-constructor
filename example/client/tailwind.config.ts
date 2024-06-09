import type { Config } from 'tailwindcss';
import defaultTheme from 'tailwindcss/defaultTheme';
import plugin from 'tailwindcss/plugin';
import * as plugins from './tailwind/plugins';

export default {
  content: [
    './components/**/*.{js,vue,ts}',
    './layouts/**/*.vue',
    './pages/**/*.vue',
    './plugins/**/*.{js,ts}',
    './app.vue',
    './error.vue',
  ],
  corePlugins: {
    container: false,
  },
  plugins: [
    ...Object.values(plugins),
    plugin(({ addBase, matchUtilities, addComponents, theme }) => {
      addBase({
        ':root': {
          '--container-padding': '1.25rem',
          '@screen sm': {
            '--container-padding': '1.5rem',
          },
          '@screen lg': {
            '--container-padding': '2rem',
          },
        },
      });

      addComponents({
        '.container': {
          width: '100%',
          marginLeft: 'auto',
          marginRight: 'auto',
          paddingLeft: 'var(--container-padding)',
          paddingRight: 'var(--container-padding)',
          maxWidth: theme('screens.2xl'),
        },
      });

      matchUtilities(
        {
          'grid-cols-autofill': (value) => ({
            gridTemplateColumns: `repeat(auto-fill, minmax(${value}, 1fr))`,
          }),
          'grid-cols-autofit': (value) => ({
            gridTemplateColumns: `repeat(auto-fit, minmax(${value}, 1fr))`,
          }),
        },
        { values: theme('width') },
      );
    }),
  ],
  theme: {
    fontFamily: {
      sans: defaultTheme.fontFamily.sans,
      comic: ['"Coming Soon"', ...defaultTheme.fontFamily.sans],
      mono: ['"Overpass Mono"', ...defaultTheme.fontFamily.mono],
    },
    lineHeight: {
      1: '1',
      xs: '1.15',
      sm: '1.25',
      md: '1.375',
    },
    borderRadius: {
      DEFAULT: '1px',
      md: '2px',
      lg: '4px',
    },
    opacity: {
      0: '0',
      20: '0.2',
      50: '0.5',
      80: '0.8',
      100: '1',
    },
    transitionDuration: {
      xs: '100ms',
      sm: '200ms',
      md: '400ms',
      lg: '800ms',
    },
    extend: {
      screens: {
        xs: '480px',
      },
      colors: {
        current: 'currentColor',
        transparent: 'transparent',
      },
      spacing: {
        em: '1em',
        container: 'var(--container-padding)',
      },
      fontSize: {
        '1/2': '0.5em',
        '5/8': '0.625em',
        '3/4': '0.75em',
        '7/8': '0.875em',
        'em': '1em',
        '9/8': '1.125em',
        '5/4': '1.25em',
        '11/8': '1.375em',
        '3/2': '1.5em',
      },
    },
  },
} satisfies Config;
