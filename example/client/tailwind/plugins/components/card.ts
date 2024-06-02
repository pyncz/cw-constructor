import plugin from 'tailwindcss/plugin';
import { BG_OPACITY, rgbBg } from '../../utils';

export const cardPlugin = plugin(({ addComponents, theme }) => {
  addComponents({
    '.card': {
      [BG_OPACITY]: '1',
      'padding': theme('padding.4'),
      'width': theme('width.36'),
      'minHeight': theme('height.60'),
      'transitionDuration': theme('transitionDuration.md'),
      'flexShrink': '0',
      '&:hover': {
        backgroundColor: rgbBg('--main-900'),
      },
    },
  });
});
