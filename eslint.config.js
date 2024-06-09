import antfu from '@antfu/eslint-config';

export default antfu({
  vue: true,
  rules: {
    'no-console': 'warn',
    'curly': ['error', 'all'],
    'camelcase': 'error',
    'no-useless-constructor': 'error',
    'no-control-regex': 'error',
    'antfu/top-level-function': 'off',
    'node/prefer-global/process': ['error', 'always'],

    'import/named': 'error',
    'import/no-anonymous-default-export': ['error', {
      allowCallExpression: true,
    }],

    'style/member-delimiter-style': 'error',
    'style/semi': ['error', 'always'],
    'style/quotes': ['error', 'single'],
    'style/indent': ['error', 2],
    'style/arrow-parens': ['error', 'always'],
    'style/brace-style': ['error', '1tbs'],
    'style/comma-dangle': ['error', 'always-multiline'],
    'style/object-property-newline': ['error', { allowAllPropertiesOnSameLine: true }],
    'style/space-before-function-paren': [
      'error',
      {
        anonymous: 'never',
        named: 'never',
        asyncArrow: 'always',
      },
    ],

    'ts/no-unused-vars': [
      'error',
      {
        argsIgnorePattern: '^_',
        destructuredArrayIgnorePattern: '^_',
        ignoreRestSiblings: true,
      },
    ],
    // 'ts/consistent-type-imports': 'error',
    'ts/array-type': [
      'error',
      {
        default: 'array',
      },
    ],

    'vue/component-name-in-template-casing': ['error', 'kebab-case'],
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
    'vue/component-tags-order': ['error', {
      order: ['script', 'template', 'style'],
    }],
  },
});
