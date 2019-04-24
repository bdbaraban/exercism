/**
 * encode - Run-length encode a string of letters.
 * @string: The string to run-length encode.
 *
 * Return: The run-length encoded string.
 */
const encode = string => {
	let letter = {};
	let l;
	let encoded = "";

	for (let char of string) {
		if (letter[char] === undefined) {
			l = Object.keys(letter)[0];
			if (l !== undefined) {
				encoded += (letter[l] !== 1) ? `${letter[l]}${l}` : `${l}`;
				delete letter[l];
			}
			letter[char] = 1;

		} else {
			letter[char] += 1;
		}
	}
	
	l = Object.keys(letter)[0];
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
const decode = string => {
	let count = 0;
	let decoded = "";

	for (let char of string) {
		if (+char >= 1 && +char <= 9) {
			count = (count * 10) + +char;
		} else {
			count = (count === 0) ? 1 : count;
			while (count > 0) {
				decoded += char;
				count--;
			}
		}
	}

	return decoded;
}

module.exports = {
	encode: encode,
	decode: decode
}
