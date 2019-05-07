export const toRoman = num => {
  let codex = new Map([
    [1, 'I'],
    [2, 'II'],
    [3, 'III'],
    [4, 'IV'],
    [5, 'V'],
    [6, 'VI'],
    [7, 'VII'],
    [8, 'VIII'],
    [9, 'IX'],
    [10, 'X'],
    [20, 'XX'],
    [30, 'XXX'],
    [40, 'XL'],
    [50, 'L'],
    [60, 'LX'],
    [70, 'LXX'],
    [80, 'LXXX'],
    [90, 'XC'],
    [100, 'C'],
    [200, 'CC'],
    [300, 'CCC'],
    [400, 'CD'],
    [500, 'D'],
    [600, 'DC'],
    [700, 'DCC'],
    [800, 'DCCC'],
    [900, 'CM'],
  ]);

  let numeral = '';
  if (num >= 1000) {
    let place = Math.floor(num / 1000);
    for (let i = 0; i < place; i++) numeral += 'M';
    num -= 1000 * place;
  }
  if (num >= 100) {
    let place = Math.floor(num / 100) * 100;
    numeral += codex.get(place);
    num -= place;
  }
  if (num >= 10) {
    let place = Math.floor(num / 10) * 10;
    numeral += codex.get(place);
    num -= place;
  }
  numeral += num !== 0 ? codex.get(num) : '';

  return numeral;
}
