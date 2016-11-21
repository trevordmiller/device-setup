module.exports = {
  'extends': [
    'eslint:recommended',
    'plugin:react/recommended',
  ],
  'plugins': ['react'],
  'env': {
    'es6': true,
    'node': true,
    'commonjs': true,
    'browser': true,
    'jest': true,
  },
  'parserOptions': {
    'ecmaVersion': 6,
    'sourceType': 'module',
    'ecmaFeatures': {
      'experimentalObjectRestSpread': true,
      'jsx': true,
    },
  },
  'rules': {
    'linebreak-style': ['error', 'unix'],
    'indent': ['error', 2],
    'quotes': ['error', 'single'],
    'semi': ['error', 'never'],
    'comma-dangle': ['error', 'always-multiline'],
    'no-console': ['off'],
    'react/display-name': ['off'],
  },
}
