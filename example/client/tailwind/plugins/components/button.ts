import plugin from 'tailwindcss/plugin';
import { BG_OPACITY, BORDER_OPACITY, TEXT_OPACITY, rgbBg, rgbText } from '../../utils';

export const buttonPlugin = plugin(({ addComponents, theme }) => {
  addComponents({
    '.button-primary': {
      [TEXT_OPACITY]: '1',
      [BG_OPACITY]: '1',
      [BORDER_OPACITY]: '1',
      'cursor': 'pointer',
      'padding': '0.375em 0.5em',
      'display': 'flex',
      'gap': '0.5em',
      'height': '2.5em',
      'alignItems': 'center',
      'justifyContent': 'center',
      'lineHeight': '1',
      'color': rgbText('--main-1000'),
      'backgroundColor': rgbBg('--main-0'),
      'borderRadius': theme('borderRadius.DEFAULT'),
      'transitionDuration': theme('transitionDuration.md'),
      '&:hover': {
        backgroundColor: rgbBg('--main-100'),
      },
      '&:focus': {
        outline: 'none',
        textDecorationLine: 'underline',
        textUnderlineOffset: '2px',
      },
      '&:disabled': {
        cursor: 'default',
        color: rgbText('--main-800'),
        backgroundColor: rgbBg('--main-200'),
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
