//
// Given a name, returns a string with the message "One for X, one for me."
// Where X is the given name.
//

export const twoFer = name => {
	if (name === undefined) {
		return "One for you, one for me.";
	}
	return `One for ${name}, one for me.`;
}
