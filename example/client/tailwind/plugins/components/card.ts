import plugin from 'tailwindcss/plugin';
import { BG_OPACITY, BORDER_OPACITY, rgbBg, rgbBorder } from '../../utils';

export const cardPlugin = plugin(({ addComponents, theme }) => {
  addComponents({
    '.card': {
      [BG_OPACITY]: '1',
      [BORDER_OPACITY]: '0',
      'padding': theme('padding.4'),
      'width': theme('width.36'),
      'transitionDuration': theme('transitionDuration.md'),
      'borderWidth': '1px',
      'borderStyle': 'dashed',
      'borderColor': rgbBorder('--main-400'),
      'flexShrink': '0',
      '&:hover': {
        backgroundColor: rgbBg('--main-900'),
      },
      '&:focus-within': {
        [BORDER_OPACITY]: '1',
      },
    },
  });
});
