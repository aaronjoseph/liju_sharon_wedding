# Liju & Sharon — Wedding Invitation

Dioxus 0.7.10 web invitation and guest guide for **Liju & Sharon** (`Sunday, 24 January 2027` • Vadakkencherry, Kerala).

## Local Development

```zsh
dx serve --web
```

## Production Web Bundle

```zsh
# Root domain (e.g. Netlify / custom domain)
dx bundle --web --release

# GitHub Pages project subpath (/liju_sharon_wedding/)
dx bundle --web --release --base-path liju_sharon_wedding
```
