import plugin from 'tailwindcss/plugin';
import { defu } from 'defu';
import type { CSSRuleObject } from 'tailwindcss/types/config';
import { BORDER_OPACITY, rgbBorder } from '../../utils';

export const framePlugin = plugin(({ addComponents, theme }) => {
  const overlay: CSSRuleObject = {
    content: 'var(--tw-content)',
    position: 'absolute',
    top: '-1px',
    bottom: '-1px',
    width: 'var(--b)',
    pointerEvents: 'none',
    borderTopWidth: '1px',
    borderBottomWidth: '1px',
    borderStyle: 'solid',
    borderColor: rgbBorder('--main-100'),
    maskImage: 'linear-gradient(0deg, rgb(0 0 0) var(--b), rgb(0 0 0 / 0) var(--b), rgb(0 0 0 / 0) calc(100% - var(--b)), rgb(0 0 0) calc(100% - var(--b)))',
  };

  addComponents({
    '.frame': {
      [BORDER_OPACITY]: '1',
      '--b': '1em',
      'position': 'relative',
      'transitionDuration': theme('transitionDuration.md'),
      'borderWidth': '1px',
      'borderStyle': 'solid',
      'borderColor': rgbBorder('--main-700'),
      '&:focus-within': {
        borderColor: rgbBorder('--main-400'),
      },
      '&:before': defu({
        left: '-1px',
        borderLeftWidth: '1px',
      }, overlay),
      '&:after': defu({
        right: '-1px',
        borderRightWidth: '1px',
      }, overlay),
    },
  });
});
