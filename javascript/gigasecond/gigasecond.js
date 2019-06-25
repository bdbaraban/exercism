const GIGASECOND_IN_MS = 1000000000000;

export const gigasecond = date => new Date(date.getTime() + GIGASECOND_IN_MS);
