module.exports = {
  root: true,
  env: {
    node: true
  },
  extends: [
    'plugin:vue/vue3-essential',
    '@vue/standard'
  ],
  parserOptions: {
    parser: '@babel/eslint-parser',
    // parser: '@typescript-eslint/parser',
    // parser: 'vue-eslint-parser',
    requireConfigFile: false
  },
  // parser: 'vue-eslint-parser',
  rules: {
    'no-console': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'no-debugger': process.env.NODE_ENV === 'production' ? 'warn' : 'off',
    'comma-dangle': 0,
    'no-trailing-spaces': 0,
    'eol-last': 0,
    'space-before-function-paren': 0,
    indent: 0,
    semi: 0,
    'no-unused-vars': 0
  }
}
