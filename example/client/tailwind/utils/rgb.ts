import * as o from './opacities';

/**
 * Return color by the applied opacity
 * that can be overriden by `[text|bg|border|fill]-opacity` utils
 * @see {@link https://tailwindcss.com/docs/customizing-colors#using-css-variables}
 */
export const rgb = (variable: string, alpha = '<alpha-value>') => {
  return `rgb(var(${variable}) / ${alpha})`;
};

export const rgbBg = (v: string) => rgb(v, `var(${o.BG_OPACITY})`);
export const rgbText = (v: string) => rgb(v, `var(${o.TEXT_OPACITY})`);
export const rgbBorder = (v: string) => rgb(v, `var(${o.BORDER_OPACITY})`);
export const rgbPlaceholder = (v: string) => rgb(v, `var(${o.PLACEHOLDER_OPACITY})`);
