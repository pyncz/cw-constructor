import plugin from 'tailwindcss/plugin';
import { BORDER_OPACITY, TEXT_OPACITY, rgbBorder, rgbText } from '../../utils';

export const placeholderPlugin = plugin(({ addComponents }) => {
  addComponents({
    '.placeholder': {
      [TEXT_OPACITY]: '1',
      [BORDER_OPACITY]: '1',
      borderWidth: '1px',
      borderStyle: 'solid',
      borderColor: rgbBorder('--main-100'),
      color: rgbText('--main-200'),
      padding: '2em',
      textAlign: 'center',
    },
  });
});
