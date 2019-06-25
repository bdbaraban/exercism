export default class SpaceAge {
  private SECONDS_IN_YEAR = 31557600;
  public seconds: number;

  public constructor(seconds: number) {
    this.seconds = seconds;
  }

  private round = (age: number): number => Number(age.toFixed(2));

  private calculate(orbital: number): number {
    return this.round(this.seconds / (this.SECONDS_IN_YEAR * orbital));
  }

  public onEarth(): number {
    return this.calculate(1);
  }

  public onMercury(): number {
    return this.calculate(0.2408467);
  }

  public onVenus(): number {
    return this.calculate(0.61519726);
  }

  public onMars(): number {
    return this.calculate(1.8808158);
  }

  public onJupiter(): number {
    return this.calculate(11.862615);
  }

  public onSaturn(): number {
    return this.calculate(29.447498);
  }

  public onUranus(): number {
    return this.calculate(84.016846);
  }

  public onNeptune(): number {
    return this.calculate(164.79132);
  }
}
