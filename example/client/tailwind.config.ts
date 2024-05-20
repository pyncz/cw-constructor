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
        h1: { fontSize: '2.5rem' },
        h2: { fontSize: '2rem' },
        h3: { fontSize: '1.625rem' },
        h4: { fontSize: '1.25rem' },
        h5: { fontSize: '1.125rem' },
        h6: { fontSize: '1rem' },
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
    borderRadius: {
      DEFAULT: '1px',
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
      padding: {
        container: 'var(--container-padding)',
      },
    },
  },
} satisfies Config;
