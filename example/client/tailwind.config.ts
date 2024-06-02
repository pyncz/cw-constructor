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
  plugins: [
    ...Object.values(plugins),
    plugin(({ addBase, matchUtilities, theme }) => {
      addBase({
        ':root': {
          '--container-padding': '1.25rem',
        },
      });

      addBase({
        h1: { fontSize: '2.5rem', lineHeight: '1.15' },
        h2: { fontSize: '2rem', lineHeight: '1.15' },
        h3: { fontSize: '1.625rem', lineHeight: '1.15' },
        h4: { fontSize: '1.25rem', lineHeight: '1.15' },
        h5: { fontSize: '1.125rem', lineHeight: '1.15' },
        h6: { fontSize: '1rem', lineHeight: '1.15' },
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
    container: {
      center: true,
      padding: {
        DEFAULT: 'var(--container-padding)',
        sm: '1.5rem',
        lg: '2rem',
      },
    },
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
      },
      spacing: {
        em: '1em',
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
      padding: {
        container: 'var(--container-padding)',
      },
    },
  },
} satisfies Config;
