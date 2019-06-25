const COLORS = [
  'black',
  'brown',
  'red',
  'orange',
  'yellow',
  'green',
  'blue',
  'violet',
  'grey',
  'white'
];

const colorCode = color => COLORS.indexOf(color);

export const value = colors =>
  colors.reduce((duo, color) => {
    return duo * 10 + colorCode(color);
  }, 0);
