export default class Words {
  // Returns a Map containing the word counts in a given string.
  public count = (phrase: string): Map<string, number> => {
    const words: string[] = phrase.trim().split(/\s+/g)
    const wordCount: Map<string, number> = new Map()
    words.map((word) => {
      word = word.toLowerCase()
      wordCount.has(word) ?
        wordCount.set(word, wordCount.get(word)! + 1) : wordCount.set(word, 1)
    })
    return wordCount
  }
}
