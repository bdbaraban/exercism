/**
 * solve - Returns the earned points in a single toss of a darts game.
 * @x: The x coordinate of the dart toss.
 * @y: The y coordinate of the dart toss.
 *
 * Return: The earned points of the dart toss.
 */
export const solve = (x, y) => {
	if (typeof(x) !== 'number' || typeof(y) !== 'number') {
		return (null);

	} else if (x >= -1 && x <= 1) {
		return (y >= -1 && y <= 1) ? 10 :
			(y >= -5 && y <= 5) ? 5 :
			(y >= -10 && y <= 10) ? 1 :
			0;

	} else if (x >= -5 && x <= 5) {
		return (y >= -5 && y <= 5) ? 5 :
			(y >= -10 && y <= 10) ? 1 :
			0;

	} else if (x >= -10 && x <= 10) {
		return (y >= -10 && y <= 10) ? 1 : 0;
	}

	return 0;
};
