module.exports = {
  root: true,
  env: {
    node: true,
  },
  extends: [
    'plugin:@typescript-eslint/recommended',
    '@antfu',
  ],
  plugins: [],
  rules: {
    'import/no-anonymous-default-export': 'off',
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        ignoreRestSiblings: true,
      },
    ],
    'eslint-comments/no-unlimited-disable': 'off',
    '@typescript-eslint/brace-style': 'off',
    '@typescript-eslint/consistent-type-imports': 'warn',
    '@typescript-eslint/space-before-function-paren': 'off',
    '@typescript-eslint/array-type': [
      'error',
      {
        default: 'array',
      },
    ],
    'object-property-newline': ['error', { allowAllPropertiesOnSameLine: true }],
    'comma-dangle': ['error', 'always-multiline'],
    'camelcase': 'off',
    'import/named': 'off',
    'no-useless-constructor': 'off',
    'no-control-regex': 'off',
    'no-console': 'warn',
    'arrow-parens': ['error', 'always'],
    'brace-style': [
      'error',
      '1tbs',
    ],
    'curly': [
      'error',
      'all',
    ],
    'space-before-function-paren': [
      'error',
      {
        anonymous: 'never',
        named: 'never',
        asyncArrow: 'always',
      },
    ],
  },
}
