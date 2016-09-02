const echo = {
  normal: {
    black: '#445660',
    red: '#DF8C8C',
    green: '#A8CE93',
    yellow: '#DADA93',
    blue: '#83AFE5',
    magenta: '#9A93E1',
    cyan: '#7FC1CA',
    white: '#C5D4DD',
  },
  bright: {
    black: '#899BA6',
    red: '#F2C38F',
    magenta: '#D18EC2',
    white: '#E6EEF3',
  },
  decoration: {
    dark: '#1E272C',
    medium: '#556873',
    light: '#6A7D89',
  },
};

const fontFamily = '"Fira Mono", monospace';

module.exports = {
  plugins: [
  ],
  config: {
    fontFamily,
    padding: '17px',
    fontSize: 17,
    cursorShape: 'BLOCK',
    cursorColor: echo.bright.white,
    foregroundColor: echo.normal.white,
    backgroundColor: echo.normal.black,
    borderColor: echo.normal.black,
    colors: {
      black: echo.normal.black,
      red: echo.normal.red,
      green: echo.normal.green,
      yellow: echo.normal.yellow,
      blue: echo.normal.blue,
      magenta: echo.normal.magenta,
      cyan: echo.normal.cyan,
      white: echo.normal.white,
      lightBlack: echo.bright.black,
      lightRed: echo.bright.red,
      lightGreen: echo.normal.green,
      lightYellow: echo.normal.yellow,
      lightBlue: echo.normal.blue,
      lightMagenta: echo.bright.magenta,
      lightCyan: echo.normal.cyan,
      lightWhite: echo.bright.white,
    },
    termCSS: `
      .cursor-node {
        opacity: 0.5 !important;
      }
    `,
    css: `
      .tab_first {
        margin-left: 0 !important;
      }
      .tab_tab {
        background-color: ${echo.decoration.medium} !important;
      }
      .tab_tab.tab_active {
        background-color: ${echo.normal.black} !important;
        border: none !important;
      }
      .tab_tab.tab_active::before {
        border: none !important;
      }
      .tab_text {
        border: none !important;
      }
      .tabs_title,
      .tab_textInner {
        font-size: 14px !important;
        font-weight: bold !important;
      }
    `,
  },
};
