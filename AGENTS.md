# Liju & Sharon — Wedding Website (`sharons_wedding`)

Workspace guide and rules for maintaining, building, and deploying the Dioxus 0.7.10 Christian wedding invitation and guest guide website for **Liju & Sharon**.

---

## 1. Couple, Families & Event Metadata

### Naming & Ordering Rule
- **Mandatory ordering:** Always list **Liju & Sharon** (Groom first, Bride second) across document titles, headings, monograms (`L & S`), WhatsApp RSVP templates, URLs/subdomains, and prose.
- **Scripture:** *“And the two shall become one”* — Ephesians 5:31.

### Families
- **Groom:** Liju — S/o **Solly Thomas & Thomas Mathew**, *Kadankavil House, Vadakkencherry, Palakkad*.
- **Bride:** Sharon — D/o **Anitha Joseph & Joseph Mathew**, *Kalloor House, Kavumbhagam, Thiruvalla*.
- **Omitted terms:** Keep `"Sharing the love: Liya, Aaron & Hrudya"`, `"Horizon"`, `"Diptych"`, and all National Highway (`NH`) / highway / toll / tunnel route details omitted across all UI copy.

### Schedule & Verified Venues (`Sunday, 24 January 2027`)
Target countdown timestamp: `2027-01-24T15:00:00+05:30` (IST).

1. **Wedding at Church — `3:00 PM IST`** *(1st event)*
   - **Venue:** Lourde Matha Forane Church, Vadakkencherry, Palakkad Dist., Kerala 678683
   - **Google Maps:** `https://maps.app.goo.gl/3imLUGfDBieyiQkd6?g_st=iw`
2. **Evening Reception — `Around 7:00 PM onwards IST`** *(2nd event)*
   - **Venue:** TMK Arena, Near Panniyankara Toll Plaza, Vadakkencherry, Kerala 678683 (~3.5 km / 5–8 mins from the church)
   - **Google Maps:** `https://maps.app.goo.gl/6Qqgu8MbnhjLqPpQ6?g_st=iw`

- **Guest Attire Palette & Combined Studio Ghibli Lookbook (both events):**
  - **Shades:** `Dusty Rose` (`#d68a98`), `Ivory` (`#fbf6ec`), `Champagne` (`#ebd6b3`), and `Sage Green` (`#9caf88`) — shown as a compact pill strip above a single vibrant, original Studio Ghibli wedding celebration tableau (`attire_guests_ghibli.jpg`) combining **Gentlemen and Ladies together** in front of a floral wedding arch & white Kerala church.
  - **Gentlemen:** `Pant & Shirt`, `Pant & Kurta`, and `Mundu & Jubba` (no shawls/stoles, no sherwanis).
  - **Ladies:** `Salwar Suits`, `Pastel Sarees`, and `Anarkali / Churidar` rotating the four wedding shades (do not use overly traditional Kerala kasavu lehenga).

### Guest Stay, Travel & Chronological Visual Route
- **Visual Route (`SchematicRouteMap`):** Chronological (time-wise) Studio Ghibli–illustrated journey showing **Kochi (`COK`) → Thrissur (`TCR`) → Lourde Matha Forane Church (`3:00 PM` Wedding) → TMK Arena (`~7:00 PM` Reception)**, with Ghibli illustrations for COK Airport (`route_cok_airport.jpg`) & Thrissur (`route_thrissur.jpg`), living animated Ghibli frames faithful to the real facades of Lourde Matha Forane Church (`route_church_ghibli.jpg`) & TMK Arena (`route_tmk_arena_ghibli.jpg`), Directions links, and Copy Address actions integrated directly inside the route stops (do not duplicate the two venue cards below the route map).
- **Accommodation:** Guests can book Airbnbs or hotels in **Thrissur** (~30 km / 35 mins west), **Palakkad** (~35 km / 40 mins east), or **Vadakkencherry** (5–10 mins from both venues).
- **Local commute:** Recommend **Uber**, local cabs, or a **car rental** for commuting smoothly between Airbnbs/hotels, Lourde Matha Forane Church, and TMK Arena.
- **Transit hubs:** Thrissur (TCR) & Palakkad (PGT) railway stations; Cochin (COK) & Coimbatore (CJB) airports; Vadakkencherry bus stops.
- **RSVP arrival slots (`ARRIVAL_SLOTS`):** `Jan 23 (Day Before)`, `Jan 24 Morning`, `Jan 24 • By 3:00 PM (Church)`, `Jan 24 • By 7:00 PM (Reception)`.
- **WhatsApp RSVP (`WHATSAPP_NUMBER`):** Configured in `src/main.rs` as `"919844348064"`.

---

## 2. Visual Design System & UX Rules

- **Mood & aesthetic:** **Sunlit Ivory, Champagne Gold, Dusty Rose & Sage Botanical Elegance** paired with **Studio Ghibli–inspired hand-painted watercolor illustrations and breeze animations** (drifting blossom petals, rising champagne light motes, sunlit sheen, and gentle floating card motion).
- **Color palette (`assets/main.css` `:root`):**
  - Core wedding palette: `--dusty-rose` (`#d68a98`), `--ivory-silk` (`#fbf6ec`), `--champagne` (`#ebd6b3`), `--sage-green` (`#9caf88`).
  - Supporting surfaces & accents: `--pearl-alabaster` (`#fffdf9`), `--antique-gold` (`#b58e54`), `--dusty-rose-deep` (`#a95c6b`), `--sage-deep` (`#556b48`), `--ink-espresso` (`#2f2724`), `--ink-muted` (`#685b55`).
- **Typography (Google Fonts loaded in `App`):**
  - `Great Vibes` — script display names (`Liju & Sharon`), monogram (`L & S`), and footer wordmark.
  - `Lora` — warm editorial serif headings (`h2`, `h3`), family names, and scripture accents.
  - `Nunito` — clean rounded UI, metadata pills, and body font.
- **Countdown rule:** Display **Days and Hours only** (`#cd-days` and `#cd-hours` in `HeroInvitationScene` / `ENHANCE_SCRIPT`). Do **not** add minutes or seconds.
- **Motion & responsiveness:** Respect `@media (prefers-reduced-motion: reduce)` and keep layouts tested across desktop and the `@media (max-width: 980px)` / `@media (max-width: 680px)` breakpoints in `assets/main.css`.

---

## 3. Architecture & File Map

- `Cargo.toml` — Rust `2021` edition, `dioxus = "0.7.10"` (`default = ["web"]`, plus `desktop` and `mobile` features). Keep dependencies minimal (`dioxus` only) so offline builds succeed from the local Cargo cache (`Cargo.lock`).
- `Dioxus.toml` — App name `sharons_wedding`; web title `"Liju & Sharon | Wedding Invitation • 24 Jan 2027"`.
- `src/main.rs` — Single-file Dioxus app containing:
  - Constants & typed data tables (`Swatch`, `EventInfo`, `ATTIRE_SWATCHES`, `EVENTS`, `ARRIVAL_SLOTS`).
  - Components: `App`, `Navbar` (compact floating pill + active-section indicator + English/Malayalam toggle + sections popover), `GhibliBreezeLayer`, `HeroInvitationScene` + `CoupleGhibliCameo` + `HeroMeadowWaveSvg`, `EventsSection` (two side-by-side arched cards without filter bar), `AttireSection` (softened-contrast Studio Ghibli combined wedding lookbook for Gentlemen & Ladies), `SchematicRouteMap` + `VenueAndTravelSection` + `SmartJourneyPlanner` (chronological Kochi → Thrissur → Wedding → Reception map, Stay/Airbnb & Uber/Car Rental cards, and interactive Google Maps Arrival + Return/Exit planner for Kochi Flight & Palakkad/Kochi Train), `RsvpSection` (single-portrait Groom's Side / Bride's Side Ghibli selector gate that expands the full RSVP form on click), and `Footer`.
- `assets/main.css` — Sunlit Ivory & Champagne stylesheet, Ghibli petal/mote/sunbeam/firefly animations, frosted pearl navbar, arched event cards, low-contrast combined Ghibli lookbook showcase, chronological route map, interactive Smart Journey Planner, RSVP portrait gate + expandable drawer + keepsake pass, and responsive rules.
- `assets/*.jpg` — Studio Ghibli watercolor hero (`hero_ghibli_garden.jpg`), couple cameo (`couple_ghibli_cameo.jpg` — Liju in Ivory Mundu & Jubba and Sharon in Cream Kerala Saree with Golden Handwork Blouse), vibrant combined guest attire celebration frame (`attire_guests_ghibli.jpg`), chronological route illustrations (`route_cok_airport.jpg`, `route_thrissur.jpg`, `route_church_ghibli.jpg`, `route_tmk_arena_ghibli.jpg`), and animated RSVP Groom/Bride Ghibli portraits (`rsvp_liju_fountain.jpg`, `rsvp_sharon_fountain.jpg`).
- `assets/*.ics` — RFC-5545 calendar invites (`liju_sharon_wedding.ics` with summary `Liju & Sharon - Wedding`, and `liju_sharon_reception.ics` with summary `Liju & Sharon - Wedding Reception`), each embedding a 2-days-in-advance notification (`VALARM` / `TRIGGER:-P2D`).
- `netlify.toml` — Netlify build, SPA fallback, Wasm & `.ics` (`text/calendar`) MIME headers, and immutable asset caching config.
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
- **Clean release web bundle:**
  ```zsh
  rm -rf target/dx/sharons_wedding/release/web/public && CARGO_NET_OFFLINE=true PATH="$HOME/.cargo/bin:$PATH" dx bundle --web --release
  ```

### Corporate TLS Inspection (`UnknownIssuer`) Workarounds
- Pre-cached Dioxus CLI tools live in `~/.dx/tools/` (`wasm-bindgen-0.2.130`, `esbuild-0.27.3`, `binaryen-129`).
- For Node CLI tools (`npx netlify-cli` / `@netlify/mcp`) on Node `v25+`, pass `NODE_USE_SYSTEM_CA=1` if HTTPS requests hit TLS issuer errors.

---

## 5. Netlify Deployment & Guardrails

- **Netlify project:** Hosted on team **Sharon** (`aaron-mathew`) under project **`liju-and-sharon`** (`https://liju-and-sharon.netlify.app`), publicly accessible.
- **Fast prebuilt deploy note:** Stage `target/dx/sharons_wedding/release/web/public/*` plus a static `publish = "."` `netlify.toml` into an isolated scratch folder before running `@netlify/mcp` so Cargo's `target/` directory (~1.5 GB) is excluded.
- **Guardrails:**
  - Keep `.netlify` in `.gitignore` so `.netlify/state.json` is never committed.
  - Never hardcode Netlify site IDs, auth tokens, or secrets into `netlify.toml`, `AGENTS.md`, or source files, and never disable secrets scanning (`SECRETS_SCAN_ENABLED=false`).
