import { defu } from 'defu';
import plugin from 'tailwindcss/plugin';
import type { CSSRuleObject } from 'tailwindcss/types/config';
import { BG_OPACITY, BORDER_OPACITY, TEXT_OPACITY, rgbBg, rgbBorder, rgbText } from '../../utils';

export const buttonPlugin = plugin(({ addComponents, theme }) => {
  // Base components
  const baseButton: CSSRuleObject = {
    [TEXT_OPACITY]: '1',
    [BG_OPACITY]: '1',
    [BORDER_OPACITY]: '0',
    'cursor': 'pointer',
    'display': 'flex',
    'gap': '0.5em',
    'alignItems': 'center',
    'justifyContent': 'center',
    'lineHeight': '1',
    'transitionDuration': theme('transitionDuration.md'),
    'borderRadius': theme('borderRadius.DEFAULT'),
    'borderStyle': 'solid',
    'borderWidth': '1px',
    '&:focus': {
      outline: 'none',
      textDecorationLine: 'underline',
      textUnderlineOffset: '2px',
    },
    '&:disabled': {
      cursor: 'default',
    },
    '&[target="_blank"]': {
      '&::after': {
        fontSize: '0.8em',
        paddingLeft: '0.35ch',
        content: '"↗"',
      },
    },
  };

  const mainButton: CSSRuleObject = defu({
    padding: '0.375em 0.5em',
    height: '2.5em',
  }, baseButton);

  const inlineButton: CSSRuleObject = defu({
    padding: '0.25em 0.375em 0.125em 0.375em',
  }, baseButton);

  addComponents({
    '.button': baseButton,
  });

  // Add primary buttons
  const primaryButtonColors: CSSRuleObject = {
    'color': rgbText('--main-1000'),
    'backgroundColor': rgbBg('--main-0'),
    'borderColor': rgbBorder('--main-0'),
    '&:hover': {
      backgroundColor: rgbBg('--main-100'),
    },
    '&:disabled': {
      color: rgbText('--main-800'),
      backgroundColor: rgbBg('--main-200'),
    },
  };
  addComponents({
    '.button-primary': defu(primaryButtonColors, mainButton),
    '.button-inline-primary': defu(primaryButtonColors, inlineButton),
  });

  // Add secondary buttons
  const secondaryButtonColors: CSSRuleObject = {
    'color': rgbText('--main-300'),
    'backgroundColor': rgbBg('--main-800'),
    'borderColor': rgbBorder('--main-800'),
    '&:hover': {
      backgroundColor: rgbBg('--main-700'),
    },
    '&:focus': {
      color: rgbText('--main-200'),
    },
    '&:disabled': {
      color: rgbText('--main-400'),
      backgroundColor: `${rgbBg('--main-800')} !important`,
    },
  };
  addComponents({
    '.button-secondary': defu(secondaryButtonColors, mainButton),
    '.button-inline-secondary': defu(secondaryButtonColors, inlineButton),
  });
});
