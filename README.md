# rlmp

### Install
- Clone repo `git clone git@github.com:nevalicjus/rlmp`
- Run `cargo build --release`
- Copy `target/release/rlmp` to within PATH

To uninstall, remove the `rlmp` exec from where it was put, and rm `.cache/rlmp/persistent.cache`

### Usage
```
Usage: rlmp [options] [place] [selected date]

Arguments:
  [place]          Place to fetch weather for [default: approx. location]
  [selected date]  Starting date [default: 2026-04-28]

Options:
      --no-namedays  Don't show namedays
      --no-tui       Don't show tui
  -h, --help         Print help
  -V, --version      Print version
```

### Credits
- Weather, API [Open-Meteo](https://open-meteo.com)
- Namedays: 
  - Calendar pages, API [wikipedia](https://www.mediawiki.org/wiki/API:REST_API)
  - Polish names dataset [dane.gov.pl](https://dane.gov.pl/pl/dataset/1501,lista-imion-wystepujacych-w-rejestrze-pesel)
