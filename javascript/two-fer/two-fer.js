/**
 * Returns a string with the message "One for X, one for me."
 * where X is a given name.
 */
export const twoFer = (name = undefined) => {
	if (name === undefined) {
		return "One for you, one for me.";
	}
	return `One for ${name}, one for me.`;
}
