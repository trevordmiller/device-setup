const black = 'rgba(43, 48, 59, 1)';
const red = '#bf616a';
const green = '#a3be8c';
const yellow = '#ebcb8b';
const blue = '#8fa1b3';
const magenta = '#b48ead';
const cyan = '#96b5b4';
const white = 'rgba(239, 241, 245, 1)';
const white30 = 'rgba(239, 241, 245, 0.3)';
const white5 = 'rgba(239, 241, 245, 0.025)';
const padding = '17px';
const fontSize = 17;
const fontFamily = '"Fira Mono", monospace';
const cursorShape = 'BLOCK';
const css = `
  .tabs_nav {
    background: ${white5}
  }
`;

module.exports = {
  plugins: [],
  config: {
    padding,
    fontSize,
    fontFamily,
    cursorShape,
    cursorColor: white30,
    backgroundColor: black,
    foregroundColor: white,
    colors: {
      black,
      red,
      green,
      yellow,
      blue,
      magenta,
      cyan,
      white,
      lightBlack: black,
      lightRed: red,
      lightGreen: green,
      lightYellow: yellow,
      lightBlue: blue,
      lightMagenta: magenta,
      lightCyan: cyan,
      lightWhite: white,
    },
    css,
  },
};
