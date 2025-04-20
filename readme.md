# XC - Nastroj pro prikazovou radku

Jednoduchy nastroj pro ukladani a spousteni prikazu z clipboard.

## Instalace

```bash
# Kompilace z repozitare
git clone https://github.com/[vas-uzivatel]/xc-clipboard.git
cd xc-clipboard
cargo build --release

# Kopirovani binarniho souboru do adresare, ktery je v PATH
cp target/release/xc ~/.local/bin/  # pro Linux/MacOS
# nebo
copy target\release\xc.exe C:\Windows\System32\  # pro Windows
```

## Pouziti

### Kopirovani prikazu do schranky

```bash
xc ls -la ~/.config
```

Prikaz bude ulozen do souboru `.xc_clipboard` v domovskem adresari.

### Spousteni prikazu ze schranky

```bash
xc -p
```

Spusti drive ulozeny prikaz a vrati stejny navratovy kod.

## Demo

Podivejte se na demo nastroje:

![XC demo](.assets/showcase.mp4)

## Jak to funguje

1. **Kopirovani prikazu:**

   - Program vezme vstupni argumenty a ulozi je do souboru v domovskem adresari
   - Zobrazi potvrzeni o kopirovani

2. **Vlozeni a spusteni prikazu:**
   - Program precte ulozeny prikaz ze souboru
   - Rozdeli prikaz na spustitelny soubor a argumenty
   - Spusti prikaz a predava vsechny standardni vstupy/vystupy
   - Ukonci s navratovym kodem spusteneho prikazu

## Pouziti v praxi

- Ulozeni slozitych prikazu, ktere se tezko pisi
- Rychle opakovani poslednich prikazu bez historie prikazove radky
- Sdileni prikazu mezi ruznymi terminaly

## Pozadavky

- Rust 1.85 nebo vyssi
- Pristup k domovskemu adresari pro uchovavani souboru schranky

## Licence

MIT
