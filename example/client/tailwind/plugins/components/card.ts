import plugin from 'tailwindcss/plugin';
import { BG_OPACITY, rgbBg } from '../../utils';

export const cardPlugin = plugin(({ addComponents, theme }) => {
  addComponents({
    '.card': {
      [BG_OPACITY]: '1',
      'padding': theme('padding.4'),
      'width': theme('width.36'),
      'minHeight': theme('height.60'), // 48
      'backgroundColor': rgbBg('--main-900'),
      'transitionDuration': theme('transitionDuration.md'),
      '&:hover': {
        backgroundColor: rgbBg('--main-800'),
      },
    },
  });
});
