const black = '#36393B';
const red = '#FF7C7A';
const green = '#8FD5A6';
const yellow = '#FFE57F';
const blue = '#378AAD';
const magenta = '#8C82AF';
const cyan = '#8CCEC6';
const white = '#F4F1DE';
const lightBlack = '#817F823';
const lightWhite = '#FFFFFA';

module.exports = {
  plugins: [
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
