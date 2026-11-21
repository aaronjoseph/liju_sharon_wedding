# Liju & Sharon — Wedding Website (`sharons_wedding`)

Workspace guide and rules for maintaining, building, and deploying the Dioxus 0.7.10 Christian wedding invitation and guest guide website for **Liju & Sharon**.

---

## 1. Couple, Families & Event Metadata

### Naming & Ordering Rule
- **Mandatory ordering:** Always list **Liju & Sharon** (Groom first, Bride second) across document titles, headings, monograms (`L & S`), WhatsApp RSVP templates, URLs, and prose.
- **Scripture:** *“And the two shall become one”* — Ephesians 5:31.

### Families & Footer
- **Groom:** Liju — S/o **Solly Thomas & Thomas Mathew**, *Kadankavil House, Vadakkencherry, Palakkad*.
- **Bride:** Sharon — D/o **Anitha Joseph & Joseph Mathew**, *Kalloor House, Kavumbhagam, Thiruvalla*.
- **Footer sign-off:** Display *"With love, Liya, Aaron & Hrudya"* (do not show *"Kadankavil & Kalloor Families"* in the footer).
- **Omitted terms:** Keep `"Horizon"`, `"Diptych"`, `"IST"`, `"TCR"`, and all National Highway (`NH`) / highway / toll / tunnel route details omitted across all UI copy.

### Schedule & Verified Venues (`Sunday, 24 January 2027`)
Target countdown timestamp: `2027-01-24T15:00:00+05:30`.

1. **Wedding — `3:00 PM`** *(1st event)*
   - **Venue:** Lourde Matha Forane Church, Vadakkencherry, Kerala 678683
   - **Google Maps:** `https://maps.app.goo.gl/3imLUGfDBieyiQkd6?g_st=iw`
2. **Reception — `6:30 PM onwards`** *(2nd event)*
   - **Venue:** TMK Arena, Vadakkencherry, Kerala 678683 (~3.5 km / 5–8 mins from the church)
   - **Google Maps:** `https://maps.app.goo.gl/6Qqgu8MbnhjLqPpQ6?g_st=iw`

- **Event cards (`EventsSection`):** Two center-aligned arched cards without Roman numerals (`I`/`II`) or filter bars, each with Google Maps and `.ics` calendar download actions.

### Memory Wall (`MemoryWallSection`)
- **Shared Google Photos album (`MEMORY_WALL_URL`):** `https://photos.app.goo.gl/1gW4Lzezf9qKHPKJ6`
- **Visual & copy:** Single clickable `3:2` landscape couple polaroid (`memory_couple_polaroid.jpg`, caption *"Liju & Sharon"* + *"Add wedding pictures here ↗"*) linking directly to Google Photos (no second `+ Your Snapshot` card). Heading *"Add Your Memories & Wedding Pictures Here"* and crowdsourced description *"Feel free to add memories and pictures from the wedding — favourite throwback moments, candid clicks, and short clips straight into our shared Google Photos album."* plus Copy Album Link and WhatsApp share buttons.

### RSVP (`RsvpSection`)
- Positioned directly **above What to Wear** on the main invitation page.
- **Side selector gate:** Animated Groom's Side / Bride's Side Ghibli portraits (`rsvp_liju_fountain.jpg`, `rsvp_sharon_fountain.jpg`) that expand the full RSVP form on selection.
- **Arrival slots (`ARRIVAL_SLOTS`):** `Jan 23 (Day Before)`, `Jan 24 Morning`, `Jan 24 • By 3:00 PM (Church)`, `Jan 24 • By 6:30 PM (Reception)`.
- **Google Sheets sync (`GOOGLE_SHEETS_WEBHOOK_URL`):** Posts RSVP payloads to the configured Google Apps Script Web App webhook.
- **WhatsApp RSVP numbers:**
  - Groom's Side (`LIJU_WHATSAPP_NUMBER`): `"919037569110"`
  - Bride's Side (`SHARON_WHATSAPP_NUMBER`): `"919844348041"`

### Guest Attire Palette & Combined Studio Ghibli Lookbook (`AttireSection`)
- **Shades:** `Dusty Rose` (`#d68a98`), `Ivory` (`#fbf6ec`), `Champagne` (`#ebd6b3`), and `Sage Green` (`#9caf88`) — shown as a compact pill strip above a single Studio Ghibli wedding celebration tableau (`attire_guests_ghibli.jpg`).
- **Gentlemen:** `Pant & Shirt`, `Pant & Kurta`, and `Mundu & Jubba` (no shawls/stoles, no sherwanis).
- **Ladies:** `Salwar Suits`, `Sarees`, and `Lehenga`.

### Standalone Travel & Stay Page (`#/travel`, `TravelPageView`)
- **Separated from the main invitation page:** **The Way to Vadakkencherry** (`VenueAndTravelSection` — chronological visual route + **Where to Stay & Getting Around**) lives on its own client-side page (`ActivePage::Travel`, URL `#/travel`), reachable from:
  1. The top navigation popover (`Way to Vadakkencherry →` and `Where to Stay`),
  2. The **The Way to Vadakkencherry** teaser card (`TravelGuideTeaserSection`, *"Open Travel & Stay Guide →"*) at the bottom of the main page, or
  3. Direct shareable link (`https://aaronjoseph.github.io/liju_sharon_wedding/#/travel`).
- **Visual Route (`SchematicRouteMap`):** Chronological Studio Ghibli–illustrated journey showing **Kochi (`COK`) → Thrissur → Lourde Matha Forane Church (`3:00 PM` Wedding) → TMK Arena (`6:30 PM` Reception)**, using `route_cok_airport.jpg`, `route_thrissur.jpg`, full-facade `route_church_ghibli.jpg`, and `route_tmk_arena_ghibli.jpg`, with Directions and Copy Address buttons integrated inside the venue stops.
- **Accommodation & local commute:** Airbnbs/hotels in **Vadakkencherry** (5–10 mins), **Thrissur** (~30 km / 35 mins), or **Palakkad** (~35 km / 40 mins); recommend **Uber**, local cabs, or **car rental**.

---

## 2. Visual Design System, Audio & UX Rules

- **Mood & aesthetic:** **Sunlit Ivory, Champagne Gold, Dusty Rose & Sage Botanical Elegance** paired with **Studio Ghibli–inspired hand-painted watercolor illustrations and breeze animations** (drifting blossom petals, rising champagne light motes, sunlit sheen, and gentle floating card motion).
- **Color palette (`assets/main.css` `:root`):**
  - Core wedding palette: `--dusty-rose` (`#d68a98`), `--ivory-silk` (`#fbf6ec`), `--champagne` (`#ebd6b3`), `--sage-green` (`#9caf88`).
  - Supporting surfaces & accents: `--pearl-alabaster` (`#fffdf9`), `--antique-gold` (`#b58e54`), `--dusty-rose-deep` (`#a95c6b`), `--sage-deep` (`#556b48`), `--ink-espresso` (`#2f2724`), `--ink-muted` (`#685b55`).
- **Typography (Google Fonts loaded in `index.html`):**
  - `Great Vibes` — script display names (`Liju & Sharon`), monogram (`L & S`), and footer wordmark.
  - `Lora` — warm editorial serif headings (`h2`, `h3`), family names, and scripture accents.
  - `Nunito` — clean rounded UI, metadata pills, and body font.
- **Browser tab icon (`index.html`):** Inline SVG `L&S` favicon matching the hero/navbar monogram, plus `#app-boot-curtain` FOUC prevention lifted once CSS is applied.
- **Hero dock & RSVP nudges:**
  - Compact `fit-content` hero dock (`max-width: min(640px, 100%)` on desktop, `width: 100%` on mobile) with **Days and Hours only** (`#cd-days`, `#cd-hours` — do **not** add minutes or seconds) and a single **RSVP & Arrival** button (no `Visual Route` button in the hero dock).
  - Moving nudge elements over RSVP buttons: floating bobbing **`✦ Kindly RSVP`** pill badge, diagonal pearl sheen sweep, expanding dusty-rose halo pulse, bobbing `↓` arrow, and pulsing ivory dot + sheen on the sticky top navbar `RSVP` pill.
- **Background music (`BackgroundMusicFab`):**
  - Audio file: `assets/wedding_bgm.m4a` (remuxed AAC `.m4a` via `afconvert -f m4af -d aac -b 128000` for universal browser compatibility).
  - **Icon-only floating button:** Musical note when playing, slashed note when muted — **never** display the song title or text label in the UI.
  - **Continuous across pages:** Mounted at the root of `App` outside `<main>` so switching between Home and `#/travel` never interrupts playback, and direct `#/travel` visits run the same music.
  - **iOS WebKit unlock:** Autoplay is attempted on load; when blocked by iOS Safari/Chrome autoplay policy, `unlockOnGesture` unlocks playback on the first tap (`pointerdown`, `touchstart`, `touchend`, `click`, `keydown`) with `playsinline` / `webkit-playsinline`.
- **Motion & responsiveness:** Respect `@media (prefers-reduced-motion: reduce)` and keep layouts tested across desktop and the `@media (max-width: 980px)` / `@media (max-width: 680px)` breakpoints in `assets/main.css`.

---

## 3. Architecture & File Map

- `Cargo.toml` — Rust `2021` edition, `dioxus = "0.7.10"` (`default = ["web"]`, plus `desktop` and `mobile` features). Keep dependencies minimal (`dioxus` only) so offline builds succeed from the local Cargo cache (`Cargo.lock`).
- `Dioxus.toml` — App name `sharons_wedding`; web title `"Liju & Sharon | Wedding Invitation • 24 Jan 2027"`.
- `index.html` — Custom HTML shell with `L&S` SVG favicon, Google Fonts, and `#app-boot-curtain` FOUC-prevention styles.
- `src/main.rs` — Single-file Dioxus app containing:
  - Constants, bilingual helpers (`Lang`, `tr`), `ENHANCE_SCRIPT` (scroll-spy, reveal observer, countdown, BGM autoplay/unlock, boot curtain lift), and typed data tables (`ATTIRE_SWATCHES`, `EVENTS`, `ARRIVAL_SLOTS`).
  - Client-side page routing (`ActivePage::Home` / `ActivePage::Travel`, `navigate_to_page`, `hashchange`/`popstate` sync, instant top-scroll on page switches).
  - Components: `App`, `Navbar`, `BackgroundMusicFab`, `GhibliBreezeLayer`, `HeroInvitationScene` + `CoupleGhibliCameo` + `HeroMeadowWaveSvg`, `EventsSection`, `MemoryWallSection`, `RsvpSection`, `AttireSection`, `TravelGuideTeaserSection`, `TravelPageView` + `VenueAndTravelSection` + `SchematicRouteMap`, and `Footer`.
- `assets/main.css` — Full stylesheet for sunlit ivory/champagne palette, Ghibli breeze/petal animations, frosted navbar, RSVP nudge animations, arched event cards, Memory Wall polaroid board, RSVP portrait gate + expandable drawer + keepsake pass, attire showcase, standalone Travel page + chronological route map, and responsive rules.
- `assets/*.jpg` — Active bundled images: `hero_ghibli_garden.jpg`, `couple_ghibli_cameo.jpg`, `memory_couple_polaroid.jpg`, `attire_guests_ghibli.jpg`, `route_cok_airport.jpg`, `route_thrissur.jpg`, `route_church_ghibli.jpg`, `route_tmk_arena_ghibli.jpg`, `rsvp_liju_fountain.jpg`, `rsvp_sharon_fountain.jpg`.
- `assets/wedding_bgm.m4a` — Background piano track (`128 kbps` AAC `.m4a`).
- `assets/*.ics` — RFC-5545 calendar invites (`liju_sharon_wedding.ics` and `liju_sharon_reception.ics`), each embedding a 2-days-in-advance notification (`VALARM` / `TRIGGER:-P2D`).
- `netlify.toml` — Legacy/optional Netlify build & MIME headers config.
- `.gitignore` — Ignores `/target`, `.DS_Store`, and `.netlify`.

---

## 4. Local Toolchain, Commands & TLS Proxy Notes

### Binary Paths
- `cargo` / `rustup`: `/opt/homebrew/opt/rustup/bin` (`wasm32-unknown-unknown` target installed)
- `dx` (`dioxus-cli v0.7.10`): `~/.cargo/bin/dx` — ensure `"$HOME/.cargo/bin"` is on `PATH` when invoking `dx`.

### Common Commands
- **Format check:**
  ```zsh
  rustfmt --edition 2021 --check src/main.rs
  ```
- **Offline Wasm typecheck:**
  ```zsh
  cargo check --offline --target wasm32-unknown-unknown
  ```
- **Clean GitHub Pages release bundle:**
  ```zsh
  rm -rf target/dx/sharons_wedding/release/web/public && CARGO_NET_OFFLINE=true PATH="$HOME/.cargo/bin:$PATH" dx bundle --web --release --base-path liju_sharon_wedding --out-dir <scratch_out>
  ```

### Corporate TLS Inspection (`UnknownIssuer`) Workarounds
- Pre-cached Dioxus CLI tools live in `~/.dx/tools/` (`wasm-bindgen-0.2.130`, `esbuild-0.27.3`, `binaryen-129`).
- When downloading media with `yt-dlp` on this machine, pass `--no-check-certificates` and remux DASH `.m4a` audio via `afconvert -f m4af -d aac -b 128000` for universal browser/iOS playback.
- For Node CLI tools (`npx netlify-cli` / `@netlify/mcp`) on Node `v25+`, pass `NODE_USE_SYSTEM_CA=1` if HTTPS requests hit TLS issuer errors.

---

## 5. GitHub Pages Deployment (Primary) & Guardrails

- **Live site:** `https://aaronjoseph.github.io/liju_sharon_wedding/` (and `https://aaronjoseph.github.io/liju_sharon_wedding/#/travel` for the Travel & Stay Guide).
- **Repository:** `https://github.com/aaronjoseph/liju_sharon_wedding.git`
  - `main` branch — Rust/Dioxus source, assets, and `AGENTS.md`.
  - `gh-pages` branch — Published static release bundle (`index.html`, `404.html`, `.nojekyll`, `assets/*`; unused `wasm/` subdir removed).
- **Release publish steps:**
  1. Always remove `target/dx/sharons_wedding/release/web/public` before running `dx bundle --web --release --base-path liju_sharon_wedding` so no stale hashed assets are copied into the output.
  2. Inject `<link rel="preload">` tags into `public/index.html` for the single hashed `main-*.css`, `sharons_wedding_bg-*.wasm`, `couple_ghibli_cameo-*.jpg`, and `hero_ghibli_garden-*.jpg`, copy `index.html` to `404.html`, touch `.nojekyll`, and remove `public/wasm`.
  3. Force-push `public/` to `origin gh-pages` using repo-local personal git identity and `gh auth git-credential`:
     ```zsh
     git config --local user.name "Aaron Joseph Mathew"
     git config --local user.email "aaronjosephmathew@gmail.com"
     git -c credential.helper= -c 'credential.helper=!/opt/homebrew/bin/gh auth git-credential' push -f origin gh-pages
     ```
- **Git & security guardrails:**
  - Never modify global git config (`--global`); keep personal GitHub identity scoped `--local` to this repo and scratch deploy folders.
  - Keep `.netlify` in `.gitignore` and never hardcode site IDs, auth tokens, or secrets into config or source files.
