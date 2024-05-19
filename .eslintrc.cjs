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
    '@typescript-eslint/semi': 'off',
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
    'semi': ['error', 'always'],
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
    'vue/html-self-closing': ['error', {
      html: {
        normal: 'always',
        void: 'always',
        component: 'always',
      },
    }],
    'vue/max-attributes-per-line': ['error', {
      singleline: {
        max: 3,
      },
      multiline: {
        max: 1,
      },
    }],
    'vue/custom-event-name-casing': ['error', 'kebab-case'],
    'vue/component-name-in-template-casing': ['error', 'kebab-case'],
    'vue/component-tags-order': ['error', {
      order: ['script', 'template', 'style'],
    }],
  },
};
