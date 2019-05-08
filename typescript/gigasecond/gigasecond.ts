// Calculate a Date one gigasecond later.
export default class Gigasecond {
  public giga: Date
  constructor(date: Date) {
    this.giga = new Date(date)
    this.giga.setSeconds(this.giga.getSeconds() + 1000000000)
  }

  public date(): Date {
    return this.giga
  }
}
