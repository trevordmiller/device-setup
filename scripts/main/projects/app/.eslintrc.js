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
      'jsx': true,
    },
  },
  'rules': {
    'react/display-name': ['off'],
  },
}
