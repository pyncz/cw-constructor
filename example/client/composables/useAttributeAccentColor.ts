import { NumericAttribute } from '~/types';

export const useAttributeAccentColor = (traitType: MaybeRefOrGetter<string | undefined>) => {
  const variable = computed(() => {
    switch (toValue(traitType)) {
      case NumericAttribute.RAGE:
        return 'var(--accent-1)';
      case NumericAttribute.DECEPTION:
        return 'var(--accent-2)';
      case NumericAttribute.PSYCHIC:
        return 'var(--accent-3)';
      default:
        return 'var(--main-100)';
    }
  });

  const rgb = computed(() => `rgb(${variable.value})`);

  return {
    variable,
    rgb,
  };
};
