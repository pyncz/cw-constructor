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
    plugin(({ addBase }) => {
      addBase({
        ':root': {
          '--container-padding': '1.25rem',
        },
      });
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
