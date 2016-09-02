const nova = {
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
    cursorColor: nova.bright.white,
    foregroundColor: nova.normal.white,
    backgroundColor: nova.normal.black,
    borderColor: nova.normal.black,
    colors: {
      black: nova.normal.black,
      red: nova.normal.red,
      green: nova.normal.green,
      yellow: nova.normal.yellow,
      blue: nova.normal.blue,
      magenta: nova.normal.magenta,
      cyan: nova.normal.cyan,
      white: nova.normal.white,
      lightBlack: nova.bright.black,
      lightRed: nova.bright.red,
      lightGreen: nova.normal.green,
      lightYellow: nova.normal.yellow,
      lightBlue: nova.normal.blue,
      lightMagenta: nova.bright.magenta,
      lightCyan: nova.normal.cyan,
      lightWhite: nova.bright.white,
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
        background-color: ${nova.decoration.medium} !important;
      }
      .tab_tab.tab_active {
        background-color: ${nova.normal.black} !important;
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
