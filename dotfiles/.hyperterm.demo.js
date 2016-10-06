module.exports = {
  plugins: [
    'hyperterm-one-light'
  ],
  config: {
    fontFamily: '"Monaco", monospace',
    padding: '17px',
    fontSize: 30,
    cursorShape: 'BLOCK',
    windowSize: [1280, 720],
    css: `
      .tabs_nav {
        display: none;
      }
    `,
  },
}
