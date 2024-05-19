import plugin from 'tailwindcss/plugin';
import { rgbText } from '../../utils';

export const linkPlugin = plugin(({ addComponents, theme }) => {
  addComponents({
    '.link': {
      'color': rgbText('--main-200'),
      'transitionDuration': theme('transitionDuration.sm'),
      'textDecorationLine': 'underline',
      'textDecorationStyle': 'dashed',
      'textUnderlineOffset': '4px',
      '&:hover': {
        color: rgbText('--main-100'),
      },
      '&:focus': {
        outline: 'none',
        color: rgbText('--main-0'),
        textDecorationStyle: 'solid',
      },
      '&[target="_blank"]': {
        '&::after': {
          fontSize: '0.8em',
          paddingLeft: '0.35ch',
          content: '"↗"',
        },
      },
    },
  });
});