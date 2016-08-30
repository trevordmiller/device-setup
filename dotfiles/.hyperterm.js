const black = '#2A2E35';
const red = '#FF7C7A';
const green = '#8FD5A6';
const yellow = '#FFE57F';
const blue = '#378AAD';
const magenta = '#8C82AF';
const cyan = '#8CCEC6';
const white = '#EEEBD0';
const lightBlack = '#3C424C';
const lightWhite = '#FFFFFA';

module.exports = {
  plugins: [
    // 'hyperterm-atom-dark',
  ],
  config: {
    padding: '17px',
    fontSize: 17,
    fontFamily: '"Fira Mono", monospace',
    cursorShape: 'BLOCK',
    cursorColor: white,
    foregroundColor: lightWhite,
    backgroundColor: black,
    borderColor: lightBlack,
    colors: {
      black,
      red,
      green,
      yellow,
      blue,
      magenta,
      cyan,
      white,
      lightBlack,
      lightRed: red,
      lightGreen: green,
      lightYellow: yellow,
      lightBlue: blue,
      lightMagenta: magenta,
      lightCyan: cyan,
      lightWhite,
    },
  },
};
