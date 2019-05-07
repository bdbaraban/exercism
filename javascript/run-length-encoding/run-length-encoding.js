/**
 * encode - Run-length encode a string of letters.
 * @string: The string to run-length encode.
 *
 * Return: The run-length encoded string.
 */
export const encode = str => {
  let letter = {};
  let encoded = "";

  for (let ch of str) {
    if (letter[ch] === undefined) {
      encoded = encode_letter(letter, encoded);
      letter[ch] = 1;
    } else {
      letter[ch] += 1;
    }
  }
	
  return encode_letter(letter, encoded);
}

/**
 * encode_letter - Encode a letter in a run-length encoded string.
 * letter: An object containing the key/value count of the letter to encode.
 * encoded: The building run-length encoded string.
 *
 * Return: The run-length encoded string with the added encoded letter.
  */
const encode_letter = (letter, encoded) => {
  let l = Object.keys(letter)[0];
  if (l !== undefined) {
    encoded += (letter[l] !== 1) ? `${letter[l]}${l}` : `${l}`;
    delete letter[l];
  }
  return encoded;
}

/**
 * decode - Decode a run-length encoded string of letters.
 * @string: The run-length encoded string to decode.
 *
 * Return: The decoded run-length encoded string.
 */
export const decode = string => {
  let count = 0;
  let decoded = "";

  for (let ch of string) {
    if (+ch >= 1 && +ch <= 9) {
      count = (count * 10) + +ch;
    } else {
      for (count = (count === 0) ? 1 : count; count > 0; count--) {
        decoded += ch;
      }
    }
  }

  return decoded;
}
