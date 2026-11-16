use dioxus::document;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HERO_GHIBLI_BG: Asset = asset!("/assets/hero_ghibli_garden.jpg");
const COUPLE_GHIBLI_CAMEO: Asset = asset!("/assets/couple_ghibli_cameo.jpg");

const ROUTE_COK_AIRPORT: Asset = asset!("/assets/route_cok_airport.jpg");
const ROUTE_THRISSUR: Asset = asset!("/assets/route_thrissur.jpg");
const CHURCH_GHIBLI: Asset = asset!("/assets/route_church_ghibli.jpg");
const RECEPTION_GHIBLI: Asset = asset!("/assets/route_tmk_arena_ghibli.jpg");

const ATTIRE_GUESTS_GHIBLI: Asset = asset!("/assets/attire_guests_ghibli.jpg");

const RSVP_LIJU_FOUNTAIN: Asset = asset!("/assets/rsvp_liju_fountain.jpg");
const RSVP_SHARON_FOUNTAIN: Asset = asset!("/assets/rsvp_sharon_fountain.jpg");

const WEDDING_ICS: Asset = asset!("/assets/liju_sharon_wedding.ics");
const RECEPTION_ICS: Asset = asset!("/assets/liju_sharon_reception.ics");

const CHURCH_NAME: &str = "Lourde Matha Forane Church";
const CHURCH_ADDRESS: &str = "Vadakkencherry, Kerala 678683";
const CHURCH_MAP_URL: &str = "https://maps.app.goo.gl/3imLUGfDBieyiQkd6?g_st=iw";

const RECEPTION_NAME: &str = "TMK Arena";
const RECEPTION_ADDRESS: &str = "Vadakkencherry, Kerala 678683";
const RECEPTION_MAP_URL: &str = "https://maps.app.goo.gl/6Qqgu8MbnhjLqPpQ6?g_st=iw";

const WHATSAPP_NUMBER: &str = "919844348064";
const GOOGLE_SHEETS_WEBHOOK_URL: &str = "https://script.google.com/macros/s/AKfycbwc5AE_05hsVLx0QKlHUwUGxtgN8wAbkuWMrQ41i7DwVnHyaZZzgzWYwPVxRtIL9KGNGg/exec";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Lang {
    En,
    Ml,
}

#[inline]
fn tr(lang: Lang, en: &'static str, ml: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Ml => ml,
    }
}

const ENHANCE_SCRIPT: &str = r##"
(function () {
    if (window.__slDeckAbort) {
        window.__slDeckAbort.abort();
        window.__slDeckAbort = null;
    }
    const root = document.documentElement;
    const progressEl = document.getElementById("scroll-progress");
    const navEl = document.getElementById("floating-nav");
    const reducedMotion = window.matchMedia &&
        window.matchMedia("(prefers-reduced-motion: reduce)").matches;

    const trackedSections = [
        { id: "invitation", en: "Invitation", ml: "ക്ഷണക്കത്ത്" },
        { id: "events", en: "The Wedding Day", ml: "വിവാഹ ദിനം" },
        { id: "palette", en: "What to Wear", ml: "വസ്ത്രധാരണം" },
        { id: "venue", en: "Visual Route", ml: "യാത്രാ മാർഗ്ഗം" },
        { id: "stay-travel", en: "Where to Stay", ml: "താമസ സൗകര്യങ്ങൾ" },
        { id: "journey-planner", en: "Journey Planner", ml: "യാത്രാ പ്ലാനർ" },
        { id: "rsvp", en: "RSVP & Arrival", ml: "RSVP & വരവ്" },
    ];
    window.__slLang = window.__slLang || "en";
    let lastActiveKey = "";

    function updateActiveSection(y, max) {
        const navSectionLabel = document.getElementById("nav-current-section");
        const popoverLinks = Array.from(document.querySelectorAll(".nav-popover a[href^='#']"));
        let active = trackedSections[0];
        if (y + 12 >= max) {
            active = trackedSections[trackedSections.length - 1];
        } else {
            const probe = Math.max(110, Math.min(240, window.innerHeight * 0.3));
            for (const sec of trackedSections) {
                const el = document.getElementById(sec.id);
                if (!el) continue;
                const rect = el.getBoundingClientRect();
                if (rect.top <= probe) {
                    active = sec;
                }
            }
        }

        const langCode = window.__slLang === "ml" ? "ml" : "en";
        const key = active.id + ":" + langCode;
        if (key !== lastActiveKey) {
            lastActiveKey = key;
            if (navSectionLabel) {
                navSectionLabel.textContent = active[langCode];
            }
            const targetHash = "#" + active.id;
            for (const link of popoverLinks) {
                if (link.getAttribute("href") === targetHash) {
                    link.classList.add("is-active");
                } else {
                    link.classList.remove("is-active");
                }
            }
        }
    }

    let ticking = false;
    function updateScrollState() {
        ticking = false;
        const y = window.scrollY || window.pageYOffset || 0;
        const max = Math.max(
            1,
            document.documentElement.scrollHeight - window.innerHeight
        );
        const ratio = Math.min(1, Math.max(0, y / max));
        if (progressEl) {
            progressEl.style.setProperty("--scroll-progress", ratio.toFixed(4));
        }
        if (!reducedMotion) {
            const shift = Math.min(140, y * 0.18).toFixed(1) + "px";
            root.style.setProperty("--parallax-shift", shift);
        }
        if (navEl) {
            if (y > 36) {
                navEl.classList.add("is-scrolled");
            } else {
                navEl.classList.remove("is-scrolled");
            }
        }
        updateActiveSection(y, max);
    }

    if (window.__slScrollHandler) {
        window.removeEventListener("scroll", window.__slScrollHandler);
    }
    window.__slScrollHandler = function () {
        if (!ticking) {
            ticking = true;
            window.requestAnimationFrame(updateScrollState);
        }
    };
    window.addEventListener("scroll", window.__slScrollHandler, { passive: true });
    window.__slSetLang = function (code) {
        window.__slLang = code === "ml" ? "ml" : "en";
        root.setAttribute("lang", window.__slLang);
        lastActiveKey = "";
        updateScrollState();
    };
    updateScrollState();

    if (!reducedMotion && "IntersectionObserver" in window) {
        root.classList.add("reveal-ready");
        if (window.__slRevealObserver) {
            window.__slRevealObserver.disconnect();
        }
        const observer = new IntersectionObserver(
            (entries) => {
                for (const entry of entries) {
                    if (entry.isIntersecting) {
                        entry.target.classList.add("is-visible");
                        observer.unobserve(entry.target);
                    }
                }
            },
            { threshold: 0.1, rootMargin: "0px 0px -30px 0px" }
        );
        window.__slRevealObserver = observer;

        const observeAll = () => {
            document.querySelectorAll("[data-reveal]:not(.is-visible)").forEach((el) => {
                observer.observe(el);
            });
        };
        observeAll();

        if (window.__slRevealMutation) {
            window.__slRevealMutation.disconnect();
        }
        const mutObserver = new MutationObserver(() => observeAll());
        mutObserver.observe(document.body, { childList: true, subtree: true });
        window.__slRevealMutation = mutObserver;
    }

    const targetMs = new Date("2027-01-24T15:00:00+05:30").getTime();
    function tickCountdown() {
        const diff = Math.max(0, targetMs - Date.now());
        const days = Math.floor(diff / 86400000);
        const hours = Math.floor((diff % 86400000) / 3600000);

        const dEl = document.getElementById("cd-days");
        const hEl = document.getElementById("cd-hours");
        if (dEl) dEl.textContent = String(days);
        if (hEl) hEl.textContent = String(hours).padStart(2, "0");
    }
    if (window.__slCountdownInterval) {
        clearInterval(window.__slCountdownInterval);
    }
    tickCountdown();
    window.__slCountdownInterval = setInterval(tickCountdown, 60000);
})();
"##;

struct Swatch {
    name: &'static str,
    name_ml: &'static str,
    hex: &'static str,
}

struct EventInfo {
    numeral: &'static str,
    badge: &'static str,
    badge_ml: &'static str,
    title: &'static str,
    title_ml: &'static str,
    subtitle: &'static str,
    subtitle_ml: &'static str,
    date: &'static str,
    date_ml: &'static str,
    time: &'static str,
    time_ml: &'static str,
    venue_area: &'static str,
    venue_area_ml: &'static str,
    map_url: &'static str,
    calendar_ics: Asset,
}

const ATTIRE_SWATCHES: &[Swatch] = &[
    Swatch {
        name: "Dusty Rose",
        name_ml: "ഡസ്റ്റി റോസ്",
        hex: "#d68a98",
    },
    Swatch {
        name: "Ivory",
        name_ml: "ഐവറി",
        hex: "#fbf6ec",
    },
    Swatch {
        name: "Champagne",
        name_ml: "ഷാംപെയ്ൻ",
        hex: "#ebd6b3",
    },
    Swatch {
        name: "Sage Green",
        name_ml: "സേജ് ഗ്രീൻ",
        hex: "#9caf88",
    },
];

const EVENTS: &[EventInfo] = &[
    EventInfo {
        numeral: "I",
        badge: "Sunday • 3:00 PM",
        badge_ml: "ഞായർ • 3:00 PM",
        title: "Wedding at Church",
        title_ml: "വിവാഹം — പള്ളിയിൽ",
        subtitle: "Solemn Nuptial Mass, sacred vows & joyful blessings with family",
        subtitle_ml: "തിരുവിവാഹ കുർബാന, വിശുദ്ധ പ്രതിജ്ഞകൾ & കുടുംബത്തോടൊപ്പമുള്ള പ്രാർത്ഥനാശീർവാദം",
        date: "Sunday, 24 January 2027",
        date_ml: "ഞായർ, 24 ജനുവരി 2027",
        time: "3:00 PM IST",
        time_ml: "ഉച്ചയ്ക്ക് ശേഷം 3:00 PM IST",
        venue_area: "Lourde Matha Forane Church, Vadakkencherry",
        venue_area_ml: "ലൂർദ് മാതാ ഫൊറോന പള്ളി, വടക്കഞ്ചേരി",
        map_url: CHURCH_MAP_URL,
        calendar_ics: WEDDING_ICS,
    },
    EventInfo {
        numeral: "II",
        badge: "Sunday • 7:00 PM",
        badge_ml: "ഞായർ • 7:00 PM",
        title: "Evening Reception",
        title_ml: "സായാഹ്ന വിരുന്ന്",
        subtitle: "Celebratory dinner, music, cake cutting & happy fellowship",
        subtitle_ml: "സ്നേഹവിരുന്ന്, സംഗീതം, കേക്ക് മുറിക്കൽ & സന്തോഷസംഗമം",
        date: "Sunday, 24 January 2027",
        date_ml: "ഞായർ, 24 ജനുവരി 2027",
        time: "7:00 PM onwards IST",
        time_ml: "വൈകുന്നേരം 7:00 PM മുതൽ IST",
        venue_area: "TMK Arena, Vadakkencherry",
        venue_area_ml: "TMK അരീന, വടക്കഞ്ചേരി",
        map_url: RECEPTION_MAP_URL,
        calendar_ics: RECEPTION_ICS,
    },
];

const ARRIVAL_SLOTS: &[&str] = &[
    "Jan 23 (Day Before)",
    "Jan 24 Morning",
    "Jan 24 • By 3:00 PM (Church)",
    "Jan 24 • By 7:00 PM (Reception)",
];

const ARRIVAL_SLOTS_ML: &[&str] = &[
    "ജനുവരി 23 (തലേദിവസം)",
    "ജനുവരി 24 രാവിലെ",
    "ജനുവരി 24 • 3:00 PM (വിവാഹം)",
    "ജനുവരി 24 • 7:00 PM (വിരുന്ന്)",
];

/// Percent-encodes text for safe inclusion in a WhatsApp URL query parameter.
fn percent_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len() * 2);
    for byte in input.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(*byte as char);
            }
            b' ' => encoded.push_str("%20"),
            _ => encoded.push_str(&format!("%{:02X}", byte)),
        }
    }
    encoded
}

/// Copies `text` to the browser clipboard and toggles `copied` for 2.4 seconds.
fn copy_with_feedback(text: String, mut copied: Signal<bool>) {
    copied.set(true);
    spawn(async move {
        let eval = document::eval(
            r#"
            const text = await dioxus.recv();
            let done = false;
            if (navigator.clipboard && navigator.clipboard.writeText) {
                try {
                    await navigator.clipboard.writeText(text);
                    done = true;
                } catch (_) {}
            }
            if (!done) {
                const ta = document.createElement("textarea");
                ta.value = text;
                ta.style.position = "fixed";
                ta.style.opacity = "0";
                document.body.appendChild(ta);
                ta.select();
                try {
                    document.execCommand("copy");
                } catch (_) {}
                document.body.removeChild(ta);
            }
            await new Promise((resolve) => setTimeout(resolve, 2400));
            return true;
            "#,
        );
        let _ = eval.send(text);
        let _ = eval.await;
        copied.set(false);
    });
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SheetSyncState {
    Idle,
    Saving,
    Saved,
    Failed,
}

/// Submits the guest RSVP payload to the Google Apps Script Web App webhook.
fn submit_rsvp_to_sheets(fields: Vec<String>, mut sync_state: Signal<SheetSyncState>) {
    if GOOGLE_SHEETS_WEBHOOK_URL.is_empty() {
        return;
    }
    sync_state.set(SheetSyncState::Saving);
    spawn(async move {
        let mut eval = document::eval(
            r#"
            const [url, guestName, familySide, attendingRaw, guestCountRaw, events, arrivalSlot, wishes, lang] = await dioxus.recv();
            const isAttending = attendingRaw === "true" || attendingRaw === "Yes";
            const parsedCount = isAttending ? (Number(guestCountRaw) || 1) : 0;
            try {
                await fetch(url, {
                    method: "POST",
                    mode: "no-cors",
                    headers: { "Content-Type": "text/plain;charset=utf-8" },
                    body: JSON.stringify({
                        timestamp: new Date().toISOString(),
                        guestName,
                        familySide,
                        side: familySide,
                        attending: isAttending,
                        guestCount: parsedCount,
                        events,
                        arrivalSlot,
                        wishes,
                        lang,
                        language: lang,
                    }),
                });
                return true;
            } catch (_) {
                return false;
            }
            "#,
        );
        let _ = eval.send(fields);
        let ok = eval.recv::<bool>().await.unwrap_or(false);
        sync_state.set(if ok {
            SheetSyncState::Saved
        } else {
            SheetSyncState::Failed
        });
    });
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let lang = use_context_provider(|| Signal::new(Lang::En));

    use_effect(move || {
        let _ = document::eval(ENHANCE_SCRIPT);
    });

    use_effect(move || {
        let code = match lang() {
            Lang::En => "en",
            Lang::Ml => "ml",
        };
        let _ = document::eval(&format!(
            "if (window.__slSetLang) {{ window.__slSetLang('{code}'); }}"
        ));
    });

    let current_lang = lang();

    rsx! {
        document::Title {
            {tr(
                current_lang,
                "Liju & Sharon | Wedding Invitation • 24 Jan 2027",
                "ലിജു & ഷാരോൺ | വിവാഹ ക്ഷണക്കത്ത് • 24 ജനുവരി 2027",
            )}
        }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Great+Vibes&family=Lora:ital,wght@0,400;0,500;0,600;1,400;1,500&family=Noto+Sans+Malayalam:wght@400;500;600;700&family=Noto+Serif+Malayalam:wght@500;600;700&family=Nunito:wght@400;500;600;700&display=swap",
        }
        document::Stylesheet { href: MAIN_CSS }

        Navbar {}
        main { class: if current_lang == Lang::Ml { "page-shell lang-ml" } else { "page-shell" },
            HeroInvitationScene {}
            EventsSection {}
            AttireSection {}
            VenueAndTravelSection {}
            RsvpSection {}
        }
        Footer {}
    }
}

#[component]
fn Navbar() -> Element {
    let mut lang = use_context::<Signal<Lang>>();
    let mut menu_open = use_signal(|| false);
    let current = lang();

    rsx! {
        div { id: "scroll-progress", class: "scroll-progress-bar", "aria-hidden": "true" }
        if menu_open() {
            div {
                class: "nav-backdrop-dismiss",
                "aria-hidden": "true",
                onclick: move |_| menu_open.set(false),
            }
        }
        header { id: "floating-nav", class: "floating-nav",
            div { class: "nav-shell",
                nav { class: "nav-capsule", aria_label: "Primary",
                    a {
                        href: "#top",
                        class: "nav-monogram",
                        onclick: move |_| menu_open.set(false),
                        "L & S"
                    }
                    span { class: "nav-dot-sep", "aria-hidden": "true", "•" }
                    button {
                        r#type: "button",
                        class: if menu_open() { "nav-menu-toggle is-open" } else { "nav-menu-toggle" },
                        aria_expanded: if menu_open() { "true" } else { "false" },
                        aria_label: tr(current, "Toggle sections menu", "വിഭാഗങ്ങൾ തുറക്കുക"),
                        onclick: move |_| menu_open.set(!menu_open()),
                        span { class: "nav-toggle-icon", if menu_open() { "✕" } else { "☰" } }
                        span {
                            id: "nav-current-section",
                            class: "nav-current-label",
                            "{tr(current, \"Invitation\", \"ക്ഷണക്കത്ത്\")}"
                        }
                    }
                    button {
                        r#type: "button",
                        class: if current == Lang::Ml { "nav-lang-btn is-ml" } else { "nav-lang-btn" },
                        aria_label: tr(current, "Switch language to Malayalam", "ഇംഗ്ലീഷിലേക്ക് മാറ്റുക"),
                        onclick: move |_| {
                            let next = match lang() {
                                Lang::En => Lang::Ml,
                                Lang::Ml => Lang::En,
                            };
                            lang.set(next);
                        },
                        if current == Lang::En { "മലയാളം" } else { "EN" }
                    }
                    a {
                        href: "#rsvp",
                        class: "nav-rsvp-pill",
                        onclick: move |_| menu_open.set(false),
                        "RSVP"
                    }
                }

                div {
                    class: if menu_open() { "nav-popover is-open" } else { "nav-popover" },
                    aria_hidden: if menu_open() { "false" } else { "true" },
                    a { href: "#invitation", onclick: move |_| menu_open.set(false), "{tr(current, \"Invitation\", \"ക്ഷണക്കത്ത്\")}" }
                    a { href: "#events", onclick: move |_| menu_open.set(false), "{tr(current, \"The Wedding Day\", \"വിവാഹ ദിനം\")}" }
                    a { href: "#palette", onclick: move |_| menu_open.set(false), "{tr(current, \"What to Wear\", \"വസ്ത്രധാരണം\")}" }
                    a { href: "#venue", onclick: move |_| menu_open.set(false), "{tr(current, \"Visual Route\", \"യാത്രാ മാർഗ്ഗം\")}" }
                    a { href: "#stay-travel", onclick: move |_| menu_open.set(false), "{tr(current, \"Where to Stay\", \"താമസ സൗകര്യങ്ങൾ\")}" }
                    a { href: "#journey-planner", onclick: move |_| menu_open.set(false), "{tr(current, \"Journey Planner\", \"യാത്രാ പ്ലാനർ\")}" }
                    a {
                        href: "#rsvp",
                        class: "popover-rsvp-link",
                        onclick: move |_| menu_open.set(false),
                        "{tr(current, \"RSVP & Arrival →\", \"RSVP & വരവ് →\")}"
                    }
                }
            }
        }
    }
}

/// Studio Ghibli–inspired gentle drifting blossom petals, sage leaves & warm champagne light motes
/// adhering strictly to Dusty Rose (#d68a98), Ivory (#fbf6ec), Champagne (#ebd6b3), and Sage Green (#9caf88).
#[component]
fn GhibliBreezeLayer() -> Element {
    rsx! {
        div { class: "ghibli-breeze-stage", "aria-hidden": "true",
            span { class: "ghibli-petal petal-rose p-1" }
            span { class: "ghibli-petal petal-champagne p-2" }
            span { class: "ghibli-petal petal-sage p-3" }
            span { class: "ghibli-petal petal-ivory p-4" }
            span { class: "ghibli-petal petal-rose p-5" }
            span { class: "ghibli-petal petal-champagne p-6" }
            span { class: "ghibli-petal petal-sage p-7" }
            span { class: "ghibli-petal petal-rose p-8" }
            span { class: "ghibli-mote m-1" }
            span { class: "ghibli-mote m-2" }
            span { class: "ghibli-mote m-3" }
            span { class: "ghibli-mote m-4" }
            span { class: "ghibli-mote m-5" }
        }
    }
}

#[component]
fn HeroMeadowWaveSvg() -> Element {
    rsx! {
        svg {
            class: "hero-meadow-wave",
            view_box: "0 0 1440 160",
            preserve_aspect_ratio: "none",
            xmlns: "http://www.w3.org/2000/svg",
            "aria-hidden": "true",
            path {
                class: "wave-sage",
                d: "M0,64 C260,126 520,18 840,72 C1110,118 1290,54 1440,42 L1440,160 L0,160 Z",
                fill: "rgba(156, 175, 136, 0.28)",
            }
            path {
                class: "wave-champagne",
                d: "M0,96 C320,44 640,132 980,88 C1205,58 1340,92 1440,82 L1440,160 L0,160 Z",
                fill: "rgba(235, 214, 179, 0.55)",
            }
            path {
                d: "M0,122 C310,96 630,148 960,118 C1195,96 1335,118 1440,128 L1440,160 L0,160 Z",
                fill: "#faf3e6",
            }
        }
    }
}

#[component]
fn CoupleGhibliCameo() -> Element {
    rsx! {
        div { class: "couple-cameo-stage",
            div { class: "cameo-aura-halo", "aria-hidden": "true" }
            span { class: "cameo-sparkle cs-1", "aria-hidden": "true", "✦" }
            span { class: "cameo-sparkle cs-2", "aria-hidden": "true", "✦" }
            span { class: "cameo-orbit-petal cp-1", "aria-hidden": "true" }
            span { class: "cameo-orbit-petal cp-2", "aria-hidden": "true" }

            div { class: "couple-cameo-frame",
                img {
                    src: COUPLE_GHIBLI_CAMEO,
                    alt: "Studio Ghibli watercolor portrait of Liju in an Ivory Mundu & Jubba and Sharon in a Cream & Gold Kerala Saree",
                    class: "couple-cameo-img",
                }
                div { class: "cameo-sunbeam-veil", "aria-hidden": "true" }
            }
        }
    }
}

#[component]
fn HeroInvitationScene() -> Element {
    let lang = use_context::<Signal<Lang>>()();

    rsx! {
        section { id: "top", class: "hero-sanctuary",
            div { class: "hero-ghibli-backdrop", "aria-hidden": "true",
                img {
                    src: HERO_GHIBLI_BG,
                    alt: "",
                    class: "hero-ghibli-img",
                }
                div { class: "hero-ivory-veil" }
            }

            GhibliBreezeLayer {}

            div { id: "invitation", class: "hero-centerpiece",
                div { class: "verse-pill hero-stagger-1",
                    span { class: "verse-quote", "{tr(lang, \"“And the two shall become one”\", \"“ഇരുവരും ഒരു ദേഹമായിത്തീരും”\")}" }
                    span { class: "verse-divider", "✦" }
                    span { class: "verse-ref", {tr(lang, "Ephesians 5:31", "എഫെസ്യർ 5:31")} }
                }

                CoupleGhibliCameo {}

                p { class: "hero-invite-lead hero-stagger-2",
                    "{tr(lang, \"With grateful hearts, both families joyfully invite you to celebrate the Holy Matrimony of\", \"ഹൃദയംഗമമായ നന്ദിയോടെ, ഞങ്ങളുടെ ഇരു കുടുംബങ്ങളും തിരുവിവാഹ ആശീർവാദത്തിലേക്ക് നിങ്ങളെ സ്നേഹപൂർവ്വം ക്ഷണിക്കുന്നു\")}"
                }

                h1 { class: if lang == Lang::Ml { "couple-monument couple-monument-ml hero-stagger-2" } else { "couple-monument hero-stagger-2" },
                    span { class: "monument-name", "{tr(lang, \"Liju\", \"ലിജു\")}" }
                    span { class: "monument-amp", "&" }
                    span { class: "monument-name", "{tr(lang, \"Sharon\", \"ഷാരോൺ\")}" }
                }

                div { class: "hero-family-columns hero-stagger-3",
                    div { class: "family-pillar",
                        span { class: "pillar-role", "{tr(lang, \"Groom • S/o\", \"വരൻ • മകൻ\")}" }
                        p { class: "pillar-parents", "{tr(lang, \"Solly Thomas & Thomas Mathew\", \"സോളി തോമസ് & തോമസ് മാത്യു\")}" }
                        p { class: "pillar-house", "{tr(lang, \"Kadankavil House, Vadakkencherry, Palakkad\", \"കടങ്കാവിൽ വീട്, വടക്കഞ്ചേരി, പാലക്കാട്\")}" }
                    }
                    div { class: "pillar-divider", "aria-hidden": "true" }
                    div { class: "family-pillar",
                        span { class: "pillar-role", "{tr(lang, \"Bride • D/o\", \"വധു • മകൾ\")}" }
                        p { class: "pillar-parents", "{tr(lang, \"Anitha Joseph & Joseph Mathew\", \"അനിത ജോസഫ് & ജോസഫ് മാത്യു\")}" }
                        p { class: "pillar-house", "{tr(lang, \"Kalloor House, Kavumbhagam, Thiruvalla\", \"കല്ലൂർ വീട്, കാവുംഭാഗം, തിരുവല്ല\")}" }
                    }
                }

                div { class: "hero-dock hero-stagger-4",
                    div { class: "dock-date-row",
                        span { class: "dock-date-crest",
                            {tr(lang, "SUNDAY • 24 JANUARY 2027 • 3:00 PM WEDDING & 7:00 PM RECEPTION", "ഞായർ • 24 ജനുവരി 2027 • 3:00 PM വിവാഹം & 7:00 PM വിരുന്ന്")}
                        }
                    }

                    div { class: "dock-bottom-row",
                        div { class: "countdown-pill", aria_label: tr(lang, "Countdown to wedding", "വിവാഹത്തിലേക്കുള്ള കൗണ്ട്ഡൗൺ"),
                            div { class: "countdown-group",
                                span { id: "cd-days", class: "countdown-num", "70" }
                                span { class: "countdown-unit", "{tr(lang, \"Days\", \"ദിവസം\")}" }
                            }
                            span { class: "countdown-dot", "•" }
                            div { class: "countdown-group",
                                span { id: "cd-hours", class: "countdown-num", "02" }
                                span { class: "countdown-unit", "{tr(lang, \"Hours\", \"മണിക്കൂർ\")}" }
                            }
                            span { class: "countdown-caption", "{tr(lang, \"until we say ‘I do’\", \"ആ സുദിനത്തിലേക്ക്\")}" }
                        }

                        div { class: "hero-actions",
                            a { href: "#rsvp", class: "btn btn-rose", "{tr(lang, \"RSVP & Arrival\", \"RSVP & വരവ്\")}" }
                            a { href: "#venue", class: "btn btn-ivory-outline", "{tr(lang, \"Visual Route ↓\", \"യാത്രാ മാർഗ്ഗം ↓\")}" }
                        }
                    }
                }
            }

            HeroMeadowWaveSvg {}
        }
    }
}

#[component]
fn EventsSection() -> Element {
    let lang = use_context::<Signal<Lang>>()();

    rsx! {
        section { id: "events", class: "chapter-ivory",
            div { class: "section-container",
                div { class: "section-header", "data-reveal": "",
                    h2 { class: "section-heading", "{tr(lang, \"The Wedding Day\", \"വിവാഹ ദിനം\")}" }
                    div { class: "filigree-divider" }
                    p { class: "section-subcopy",
                        "{tr(lang, \"Join Liju & Sharon on Sunday, 24 January 2027 in Vadakkencherry for the Holy Matrimony followed by the Evening Reception.\", \"2027 ജനുവരി 24 ഞായറാഴ്ച വടക്കഞ്ചേരിയിൽ നടക്കുന്ന തിരുവിവാഹത്തിലും തുടർന്നുള്ള സായാഹ്ന വിരുന്നിലും ലിജുവിനോടും ഷാരോണിനോടുമൊപ്പം പങ്കുചേരൂ.\")}"
                    }
                }

                div {
                    class: "arch-events-grid is-double",
                    "data-reveal": "",
                    for event in EVENTS {
                        article { class: "arch-event-card", key: "{event.title}",
                            div { class: "card-top-row",
                                span { class: "numeral-medallion", "{event.numeral}" }
                                span { class: "event-badge", "{tr(lang, event.badge, event.badge_ml)}" }
                            }
                            h3 { "{tr(lang, event.title, event.title_ml)}" }
                            p { class: "event-subtitle", "{tr(lang, event.subtitle, event.subtitle_ml)}" }

                            div { class: "event-meta-list",
                                div { class: "event-meta-row",
                                    span { class: "meta-label", "{tr(lang, \"Date\", \"തീയതി\")}" }
                                    span { class: "meta-value", "{tr(lang, event.date, event.date_ml)}" }
                                }
                                div { class: "event-meta-row",
                                    span { class: "meta-label", "{tr(lang, \"Time\", \"സമയം\")}" }
                                    span { class: "meta-value", "{tr(lang, event.time, event.time_ml)}" }
                                }
                                div { class: "event-meta-row",
                                    span { class: "meta-label", "{tr(lang, \"Venue\", \"വേദി\")}" }
                                    span { class: "meta-value", "{tr(lang, event.venue_area, event.venue_area_ml)}" }
                                }
                            }

                            div { class: "card-link-row",
                                a {
                                    href: "{event.map_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "action-link",
                                    "{tr(lang, \"Open in Google Maps ↗\", \"ഗൂഗിൾ മാപ്പ് തുറക്കുക ↗\")}"
                                }
                                a {
                                    href: event.calendar_ics,
                                    class: "action-link",
                                    "{tr(lang, \"Add to Calendar ↓\", \"കലണ്ടറിൽ ചേർക്കുക ↓\")}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttireSection() -> Element {
    let lang = use_context::<Signal<Lang>>()();

    rsx! {
        section { id: "palette", class: "chapter-attire",
            GhibliBreezeLayer {}

            div { class: "section-container attire-z",
                div { class: "section-header", "data-reveal": "",
                    h2 { class: "section-heading", "{tr(lang, \"What to Wear\", \"വസ്ത്രധാരണം\")}" }
                    div { class: "filigree-divider" }
                    p { class: "section-subcopy",
                        "{tr(lang, \"Celebrate comfortably in breezy Indian & festive attire.\", \"ലളിതവും ആകർഷകവുമായ ഇന്ത്യൻ ഉത്സവ വസ്ത്രങ്ങൾ തിരഞ്ഞെടുക്കാം.\")}"
                    }

                    div { class: "compact-palette-ribbon", aria_label: tr(lang, "Wedding color shades", "വിവാഹ നിറങ്ങൾ"),
                        for swatch in ATTIRE_SWATCHES {
                            span { class: "compact-tone-pill", key: "{swatch.name}",
                                span {
                                    class: "compact-tone-dot",
                                    style: "background-color: {swatch.hex};",
                                }
                                "{tr(lang, swatch.name, swatch.name_ml)}"
                            }
                        }
                    }
                }

                article { class: "ghibli-combined-showcase", "data-reveal": "",
                    div { class: "ghibli-combined-frame",
                        img {
                            src: ATTIRE_GUESTS_GHIBLI,
                            alt: "Vibrant Studio Ghibli wedding celebration scene showing men and women guests together in Pant and Shirt, Pastel Sarees, Mundu and Jubba, Salwar Suits, Pant and Kurta, and Flowing Anarkali",
                            class: "ghibli-combined-img",
                            decoding: "async",
                        }
                        div { class: "ghibli-shimmer-veil", "aria-hidden": "true" }
                        span { class: "venue-mini-petal vp-1", "aria-hidden": "true" }
                        span { class: "venue-mini-petal vp-2", "aria-hidden": "true" }
                        span { class: "venue-mini-petal vp-3", "aria-hidden": "true" }
                        span { class: "venue-firefly-mote fm-1", "aria-hidden": "true" }
                        span { class: "venue-firefly-mote fm-3", "aria-hidden": "true" }
                    }

                    div { class: "combined-guide-columns",
                        div { class: "combined-guide-col",
                            span { class: "group-tag tag-sage", "{tr(lang, \"Gentlemen\", \"പുരുഷന്മാർ\")}" }
                            div { class: "lookbook-style-pills",
                                span { class: "lookbook-pill", "{tr(lang, \"Pant & Shirt\", \"പാന്റും ഷർട്ടും\")}" }
                                span { class: "lookbook-pill", "{tr(lang, \"Pant & Kurta\", \"പാന്റും കുർത്തയും\")}" }
                                span { class: "lookbook-pill", "{tr(lang, \"Mundu & Jubba\", \"മുണ്ടും ജുബ്ബയും\")}" }
                            }
                        }

                        div { class: "combined-guide-col",
                            span { class: "group-tag tag-rose", "{tr(lang, \"Ladies\", \"വനിതകൾ\")}" }
                            div { class: "lookbook-style-pills",
                                span { class: "lookbook-pill is-rose", "{tr(lang, \"Salwar Suits\", \"സൽവാർ സ്യൂട്ട്\")}" }
                                span { class: "lookbook-pill is-rose", "{tr(lang, \"Pastel Sarees\", \"പേസ്റ്റൽ സാരികൾ\")}" }
                                span { class: "lookbook-pill is-rose", "{tr(lang, \"Anarkali / Churidar\", \"അനാർക്കലി / ചുരിദാർ\")}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn SchematicRouteMap() -> Element {
    let lang = use_context::<Signal<Lang>>()();
    let church_copied = use_signal(|| false);
    let reception_copied = use_signal(|| false);

    rsx! {
        div { class: "route-map-shell", "data-reveal": "",
            div { class: "route-map-topbar",
                div { class: "route-map-title-group",
                    span { class: "route-journey-pill", "{tr(lang, \"CHRONOLOGICAL JOURNEY GUIDE\", \"സമയക്രമത്തിലുള്ള യാത്രാ വഴികാട്ടി\")}" }
                    h3 { {tr(lang, "Visual Route: Kochi → Thrissur → Wedding → Reception", "യാത്രാ മാർഗ്ഗം: കൊച്ചി → തൃശ്ശൂർ → വിവാഹം → വിരുന്ന്")} }
                }
                span { class: "route-scale-note", "{tr(lang, \"Illustrative Map • Not to Scale\", \"ഏകദേശ ഭൂപടം • സ്കെയിൽ പ്രകാരമല്ല\")}" }
            }

            div { class: "route-visual-stage",
                svg {
                    class: "route-panoramic-svg",
                    view_box: "0 0 1000 164",
                    xmlns: "http://www.w3.org/2000/svg",
                    "aria-hidden": "true",
                    defs {
                        linearGradient { id: "journey-ribbon-grad", x1: "0%", y1: "0%", x2: "100%", y2: "0%",
                            stop { offset: "0%", "stop-color": "#9caf88" }
                            stop { offset: "38%", "stop-color": "#c9b38c" }
                            stop { offset: "70%", "stop-color": "#d68a98" }
                            stop { offset: "100%", "stop-color": "#b87382" }
                        }
                        linearGradient { id: "meadow-sage-hills", x1: "0%", y1: "0%", x2: "0%", y2: "100%",
                            stop { offset: "0%", "stop-color": "#9caf88", "stop-opacity": "0.32" }
                            stop { offset: "100%", "stop-color": "#fbf6ec", "stop-opacity": "0" }
                        }
                    }

                    /* Gentle hills silhouette */
                    path {
                        d: "M 385 136 Q 452 36 518 86 Q 566 42 628 136 Z",
                        fill: "url(#meadow-sage-hills)",
                    }

                    /* Vertical stems connecting road pins down to the 4 chronological cards */
                    line { x1: "125", y1: "98", x2: "125", y2: "162", stroke: "rgba(156,175,136,0.65)", "stroke-width": "2", "stroke-dasharray": "4 4" }
                    line { x1: "375", y1: "72", x2: "375", y2: "162", stroke: "rgba(184,148,95,0.65)", "stroke-width": "2", "stroke-dasharray": "4 4" }
                    line { x1: "625", y1: "96", x2: "625", y2: "162", stroke: "rgba(214,138,152,0.8)", "stroke-width": "2.5", "stroke-dasharray": "4 4" }
                    line { x1: "875", y1: "70", x2: "875", y2: "162", stroke: "rgba(169,92,107,0.8)", "stroke-width": "2.5", "stroke-dasharray": "4 4" }

                    /* Soft champagne path bed + animated dashed path */
                    path {
                        d: "M 55 104 C 85 104, 100 98, 125 98 C 215 98, 285 72, 375 72 C 465 72, 535 96, 625 96 C 715 96, 785 70, 875 70 L 950 70",
                        fill: "none",
                        stroke: "rgba(184, 148, 95, 0.22)",
                        "stroke-width": "22",
                        "stroke-linecap": "round",
                    }
                    path {
                        class: "ghibli-road-dash",
                        d: "M 55 104 C 85 104, 100 98, 125 98 C 215 98, 285 72, 375 72 C 465 72, 535 96, 625 96 C 715 96, 785 70, 875 70 L 950 70",
                        fill: "none",
                        stroke: "url(#journey-ribbon-grad)",
                        "stroke-width": "3.5",
                        "stroke-dasharray": "10 8",
                        "stroke-linecap": "round",
                    }

                    /* Distance & Schedule Callout Pills */
                    g { transform: "translate(250, 48)",
                        rect { x: "-56", y: "-13", width: "112", height: "24", rx: "12", fill: "#fffdf9", stroke: "rgba(156,175,136,0.65)", "stroke-width": "1.2" }
                        text { x: "0", y: "3", "text-anchor": "middle", fill: "#4e6142", "font-size": "10.5", "font-weight": "700", "font-family": "Nunito, sans-serif", "{tr(lang, \"45 km Drive\", \"45 കി.മീ യാത്ര\")}" }
                    }
                    g { transform: "translate(500, 50)",
                        rect { x: "-56", y: "-13", width: "112", height: "24", rx: "12", fill: "#fffdf9", stroke: "rgba(184,148,95,0.65)", "stroke-width": "1.2" }
                        text { x: "0", y: "3", "text-anchor": "middle", fill: "#7a5e32", "font-size": "10.5", "font-weight": "700", "font-family": "Nunito, sans-serif", "{tr(lang, \"30 km Drive\", \"30 കി.മീ യാത്ര\")}" }
                    }
                    g { transform: "translate(750, 48)",
                        rect { x: "-74", y: "-13", width: "148", height: "24", rx: "12", fill: "#fdf2f4", stroke: "rgba(214,138,152,0.8)", "stroke-width": "1.2" }
                        text { x: "0", y: "3", "text-anchor": "middle", fill: "#8f4252", "font-size": "10.5", "font-weight": "700", "font-family": "Nunito, sans-serif", {tr(lang, "3:00 PM → 7:00 PM (3.5 km)", "3:00 PM → 7:00 PM (3.5 കി.മീ)")} }
                    }

                    /* 4 Glowing Stop Pins */
                    g { transform: "translate(125, 98)",
                        circle { r: "16", fill: "rgba(156,175,136,0.28)" }
                        circle { r: "9.5", fill: "#9caf88", stroke: "#fffdf9", "stroke-width": "2.5" }
                    }
                    g { transform: "translate(375, 72)",
                        circle { r: "16", fill: "rgba(235,214,179,0.55)" }
                        circle { r: "9.5", fill: "#c9a66b", stroke: "#fffdf9", "stroke-width": "2.5" }
                    }
                    g { transform: "translate(625, 96)",
                        circle { r: "20", fill: "rgba(214,138,152,0.32)" }
                        circle { r: "11.5", fill: "#d68a98", stroke: "#fffdf9", "stroke-width": "2.5" }
                    }
                    g { transform: "translate(875, 70)",
                        circle { r: "20", fill: "rgba(169,92,107,0.28)" }
                        circle { r: "11.5", fill: "#a95c6b", stroke: "#fffdf9", "stroke-width": "2.5" }
                    }
                }

                div { class: "route-stops-grid",
                    /* 1. Kochi (COK Airport) */
                    div { class: "route-node is-transit",
                        div { class: "node-venue-photo is-hub-photo",
                            img {
                                src: ROUTE_COK_AIRPORT,
                                alt: "Studio Ghibli illustration of Cochin International Airport (COK) Kerala heritage terminal",
                                decoding: "async",
                            }
                            div { class: "ghibli-shimmer-veil", "aria-hidden": "true" }
                            span { class: "venue-time-ribbon ribbon-sage", "{tr(lang, \"Arrival • COK\", \"ആഗമനം • COK\")}" }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-transit", "✈" }
                            span { class: "node-leg-pill", "{tr(lang, \"Step 01 • Arrival Hub\", \"ഘട്ടം 01 • വിമാനത്താവളം\")}" }
                        }
                        span { class: "node-kicker", "{tr(lang, \"Airport & Metro Hub\", \"എയർപോർട്ട് & മെട്രോ ഹബ്\")}" }
                        h4 { "{tr(lang, \"Kochi (COK)\", \"കൊച്ചി (COK)\")}" }
                        p { class: "node-meta", "{tr(lang, \"75 km • 1 hr 20 mins drive\", \"75 കി.മീ • 1 മണിക്കൂർ 20 മിനിറ്റ്\")}" }
                        p { class: "node-desc",
                            "{tr(lang, \"Arrive at Cochin Airport or Ernakulam and head towards Thrissur and Vadakkencherry.\", \"കൊച്ചി എയർപോർട്ടിലോ എറണാകുളത്തോ എത്തി തൃശ്ശൂർ വഴി വടക്കഞ്ചേരിയിലേക്ക് യാത്ര തിരിക്കാം.\")}"
                        }
                    }

                    /* 2. Thrissur */
                    div { class: "route-node is-transit",
                        div { class: "node-venue-photo is-hub-photo",
                            img {
                                src: ROUTE_THRISSUR,
                                alt: "Studio Ghibli illustration of Thrissur Swaraj Round greenery",
                                decoding: "async",
                            }
                            div { class: "ghibli-shimmer-veil", "aria-hidden": "true" }
                            span { class: "venue-time-ribbon ribbon-champagne", "{tr(lang, \"Stay Hub • TCR\", \"താമസ കേന്ദ്രം • TCR\")}" }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-stay", "⌂" }
                            span { class: "node-leg-pill", "{tr(lang, \"Step 02 • 45 km\", \"ഘട്ടം 02 • 45 കി.മീ\")}" }
                        }
                        span { class: "node-kicker", "{tr(lang, \"Stay & Rail Hub (TCR)\", \"താമസം & റെയിൽ ഹബ് (TCR)\")}" }
                        h4 { "{tr(lang, \"Thrissur\", \"തൃശ്ശൂർ\")}" }
                        p { class: "node-meta", "{tr(lang, \"30 km to venues • 35 mins\", \"വേദികളിലേക്ക് 30 കി.മീ • 35 മിനിറ്റ്\")}" }
                        p { class: "node-desc",
                            "{tr(lang, \"Ideal base for Airbnbs & family hotels, 35 minutes from Vadakkencherry.\", \"വടക്കഞ്ചേരിയിൽ നിന്ന് 35 മിനിറ്റ് അകലെ എയർബിഎൻബികൾക്കും ഹോട്ടലുകൾക്കും ഏറ്റവും അനുയോജ്യമായ സ്ഥലം.\")}"
                        }
                    }

                    /* 3. FIRST EVENT (3:00 PM): Lourde Matha Forane Church — Animated Ghibli Scene */
                    div { class: "route-node is-highlight-church",
                        div { class: "node-venue-photo is-animated-church",
                            img {
                                src: CHURCH_GHIBLI,
                                alt: "Animated Studio Ghibli illustration of Lourde Matha Forane Church in Vadakkencherry dressed for the 3:00 PM wedding",
                                decoding: "async",
                            }
                            div { class: "church-sunbeam-veil", "aria-hidden": "true" }
                            span { class: "venue-mini-petal vp-1", "aria-hidden": "true" }
                            span { class: "venue-mini-petal vp-2", "aria-hidden": "true" }
                            span { class: "venue-mini-petal vp-3", "aria-hidden": "true" }
                            span { class: "venue-time-ribbon ribbon-rose", {tr(lang, "3:00 PM Wedding", "3:00 PM വിവാഹം")} }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-church", "✟" }
                            span { class: "node-leg-pill pill-rose", "{tr(lang, \"Ceremony Venue\", \"വിവാഹ വേദി\")}" }
                        }
                        h4 { "{tr(lang, CHURCH_NAME, \"ലൂർദ് മാതാ ഫൊറോന പള്ളി\")}" }
                        p { class: "node-meta", "{tr(lang, \"Vadakkencherry\", \"വടക്കഞ്ചേരി\")}" }
                        p { class: "node-desc",
                            {tr(lang, "Join us at 3:00 PM for the Solemn Nuptial Mass & wedding vows.", "3:00 PM-ന് തിരുവിവാഹ കുർബാനയിലും പ്രതിജ്ഞാ ചടങ്ങിലും പങ്കുചേരാം.")}
                        }
                        div { class: "node-action-stack",
                            a {
                                href: "{CHURCH_MAP_URL}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "node-map-link is-primary",
                                "{tr(lang, \"Church Directions ↗\", \"പള്ളിയിലേക്കുള്ള വഴി ↗\")}"
                            }
                            button {
                                r#type: "button",
                                class: if church_copied() { "node-copy-btn is-copied" } else { "node-copy-btn" },
                                onclick: move |_| {
                                    copy_with_feedback(
                                        format!("{CHURCH_NAME}, {CHURCH_ADDRESS}"),
                                        church_copied,
                                    );
                                },
                                if church_copied() {
                                    "{tr(lang, \"Address Copied ✓\", \"വിലാസം പകർത്തി ✓\")}"
                                } else {
                                    "{tr(lang, \"Copy Address\", \"വിലാസം പകർത്തുക\")}"
                                }
                            }
                        }
                    }

                    /* 4. SECOND EVENT (7:00 PM): TMK Arena — Animated Ghibli Twilight Scene */
                    div { class: "route-node is-highlight-reception",
                        div { class: "node-venue-photo is-animated-reception",
                            img {
                                src: RECEPTION_GHIBLI,
                                alt: "Animated Studio Ghibli twilight illustration of TMK Arena in Vadakkencherry",
                                decoding: "async",
                            }
                            div { class: "reception-lantern-glow", "aria-hidden": "true" }
                            span { class: "venue-firefly-mote fm-1", "aria-hidden": "true" }
                            span { class: "venue-firefly-mote fm-2", "aria-hidden": "true" }
                            span { class: "venue-firefly-mote fm-3", "aria-hidden": "true" }
                            span { class: "venue-firefly-mote fm-4", "aria-hidden": "true" }
                            span { class: "venue-time-ribbon ribbon-champagne", {tr(lang, "7:00 PM Reception", "7:00 PM വിരുന്ന്")} }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-reception", "★" }
                            span { class: "node-leg-pill pill-warm", "{tr(lang, \"3.5 km (6 mins) from Church\", \"പള്ളിയിൽ നിന്ന് 3.5 കി.മീ (6 മിനിറ്റ്)\")}" }
                        }
                        h4 { "{tr(lang, RECEPTION_NAME, \"ടി.എം.കെ അരീന (TMK Arena)\")}" }
                        p { class: "node-meta", "{tr(lang, \"Vadakkencherry\", \"വടക്കഞ്ചേരി\")}" }
                        p { class: "node-desc",
                            {tr(lang, "Gather from 7:00 PM onwards for dinner & evening celebration.", "7:00 PM മുതൽ സായാഹ്ന വിരുന്നിനും ആഘോഷങ്ങൾക്കുമായി ഒത്തുചേരാം.")}
                        }
                        div { class: "node-action-stack",
                            a {
                                href: "{RECEPTION_MAP_URL}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "node-map-link is-primary",
                                "{tr(lang, \"Reception Directions ↗\", \"വിരുന്ന് വേദിയിലേക്കുള്ള വഴി ↗\")}"
                            }
                            button {
                                r#type: "button",
                                class: if reception_copied() { "node-copy-btn is-copied" } else { "node-copy-btn" },
                                onclick: move |_| {
                                    copy_with_feedback(
                                        format!("{RECEPTION_NAME}, {RECEPTION_ADDRESS}"),
                                        reception_copied,
                                    );
                                },
                                if reception_copied() {
                                    "{tr(lang, \"Address Copied ✓\", \"വിലാസം പകർത്തി ✓\")}"
                                } else {
                                    "{tr(lang, \"Copy Address\", \"വിലാസം പകർത്തുക\")}"
                                }
                            }
                        }
                    }
                }
            }

            div { class: "route-callout-bar",
                span { class: "callout-pill", {tr(lang, "3:00 PM Church → 7:00 PM Reception", "3:00 PM വിവാഹം → 7:00 PM വിരുന്ന്")} }
                if lang == Lang::Ml {
                    p {
                        "രണ്ട് വേദികളും തമ്മിൽ "
                        strong { "3.5 കി.മീ (5–8 മിനിറ്റ്) ദൂരം മാത്രം" }
                        "—3:00 PM-ന് ലൂർദ് മാതാ ഫൊറോന പള്ളിയിൽ വിവാഹച്ചടങ്ങും, തുടർന്ന് 7:00 PM മുതൽ TMK അരീനയിൽ സായാഹ്ന വിരുന്നും."
                    }
                } else {
                    p {
                        "Both venues are just "
                        strong { "3.5 km (5–8 minutes) apart" }
                        "—celebrate the wedding vows at Lourde Matha Forane Church at 3:00 PM, followed by the evening reception at TMK Arena (7:00 PM onwards)."
                    }
                }
            }
        }
    }
}

#[component]
fn VenueAndTravelSection() -> Element {
    let lang = use_context::<Signal<Lang>>()();

    rsx! {
        section { id: "venue", class: "chapter-venues",
            div { class: "section-container",
                div { class: "section-header", "data-reveal": "",
                    h2 { class: "section-heading", "{tr(lang, \"The Way to Vadakkencherry\", \"വടക്കഞ്ചേരിയിലേക്കുള്ള വഴി\")}" }
                    div { class: "filigree-divider" }
                    p { class: "section-subcopy",
                        {tr(lang, "Follow the journey from Kochi and Thrissur to the 3:00 PM Church Ceremony and 7:00 PM Evening Reception.", "കൊച്ചിയിൽ നിന്നും തൃശ്ശൂരിൽ നിന്നും 3:00 PM പള്ളിയിലെ വിവാഹത്തിലേക്കും 7:00 PM വിരുന്നിലേക്കുമുള്ള യാത്രാ മാർഗ്ഗം.")}
                    }
                }

                SchematicRouteMap {}

                div { id: "stay-travel", class: "stay-travel-wrap", "data-reveal": "",
                    div { class: "section-header header-compact",
                        h2 { class: "section-heading", "{tr(lang, \"Where to Stay & Getting Around\", \"താമസവും യാത്രാ സൗകര്യങ്ങളും\")}" }
                        div { class: "filigree-divider" }
                    }

                    div { class: "stay-commute-grid",
                        article { class: "highlight-rose-card", "data-reveal": "", "data-delay": "1",
                            span { class: "bento-kicker kicker-rose", "{tr(lang, \"Stay Options\", \"താമസ സൗകര്യങ്ങൾ\")}" }
                            h3 { "{tr(lang, \"Airbnbs & Hotels Nearby\", \"സമീപത്തുള്ള എയർബിഎൻബിയും ഹോട്ടലുകളും\")}" }
                            if lang == Lang::Ml {
                                p {
                                    "ദൂരെ നിന്ന് വരുന്ന അതിഥികൾക്ക് "
                                    strong { "തൃശ്ശൂർ, പാലക്കാട്, അല്ലെങ്കിൽ വടക്കഞ്ചേരി എന്നിവിടങ്ങളിൽ എയർബിഎൻബിയോ ഹോട്ടലുകളോ" }
                                    " ബുക്ക് ചെയ്യാവുന്നതാണ്:"
                                }
                                ul { class: "friendly-list",
                                    li {
                                        strong { "വടക്കഞ്ചേരി: " }
                                        "വേദികൾക്ക് ഏറ്റവും അടുത്തുള്ള ഹോട്ടലുകളും ഹോംസ്റ്റേകളും (5–10 മിനിറ്റ്)."
                                    }
                                    li {
                                        strong { "തൃശ്ശൂർ (30 കി.മീ / 35 മിനിറ്റ്): " }
                                        "ധാരാളം എയർബിഎൻബികൾ, ബൂട്ടിക് സ്റ്റേകൾ, ഫാമിലി ഹോട്ടലുകൾ."
                                    }
                                    li {
                                        strong { "പാലക്കാട് (35 കി.മീ / 40 മിനിറ്റ്): " }
                                        "മികച്ച ഹോട്ടലുകളും റിസോർട്ടുകളും എളുപ്പത്തിൽ എത്താവുന്ന ദൂരത്തിൽ."
                                    }
                                }
                            } else {
                                p {
                                    "Out-of-town guests can comfortably book "
                                    strong { "Airbnbs or hotels in Thrissur, Palakkad, or Vadakkencherry" }
                                    ":"
                                }
                                ul { class: "friendly-list",
                                    li {
                                        strong { "Vadakkencherry: " }
                                        "Closest hotels & homestays (5–10 mins to both venues)."
                                    }
                                    li {
                                        strong { "Thrissur (30 km / 35 mins): " }
                                        "Wide selection of Airbnbs, boutique stays & family hotels."
                                    }
                                    li {
                                        strong { "Palakkad (35 km / 40 mins): " }
                                        "Great hotel chains, Airbnbs & resorts within an easy drive."
                                    }
                                }
                            }
                        }

                        article { class: "highlight-sage-card", "data-reveal": "", "data-delay": "2",
                            span { class: "bento-kicker", "{tr(lang, \"Local Commute Tip\", \"യാത്രാ നിർദ്ദേശം\")}" }
                            h3 { "{tr(lang, \"Uber & Car Rentals\", \"ഊബർ & കാർ റെന്റൽ\")}" }
                            if lang == Lang::Ml {
                                p {
                                    "തൃശ്ശൂർ, പാലക്കാട് എന്നിവിടങ്ങളിൽ താമസിക്കുന്നവരോ ഫ്ലൈറ്റ്/ട്രെയിൻ വഴി എത്തുന്നവരോ ആണെങ്കിൽ സുഗമമായ യാത്രയ്ക്കായി "
                                    strong { "ഊബർ, ലോക്കൽ ക്യാബ്, അല്ലെങ്കിൽ സെൽഫ്-ഡ്രൈവ് / ഡ്രൈവറോടുകൂടിയ കാർ റെന്റൽ" }
                                    " തിരഞ്ഞെടുക്കാവുന്നതാണ്."
                                }
                                ul { class: "friendly-list",
                                    li {
                                        strong { "എളുപ്പമുള്ള യാത്ര: " }
                                        "നിങ്ങളുടെ ഹോട്ടലിൽ നിന്നും ലൂർദ് മാതാ ഫൊറോന പള്ളിയിലേക്കും തുടർന്ന് TMK അരീനയിലേക്കും സുഖമായി എത്താം."
                                    }
                                    li {
                                        strong { "വേദികൾ തമ്മിലുള്ള ദൂരം: " }
                                        "പള്ളിയും TMK അരീനയും തമ്മിൽ 3.5 കി.മീ (5–8 മിനിറ്റ്) ദൂരം മാത്രമാണുള്ളത്."
                                    }
                                    li {
                                        strong { "സഹായം ആവശ്യമുണ്ടോ? " }
                                        "താഴെയുള്ള RSVP-യിൽ നിങ്ങളുടെ എത്തുന്ന സമയം രേഖപ്പെടുത്തിയാൽ ഞങ്ങളുടെ കുടുംബം സഹായിക്കുന്നതാണ്."
                                    }
                                }
                            } else {
                                p {
                                    "If you are staying in Thrissur, Palakkad, or arriving by flight/train, you may want to book an "
                                    strong { "Uber, local cab, or self-drive / chauffeured car rental" }
                                    " for easy commuting."
                                }
                                ul { class: "friendly-list",
                                    li {
                                        strong { "Door-to-Door Ease: " }
                                        "Smoothly travel from your Airbnb/hotel to Lourde Matha Forane Church and onto TMK Arena."
                                    }
                                    li {
                                        strong { "Between Venues: " }
                                        "The church and TMK Arena are only 3.5 km (5–8 mins) apart."
                                    }
                                    li {
                                        strong { "Need Help Arranging? " }
                                        "Drop your arrival time in the RSVP below and our family can help coordinate."
                                    }
                                }
                            }
                        }
                    }
                }

                SmartJourneyPlanner {}
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PlannerPhase {
    Arrival,
    Exit,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ArrivalChannel {
    FlightKochi,
    TrainHubs,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FlightPath {
    ViaThrissurStay,
    DirectToChurch,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TrainGateway {
    Palakkad,
    Kochi,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExitOrigin {
    TmkArena,
    Church,
    ThrissurStay,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExitTarget {
    KochiAirport,
    PalakkadStation,
    ThrissurKochiRail,
}

#[component]
fn SmartJourneyPlanner() -> Element {
    let lang = use_context::<Signal<Lang>>()();
    let mut phase = use_signal(|| PlannerPhase::Arrival);
    let mut arrival_channel = use_signal(|| ArrivalChannel::FlightKochi);
    let mut flight_path = use_signal(|| FlightPath::ViaThrissurStay);
    let mut train_gateway = use_signal(|| TrainGateway::Palakkad);
    let mut exit_origin = use_signal(|| ExitOrigin::TmkArena);
    let mut exit_target = use_signal(|| ExitTarget::KochiAirport);

    let origin_query = match exit_origin() {
        ExitOrigin::TmkArena => "TMK+Arena,+Vadakkencherry,+Kerala+678683",
        ExitOrigin::Church => "Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683",
        ExitOrigin::ThrissurStay => "Thrissur,+Kerala",
    };

    let target_query = match exit_target() {
        ExitTarget::KochiAirport => "Cochin+International+Airport,+Kochi,+Kerala",
        ExitTarget::PalakkadStation => "Palakkad+Junction+Railway+Station,+Palakkad,+Kerala",
        ExitTarget::ThrissurKochiRail => "Thrissur+Railway+Station,+Thrissur,+Kerala",
    };

    let exit_drive_url = format!(
        "https://www.google.com/maps/dir/?api=1&origin={origin_query}&destination={target_query}&travelmode=driving"
    );
    let exit_transit_url = format!(
        "https://www.google.com/maps/dir/?api=1&origin={origin_query}&destination={target_query}&travelmode=transit"
    );

    let origin_short = match exit_origin() {
        ExitOrigin::TmkArena => tr(lang, "TMK Arena (Reception)", "TMK അരീന (വിരുന്ന് വേദി)"),
        ExitOrigin::Church => tr(lang, "Lourde Matha Church", "ലൂർദ് മാതാ പള്ളി"),
        ExitOrigin::ThrissurStay => tr(lang, "Thrissur Hotels", "തൃശ്ശൂർ ഹോട്ടലുകൾ"),
    };

    let target_short = match exit_target() {
        ExitTarget::KochiAirport => tr(lang, "Cochin Airport (COK)", "കൊച്ചി എയർപോർട്ട് (COK)"),
        ExitTarget::PalakkadStation => tr(lang, "Palakkad Jn (PGT)", "പാലക്കാട് ജംഗ്ഷൻ (PGT)"),
        ExitTarget::ThrissurKochiRail => tr(
            lang,
            "Thrissur (TCR) / Kochi (ERS)",
            "തൃശ്ശൂർ (TCR) / കൊച്ചി (ERS)",
        ),
    };

    let (exit_distance, exit_heading, exit_opt_primary, exit_opt_secondary, exit_tip) =
        match (exit_origin(), exit_target()) {
            (ExitOrigin::TmkArena, ExitTarget::KochiAirport) => (
                tr(lang, "72 km • 1 hr 20 mins", "72 കി.മീ • 1 മണിക്കൂർ 20 മിനിറ്റ്"),
                tr(
                    lang,
                    "Reception (TMK Arena) → Cochin International Airport (COK)",
                    "TMK അരീന (വിരുന്ന്) → കൊച്ചി അന്താരാഷ്ട്ര വിമാനത്താവളം (COK)",
                ),
                tr(
                    lang,
                    "Uber / Pre-booked Cab / Rental Drop-off: Direct 1 hr 20 min drive via Thrissur right to the COK Departure Terminal.",
                    "ഊബർ / മുൻകൂട്ടി ബുക്ക് ചെയ്ത ക്യാബ് / റെന്റൽ കാർ: തൃശ്ശൂർ വഴി 1 മണിക്കൂർ 20 മിനിറ്റിൽ നേരിട്ട് COK ടെർമിനലിൽ എത്താം.",
                ),
                tr(
                    lang,
                    "Via Thrissur Rail + Feeder: 40-min cab to Thrissur (TCR), express train to Aluva (AWY), and a 20-min airport feeder cab.",
                    "തൃശ്ശൂർ വഴി ട്രെയിൻ: 40 മിനിറ്റിൽ തൃശ്ശൂർ (TCR), അവിടെ നിന്ന് ആലുവയിലേക്ക് (AWY) ട്രെയിൻ + 20 മിനിറ്റ് എയർപോർട്ട് ക്യാബ്.",
                ),
                tr(
                    lang,
                    "Tip: Pre-book your return cab before 8:30 PM and leave 3.5–4 hours ahead of domestic flights (4.5–5 hours for international).",
                    "സൂചന: രാത്രി മടങ്ങുന്നവർ 8:30 PM-ന് മുൻപ് ക്യാബ് ബുക്ക് ചെയ്യുക; ഫ്ലൈറ്റിന് 3.5–4 മണിക്കൂർ മുൻപെങ്കിലും തിരിക്കുക.",
                ),
            ),
            (ExitOrigin::Church, ExitTarget::KochiAirport) => (
                tr(lang, "75 km • 1 hr 25 mins", "75 കി.മീ • 1 മണിക്കൂർ 25 മിനിറ്റ്"),
                tr(
                    lang,
                    "Lourde Matha Forane Church → Cochin International Airport (COK)",
                    "ലൂർദ് മാതാ ഫൊറോന പള്ളി → കൊച്ചി വിമാനത്താവളം (COK)",
                ),
                tr(
                    lang,
                    "Direct Uber / Local Cab / Car Rental: Smooth 1 hr 25 min drive from Vadakkencherry via Thrissur to COK Airport.",
                    "നേരിട്ടുള്ള ഊബർ / ക്യാബ് / റെന്റൽ കാർ: വടക്കഞ്ചേരിയിൽ നിന്ന് തൃശ്ശൂർ വഴി 1 മണിക്കൂർ 25 മിനിറ്റിൽ കൊച്ചി എയർപോർട്ടിലെത്താം.",
                ),
                tr(
                    lang,
                    "Bus / Train Option: KSRTC bus or cab to Thrissur (35 mins), then low-floor airport AC bus or train to Aluva (AWY).",
                    "ബസ് / ട്രെയിൻ മാർഗ്ഗം: 35 മിനിറ്റിൽ തൃശ്ശൂർ എത്തി, അവിടെ നിന്ന് എയർപോർട്ട് എസി ബസോ ആലുവയിലേക്കുള്ള ട്രെയിനോ തിരഞ്ഞെടുക്കാം.",
                ),
                tr(
                    lang,
                    "Tip: Allow 3.5–4 hours total buffer before flight departure time.",
                    "സൂചന: ഫ്ലൈറ്റ് സമയത്തിന് 3.5–4 മണിക്കൂർ മുൻപായി യാത്ര തിരിക്കുന്നതാണ് ഉചിതം.",
                ),
            ),
            (ExitOrigin::ThrissurStay, ExitTarget::KochiAirport) => (
                tr(lang, "52 km • 1 hr 10 mins", "52 കി.മീ • 1 മണിക്കൂർ 10 മിനിറ്റ്"),
                tr(
                    lang,
                    "Thrissur Hotels & Airbnbs → Cochin International Airport (COK)",
                    "തൃശ്ശൂർ ഹോട്ടലുകൾ / എയർബിഎൻബി → കൊച്ചി വിമാനത്താവളം (COK)",
                ),
                tr(
                    lang,
                    "Morning Uber / Hotel Taxi: Easy 1 hr 10 min ride directly from your Thrissur stay to COK Departures.",
                    "ഊബർ / ഹോട്ടൽ ടാക്സി: തൃശ്ശൂരിലെ താമസസ്ഥലത്തുനിന്ന് 1 മണിക്കൂർ 10 മിനിറ്റിൽ നേരിട്ട് എയർപോർട്ടിലെത്താം.",
                ),
                tr(
                    lang,
                    "Train to Aluva (AWY) + Feeder: 45-min train from Thrissur (TCR) to Aluva, followed by a 20-min airport feeder bus or cab.",
                    "ആലുവ വഴി ട്രെയിൻ: തൃശ്ശൂരിൽ (TCR) നിന്ന് ആലുവയിലേക്ക് (AWY) 45 മിനിറ്റ് ട്രെയിൻ + 20 മിനിറ്റ് എയർപോർട്ട് ഫീഡർ ക്യാബ്/ബസ്.",
                ),
                tr(
                    lang,
                    "Tip: Checking out Monday morning (Jan 25) makes for a relaxed 70-minute airport transfer.",
                    "സൂചന: തിങ്കളാഴ്ച രാവിലെ (ജനുവരി 25) ചെക്ക്-ഔട്ട് ചെയ്ത് സൗകര്യപ്രദമായി എയർപോർട്ടിലേക്ക് മടങ്ങാം.",
                ),
            ),
            (ExitOrigin::TmkArena, ExitTarget::PalakkadStation) => (
                tr(lang, "33 km • 38–45 mins", "33 കി.മീ • 38–45 മിനിറ്റ്"),
                tr(
                    lang,
                    "Reception (TMK Arena) → Palakkad Junction Railway Station (PGT)",
                    "TMK അരീന (വിരുന്ന്) → പാലക്കാട് ജംഗ്ഷൻ റെയിൽവേ സ്റ്റേഷൻ (PGT)",
                ),
                tr(
                    lang,
                    "Direct Uber / Pre-booked Cab: Quick 40-minute eastbound ride right to Palakkad Junction (PGT) platform entrance.",
                    "നേരിട്ടുള്ള ഊബർ / ക്യാബ്: വിരുന്ന് കഴിഞ്ഞ് 40 മിനിറ്റിനുള്ളിൽ നേരിട്ട് പാലക്കാട് ജംഗ്ഷൻ (PGT) സ്റ്റേഷനിലെത്താം.",
                ),
                tr(
                    lang,
                    "KSRTC / Fast Bus: Frequent buses from Vadakkencherry to Palakkad Stand, followed by a 10-min auto/cab to PGT.",
                    "കെ.എസ്.ആർ.ടി.സി / ഫാസ്റ്റ് ബസ്: വടക്കഞ്ചേരിയിൽ നിന്ന് പാലക്കാട്ടേക്ക് തുടർച്ചയായി ബസ് സൗകര്യം ലഭ്യമാണ്.",
                ),
                tr(
                    lang,
                    "Tip: Ideal for catching late-night overnight express trains towards Bengaluru, Chennai & Coimbatore.",
                    "സൂചന: ബെംഗളൂരു, ചെന്നൈ, കോയമ്പത്തൂർ ഭാഗത്തേക്കുള്ള രാത്രികാല ട്രെയിനുകൾ പിടിക്കാൻ ഏറ്റവും സൗകര്യപ്രദം.",
                ),
            ),
            (ExitOrigin::Church, ExitTarget::PalakkadStation) => (
                tr(lang, "35 km • 40–45 mins", "35 കി.മീ • 40–45 മിനിറ്റ്"),
                tr(
                    lang,
                    "Lourde Matha Forane Church → Palakkad Junction (PGT)",
                    "ലൂർദ് മാതാ ഫൊറോന പള്ളി → പാലക്കാട് ജംഗ്ഷൻ (PGT)",
                ),
                tr(
                    lang,
                    "Direct Cab / Uber / Rental: 40–45 minutes directly from the church in Vadakkencherry to Palakkad Junction (PGT).",
                    "നേരിട്ടുള്ള ക്യാബ് / ഊബർ: വടക്കഞ്ചേരി പള്ളിയിൽ നിന്ന് 40–45 മിനിറ്റിൽ പാലക്കാട് ജംഗ്ഷനിലെത്താം.",
                ),
                tr(
                    lang,
                    "Direct Bus: Board any Palakkad-bound KSRTC or fast bus at Vadakkencherry (5 mins from the church).",
                    "ബസ് മാർഗ്ഗം: വടക്കഞ്ചേരിയിൽ നിന്ന് പാലക്കാട്ടേക്കുള്ള നേരിട്ടുള്ള ബസുകളിൽ വേഗത്തിൽ എത്താം.",
                ),
                tr(
                    lang,
                    "Tip: Keep a 75-minute buffer before your train's scheduled departure at PGT.",
                    "സൂചന: ട്രെയിൻ പുറപ്പെടുന്നതിന് 75 മിനിറ്റ് മുൻപെങ്കിലും വടക്കഞ്ചേരിയിൽ നിന്ന് തിരിക്കുക.",
                ),
            ),
            (ExitOrigin::ThrissurStay, ExitTarget::PalakkadStation) => (
                tr(lang, "65 km • 1 hr 10–15 mins", "65 കി.മീ • 1 മണിക്കൂർ 10–15 മിനിറ്റ്"),
                tr(
                    lang,
                    "Thrissur Hotels → Palakkad Junction Railway Station (PGT)",
                    "തൃശ്ശൂർ ഹോട്ടലുകൾ → പാലക്കാട് ജംഗ്ഷൻ റെയിൽവേ സ്റ്റേഷൻ (PGT)",
                ),
                tr(
                    lang,
                    "Direct Train (TCR → PGT): Hop a 1 hr 05 min express train from Thrissur Station straight into Palakkad Junction.",
                    "നേരിട്ടുള്ള ട്രെയിൻ (TCR → PGT): തൃശ്ശൂർ സ്റ്റേഷനിൽ നിന്ന് 1 മണിക്കൂർ 05 മിനിറ്റിൽ ട്രെയിൻ വഴി പാലക്കാട് എത്താം.",
                ),
                tr(
                    lang,
                    "Direct Cab / Uber: 1 hr 15 min drive from your Thrissur stay to Palakkad Junction (PGT).",
                    "നേരിട്ടുള്ള ക്യാബ് / ഊബർ: തൃശ്ശൂരിൽ നിന്ന് 1 മണിക്കൂർ 15 മിനിറ്റ് ഡ്രൈവ് ചെയ്ത് പാലക്കാട് ജംഗ്ഷനിലെത്താം.",
                ),
                tr(
                    lang,
                    "Tip: Check if your return train also boards directly at Thrissur (TCR) to save the extra transfer!",
                    "സൂചന: നിങ്ങളുടെ മടക്ക ട്രെയിനിന് തൃശ്ശൂരിൽ (TCR) സ്റ്റോപ്പുണ്ടോ എന്ന് പരിശോധിക്കുന്നത് സമയം ലാഭിക്കാൻ സഹായിക്കും.",
                ),
            ),
            (ExitOrigin::TmkArena, ExitTarget::ThrissurKochiRail) => (
                tr(lang, "32 km to TCR • 85 km to Kochi", "തൃശ്ശൂർ 32 കി.മീ • കൊച്ചി 85 കി.മീ"),
                tr(
                    lang,
                    "Reception (TMK Arena) → Thrissur Station (TCR) & Kochi (ERS)",
                    "TMK അരീന (വിരുന്ന്) → തൃശ്ശൂർ സ്റ്റേഷൻ (TCR) & കൊച്ചി (ERS)",
                ),
                tr(
                    lang,
                    "Cab / Uber to Thrissur (TCR): 40-minute ride to Thrissur Railway Station or back to your Thrissur hotel/Airbnb.",
                    "തൃശ്ശൂരിലേക്ക് ക്യാബ് / ഊബർ: 40 മിനിറ്റിൽ തൃശ്ശൂർ റെയിൽവേ സ്റ്റേഷനിലോ ഹോട്ടലിലോ തിരിച്ചെത്താം.",
                ),
                tr(
                    lang,
                    "Onward Train to Kochi (ERS / ERN): Frequent 1 hr 15 min trains connect Thrissur (TCR) south to Ernakulam & Trivandrum.",
                    "കൊച്ചിയിലേക്ക് തുടർ ട്രെയിൻ: തൃശ്ശൂരിൽ (TCR) നിന്ന് എറണാകുളത്തേക്കും തിരുവനന്തപുരത്തേക്കും തുടർച്ചയായി ട്രെയിനുകളുണ്ട്.",
                ),
                tr(
                    lang,
                    "Tip: Thrissur (TCR) halts almost all southbound and northbound express trains in Kerala.",
                    "സൂചന: കേരളത്തിലെ മിക്കവാറും എല്ലാ എക്സ്പ്രസ് ട്രെയിനുകൾക്കും തൃശ്ശൂർ (TCR) സ്റ്റേഷനിൽ സ്റ്റോപ്പുണ്ട്.",
                ),
            ),
            (ExitOrigin::Church, ExitTarget::ThrissurKochiRail) => (
                tr(lang, "30 km to TCR • 35 mins", "തൃശ്ശൂർ (TCR) 30 കി.മീ • 35 മിനിറ്റ്"),
                tr(
                    lang,
                    "Lourde Matha Forane Church → Thrissur (TCR) & Kochi (ERS)",
                    "ലൂർദ് മാതാ ഫൊറോന പള്ളി → തൃശ്ശൂർ (TCR) & കൊച്ചി (ERS)",
                ),
                tr(
                    lang,
                    "Uber / Cab / Bus to Thrissur (TCR): 35-minute ride west to Thrissur Railway Station.",
                    "തൃശ്ശൂരിലേക്ക് ഊബർ / ക്യാബ് / ബസ്: 35 മിനിറ്റിൽ തൃശ്ശൂർ റെയിൽവേ സ്റ്റേഷനിലെത്താം.",
                ),
                tr(
                    lang,
                    "Onward to Kochi (ERS): Board a 1 hr 15 min train from TCR to Ernakulam, or continue by cab (1 hr 50 mins total).",
                    "കൊച്ചിയിലേക്ക് (ERS): തൃശ്ശൂരിൽ നിന്ന് 1 മണിക്കൂർ 15 മിനിറ്റ് ട്രെയിൻ മാർഗ്ഗമോ ക്യാബിലോ എറണാകുളത്തെത്താം.",
                ),
                tr(
                    lang,
                    "Tip: Frequent KSRTC buses from Vadakkencherry drop right near Thrissur Railway Station.",
                    "സൂചന: വടക്കഞ്ചേരിയിൽ നിന്ന് തൃശ്ശൂർ സ്റ്റേഷൻ പരിസരത്തേക്ക് എപ്പോഴും ബസുകൾ ലഭ്യമാണ്.",
                ),
            ),
            (ExitOrigin::ThrissurStay, ExitTarget::ThrissurKochiRail) => (
                tr(lang, "10 mins to TCR • 1 hr 15m to ERS", "TCR-ലേക്ക് 10 മിനിറ്റ് • കൊച്ചി 1 മണിക്കൂർ 15 മി."),
                tr(
                    lang,
                    "Thrissur Hotels → Thrissur Station (TCR) / Kochi (ERS)",
                    "തൃശ്ശൂർ ഹോട്ടലുകൾ → തൃശ്ശൂർ സ്റ്റേഷൻ (TCR) / കൊച്ചി (ERS)",
                ),
                tr(
                    lang,
                    "Board Directly at Thrissur (TCR): Just a 5–10 minute Uber/auto hop from Thrissur hotels to the railway station.",
                    "തൃശ്ശൂർ സ്റ്റേഷനിൽ (TCR) നിന്ന് കയറാം: ഹോട്ടലുകളിൽ നിന്ന് 5–10 മിനിറ്റ് യാത്ര മാത്രമേ സ്റ്റേഷനിലേക്കുള്ളൂ.",
                ),
                tr(
                    lang,
                    "Train to Kochi (ERS / ERN): Vande Bharat, Jan Shatabdi & Intercity trains reach Kochi in 1 hr to 1 hr 15 mins.",
                    "കൊച്ചിയിലേക്ക് ട്രെയിൻ: വന്ദേ ഭാരത്, ജനശതാബ്ദി, ഇന്റർസിറ്റി ട്രെയിനുകളിൽ 1 മണിക്കൂറിനുള്ളിൽ കൊച്ചിയിലെത്താം.",
                ),
                tr(
                    lang,
                    "Tip: Morning trains from TCR to Ernakulam run every 30–45 minutes.",
                    "സൂചന: രാവിലെ തൃശ്ശൂരിൽ നിന്ന് എറണാകുളത്തേക്ക് 30–45 മിനിറ്റ് ഇടവേളയിൽ ട്രെയിനുകൾ ലഭ്യമാണ്.",
                ),
            ),
        };

    rsx! {
        div { id: "journey-planner", class: "smart-planner-wrap", "data-reveal": "",
            div { class: "section-header header-compact",
                span { class: "route-journey-pill", {tr(lang, "INTERACTIVE GOOGLE MAPS PLANNER", "ഗൂഗിൾ മാപ്പ് യാത്രാ പ്ലാനർ")} }
                h2 { class: "section-heading", {tr(lang, "Smart Journey Planner", "സ്മാർട്ട് യാത്രാ പ്ലാനർ")} }
                div { class: "filigree-divider" }
                p { class: "section-subcopy",
                    {tr(
                        lang,
                        "Plan your arrival by flight or train—and map your return journey to Kochi Airport or Palakkad Station in one tap.",
                        "വിമാനത്തിലോ ട്രെയിനിലോ എത്താനുള്ള വഴികളും, വിവാഹശേഷം കൊച്ചി എയർപോർട്ടിലേക്കോ പാലക്കാട് സ്റ്റേഷനിലേക്കോ മടങ്ങാനുള്ള മാർഗ്ഗങ്ങളും പ്ലാൻ ചെയ്യാം.",
                    )}
                }
            }

            div { class: "planner-shell",
                /* Top Direction Switcher: Arrival vs Return / Exit */
                div {
                    class: "planner-phase-bar",
                    role: "tablist",
                    aria_label: tr(lang, "Journey direction", "യാത്രാ ദിശ"),
                    button {
                        r#type: "button",
                        role: "tab",
                        aria_selected: if phase() == PlannerPhase::Arrival { "true" } else { "false" },
                        class: if phase() == PlannerPhase::Arrival { "planner-phase-btn is-active" } else { "planner-phase-btn" },
                        onclick: move |_| phase.set(PlannerPhase::Arrival),
                        span { class: "phase-icon", "✦" }
                        span { {tr(lang, "Arrival to Wedding", "വിവാഹത്തിലേക്കുള്ള വരവ്")} }
                    }
                    button {
                        r#type: "button",
                        role: "tab",
                        aria_selected: if phase() == PlannerPhase::Exit { "true" } else { "false" },
                        class: if phase() == PlannerPhase::Exit { "planner-phase-btn is-active is-exit-active" } else { "planner-phase-btn" },
                        onclick: move |_| phase.set(PlannerPhase::Exit),
                        span { class: "phase-icon", "↩" }
                        span { {tr(lang, "Return / Exit Journey", "മടക്കയാത്ര (Exit)")} }
                    }
                }

                if phase() == PlannerPhase::Arrival {
                    div { class: "planner-body",
                        /* Primary Mode Tabs: 1. Flight - Kochi | 2. Train - Kochi & Palakkad */
                        div { class: "planner-mode-tabs",
                            button {
                                r#type: "button",
                                class: if arrival_channel() == ArrivalChannel::FlightKochi { "planner-mode-card is-selected" } else { "planner-mode-card" },
                                onclick: move |_| arrival_channel.set(ArrivalChannel::FlightKochi),
                                span { class: "mode-badge", "✈" }
                                div { class: "mode-card-text",
                                    span { class: "mode-card-kicker", {tr(lang, "Option 01 • Air", "ഓപ്ഷൻ 01 • വിമാനം")} }
                                    strong { {tr(lang, "Flight — Kochi (COK)", "വിമാനം — കൊച്ചി (COK)")} }
                                    span { class: "mode-card-sub", {tr(lang, "COK → Thrissur Hotels → Wedding Church", "COK → തൃശ്ശൂർ ഹോട്ടലുകൾ → വിവാഹ പള്ളി")} }
                                }
                            }
                            button {
                                r#type: "button",
                                class: if arrival_channel() == ArrivalChannel::TrainHubs { "planner-mode-card is-selected" } else { "planner-mode-card" },
                                onclick: move |_| arrival_channel.set(ArrivalChannel::TrainHubs),
                                span { class: "mode-badge", "🚆" }
                                div { class: "mode-card-text",
                                    span { class: "mode-card-kicker", {tr(lang, "Option 02 • Rail", "ഓപ്ഷൻ 02 • ട്രെയിൻ")} }
                                    strong { {tr(lang, "Train — Palakkad & Kochi", "ട്രെയിൻ — പാലക്കാട് & കൊച്ചി")} }
                                    span { class: "mode-card-sub", {tr(lang, "Reach PGT or ERS → Transfer to Church", "പാലക്കാട് / കൊച്ചി → പള്ളിയിലേക്കുള്ള വഴി")} }
                                }
                            }
                        }

                        if arrival_channel() == ArrivalChannel::FlightKochi {
                            div { class: "planner-flow-stage",
                                div { class: "planner-subfilter-row",
                                    span { class: "subfilter-label", {tr(lang, "Route Style:", "യാത്രാ രീതി:")} }
                                    div { class: "subfilter-pills",
                                        button {
                                            r#type: "button",
                                            class: if flight_path() == FlightPath::ViaThrissurStay { "subfilter-pill is-active" } else { "subfilter-pill" },
                                            onclick: move |_| flight_path.set(FlightPath::ViaThrissurStay),
                                            {tr(lang, "Via Thrissur (Most Hotels • Recommended)", "തൃശ്ശൂർ ഹോട്ടലുകൾ വഴി (ശുപാർശ ചെയ്യുന്നത്)")}
                                        }
                                        button {
                                            r#type: "button",
                                            class: if flight_path() == FlightPath::DirectToChurch { "subfilter-pill is-active" } else { "subfilter-pill" },
                                            onclick: move |_| flight_path.set(FlightPath::DirectToChurch),
                                            {tr(lang, "Direct COK → Wedding Church", "നേരിട്ട് COK → വിവാഹ പള്ളി")}
                                        }
                                    }
                                }

                                if flight_path() == FlightPath::ViaThrissurStay {
                                    /* Live Visual Chain */
                                    div { class: "planner-chain-strip",
                                        span { class: "chain-stop is-origin", {tr(lang, "✈ Kochi Airport (COK)", "✈ കൊച്ചി എയർപോർട്ട് (COK)")} }
                                        span { class: "chain-leg", {tr(lang, "52 km • 1 hr 15m →", "52 കി.മീ • 1 മണി. 15 മി. →")} }
                                        span { class: "chain-stop is-mid", {tr(lang, "⌂ Thrissur (Hotels)", "⌂ തൃശ്ശൂർ (ഹോട്ടലുകൾ)")} }
                                        span { class: "chain-leg", {tr(lang, "30 km • 35 mins →", "30 കി.മീ • 35 മി. →")} }
                                        span { class: "chain-stop is-dest", {tr(lang, "✟ Wedding Church (3:00 PM)", "✟ വിവാഹ പള്ളി (3:00 PM)")} }
                                    }

                                    div { class: "planner-legs-grid",
                                        article { class: "planner-leg-card",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge", {tr(lang, "Leg 1 • Airport to Stay Hub", "ഘട്ടം 1 • എയർപോർട്ട് → തൃശ്ശൂർ")} }
                                                span { class: "travel-distance-pill", {tr(lang, "52 km • 1 hr 15 mins", "52 കി.മീ • 1 മണിക്കൂർ 15 മിനിറ്റ്")} }
                                            }
                                            h3 { {tr(lang, "Kochi Airport (COK) → Thrissur Hotels", "കൊച്ചി എയർപോർട്ട് (COK) → തൃശ്ശൂർ ഹോട്ടലുകൾ")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "Most family hotels, boutique stays, and Airbnbs are centered in Thrissur—an easy northward ride from Cochin Airport:",
                                                    "ഭൂരിഭാഗം ഹോട്ടലുകളും എയർബിഎൻബികളും തൃശ്ശൂരിലാണ്. നെടുമ്പാശ്ശേരി എയർപോർട്ടിൽ (COK) നിന്ന് തൃശ്ശൂരിലെത്താനുള്ള വഴികൾ:",
                                                )}
                                            }
                                            ul { class: "planner-option-list",
                                                li {
                                                    strong { {tr(lang, "Uber / Pre-paid Airport Taxi / Car Rental: ", "ഊബർ / പ്രീപെയ്ഡ് എയർപോർട്ട് ടാക്സി / റെന്റൽ കാർ: ")} }
                                                    {tr(
                                                        lang,
                                                        "Available 24×7 at COK Arrivals; drops you directly at your Thrissur hotel or Airbnb in 1 hr 10–15 mins.",
                                                        "എയർപോർട്ട് അറൈവൽസിൽ 24×7 ലഭ്യമാണ്; 1 മണിക്കൂർ 15 മിനിറ്റിൽ തൃശ്ശൂരിലെ ഹോട്ടലിൽ നേരിട്ടെത്താം.",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "Aluva Feeder + Express Train: ", "ആലുവ ഫീഡർ + ട്രെയിൻ മാർഗ്ഗം: ")} }
                                                    {tr(
                                                        lang,
                                                        "Take the 20-min airport feeder bus/cab (12 km) to Aluva Railway Station (AWY), then a 45–55 min train to Thrissur (TCR).",
                                                        "20 മിനിറ്റ് ഫീഡർ ബസ്/ക്യാബ് വഴി ആലുവ റെയിൽവേ സ്റ്റേഷനിലെത്തി (AWY), അവിടെ നിന്ന് 45–55 മിനിറ്റ് ട്രെയിനിൽ തൃശ്ശൂരിലെത്താം (TCR).",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "KSRTC Low-Floor AC Bus: ", "കെ.എസ്.ആർ.ടി.സി എസി ബസ്: ")} }
                                                    {tr(
                                                        lang,
                                                        "Direct airport AC buses connect COK to Thrissur Sakthan & Municipal stands.",
                                                        "കൊച്ചി എയർപോർട്ടിൽ നിന്ന് തൃശ്ശൂരിലേക്ക് നേരിട്ടുള്ള എസി ലോ-ഫ്ലോർ ബസുകൾ ലഭ്യമാണ്.",
                                                    )}
                                                }
                                            }
                                            div { class: "leg-map-actions",
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Cochin+International+Airport,+Kochi,+Kerala&destination=Thrissur,+Kerala&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-solid",
                                                    {tr(lang, "Google Maps: COK → Thrissur (Drive) ↗", "ഗൂഗിൾ മാപ്പ്: COK → തൃശ്ശൂർ (കാർ/ക്യാബ്) ↗")}
                                                }
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Cochin+International+Airport,+Kochi,+Kerala&destination=Thrissur,+Kerala&travelmode=transit",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-outline",
                                                    {tr(lang, "Transit (Train / Bus) ↗", "ട്രെയിൻ / ബസ് മാർഗ്ഗം ↗")}
                                                }
                                            }
                                        }

                                        article { class: "planner-leg-card is-church-accent",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge is-rose", {tr(lang, "Leg 2 • Stay Hub to Wedding", "ഘട്ടം 2 • തൃശ്ശൂർ → വിവാഹ പള്ളി")} }
                                                span { class: "travel-distance-pill", {tr(lang, "30 km • 35–40 mins", "30 കി.മീ • 35–40 മിനിറ്റ്")} }
                                            }
                                            h3 { {tr(lang, "Thrissur → Lourde Matha Forane Church", "തൃശ്ശൂർ → ലൂർദ് മാതാ ഫൊറോന പള്ളി")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "Head out from your Thrissur stay by 2:00 PM for the 3:00 PM Solemn Nuptial Mass in Vadakkencherry:",
                                                    "3:00 PM-ലെ വിവാഹ കുർബാനയ്ക്കായി ഉച്ചയ്ക്ക് 2:00 PM-ഓടെ തൃശ്ശൂരിലെ ഹോട്ടലിൽ നിന്ന് തിരിക്കാം:",
                                                )}
                                            }
                                            ul { class: "planner-option-list",
                                                li {
                                                    strong { {tr(lang, "Uber / Hotel Cab / Car Rental: ", "ഊബർ / ഹോട്ടൽ ക്യാബ് / റെന്റൽ കാർ: ")} }
                                                    {tr(
                                                        lang,
                                                        "Smooth 35-minute drive right to the church; TMK Arena (7:00 PM Reception) is just 3.5 km (6 mins) further.",
                                                        "35 മിനിറ്റിൽ പള്ളിയിലെത്താം; തുടർന്ന് 7:00 PM വിരുന്ന് നടക്കുന്ന TMK അരീനയിലേക്ക് 3.5 കി.മീ (6 മിനിറ്റ്) മാത്രം.",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "Direct Fast Bus: ", "നേരിട്ടുള്ള ബസ് സൗകര്യം: ")} }
                                                    {tr(
                                                        lang,
                                                        "All Palakkad-bound KSRTC & fast buses from Thrissur stop at Vadakkencherry (5 mins from the church).",
                                                        "തൃശ്ശൂരിൽ നിന്ന് പാലക്കാട് ഭാഗത്തേക്കുള്ള എല്ലാ ബസുകളും പള്ളിക്ക് അടുത്തുള്ള വടക്കഞ്ചേരിയിൽ നിർത്തും.",
                                                    )}
                                                }
                                            }
                                            div { class: "leg-map-actions",
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Thrissur,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-solid-rose",
                                                    {tr(lang, "Google Maps: Thrissur → Church ↗", "ഗൂഗിൾ മാപ്പ്: തൃശ്ശൂർ → പള്ളി ↗")}
                                                }
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&destination=TMK+Arena,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-outline",
                                                    {tr(lang, "Church → TMK Arena (3.5 km) ↗", "പള്ളി → TMK അരീന (3.5 കി.മീ) ↗")}
                                                }
                                            }
                                        }
                                    }

                                    div { class: "planner-master-bar",
                                        span { class: "master-bar-note",
                                            {tr(
                                                lang,
                                                "Want the entire multi-stop drive open at once?",
                                                "മുഴുവൻ യാത്രയും ഒറ്റ മാപ്പിൽ കാണണോ?",
                                            )}
                                        }
                                        a {
                                            href: "https://www.google.com/maps/dir/?api=1&origin=Cochin+International+Airport,+Kochi,+Kerala&destination=TMK+Arena,+Vadakkencherry,+Kerala+678683&waypoints=Thrissur,+Kerala%7CLourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: "gmap-master-link",
                                            {tr(
                                                lang,
                                                "Open Full Multi-Stop Google Map (COK → Thrissur → Church → Reception) ↗",
                                                "പൂർണ്ണ ഗൂഗിൾ മാപ്പ് തുറക്കുക (COK → തൃശ്ശൂർ → പള്ളി → വിരുന്ന്) ↗",
                                            )}
                                        }
                                    }
                                } else {
                                    /* Direct COK -> Church */
                                    div { class: "planner-chain-strip",
                                        span { class: "chain-stop is-origin", {tr(lang, "✈ Kochi Airport (COK)", "✈ കൊച്ചി എയർപോർട്ട് (COK)")} }
                                        span { class: "chain-leg", {tr(lang, "75 km • 1 hr 25 mins →", "75 കി.മീ • 1 മണി. 25 മി. →")} }
                                        span { class: "chain-stop is-dest", {tr(lang, "✟ Wedding Church (3:00 PM)", "✟ വിവാഹ പള്ളി (3:00 PM)")} }
                                        span { class: "chain-leg", {tr(lang, "3.5 km • 6 mins →", "3.5 കി.മീ • 6 മി. →")} }
                                        span { class: "chain-stop is-mid", {tr(lang, "★ TMK Arena (7:00 PM)", "★ TMK അരീന (7:00 PM)")} }
                                    }

                                    article { class: "planner-leg-card is-single-wide is-church-accent",
                                        div { class: "leg-card-header",
                                            span { class: "leg-step-badge is-rose", {tr(lang, "Direct Airport Transfer", "നേരിട്ടുള്ള എയർപോർട്ട് യാത്ര")} }
                                            span { class: "travel-distance-pill", {tr(lang, "75 km • 1 hr 25 mins", "75 കി.മീ • 1 മണിക്കൂർ 25 മിനിറ്റ്")} }
                                        }
                                        h3 { {tr(lang, "Cochin Airport (COK) → Lourde Matha Forane Church", "കൊച്ചി എയർപോർട്ട് (COK) → ലൂർദ് മാതാ ഫൊറോന പള്ളി")} }
                                        p { class: "leg-lead",
                                            {tr(
                                                lang,
                                                "Flying in on Sunday morning (Jan 24) or staying in Vadakkencherry? Head straight from COK Airport to the church:",
                                                "ഞായറാഴ്ച രാവിലെ എത്തുന്നവർക്കും വടക്കഞ്ചേരിയിൽ താമസിക്കുന്നവർക്കും എയർപോർട്ടിൽ നിന്ന് നേരിട്ട് പള്ളിയിലേക്ക് വരാം:",
                                            )}
                                        }
                                        ul { class: "planner-option-list",
                                            li {
                                                strong { {tr(lang, "Pre-paid Airport Cab / Uber / Self-Drive Rental: ", "പ്രീപെയ്ഡ് എയർപോർട്ട് ടാക്സി / ഊബർ / റെന്റൽ കാർ: ")} }
                                                {tr(
                                                    lang,
                                                    "Direct 1 hr 25 min ride via Thrissur right into Vadakkencherry.",
                                                    "തൃശ്ശൂർ വഴി 1 മണിക്കൂർ 25 മിനിറ്റിൽ നേരിട്ട് വടക്കഞ്ചേരി പള്ളിയിലെത്താം.",
                                                )}
                                            }
                                            li {
                                                strong { {tr(lang, "Bus via Thrissur / Angamaly: ", "ബസ് മാർഗ്ഗം: ")} }
                                                {tr(
                                                    lang,
                                                    "Board a Palakkad-bound KSRTC bus to alight directly at Vadakkencherry.",
                                                    "പാലക്കാട് ഭാഗത്തേക്കുള്ള കെ.എസ്.ആർ.ടി.സി ബസിൽ വടക്കഞ്ചേരിയിൽ നേരിട്ടിറങ്ങാം.",
                                                )}
                                            }
                                        }
                                        div { class: "leg-map-actions",
                                            a {
                                                href: "https://www.google.com/maps/dir/?api=1&origin=Cochin+International+Airport,+Kochi,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                class: "gmap-btn is-solid-rose",
                                                {tr(lang, "Google Maps: COK → Church (Drive) ↗", "ഗൂഗിൾ മാപ്പ്: COK → പള്ളി (ക്യാബ്) ↗")}
                                            }
                                            a {
                                                href: "https://www.google.com/maps/dir/?api=1&origin=Cochin+International+Airport,+Kochi,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=transit",
                                                target: "_blank",
                                                rel: "noopener noreferrer",
                                                class: "gmap-btn is-outline",
                                                {tr(lang, "Google Maps: Transit ↗", "ഗൂഗിൾ മാപ്പ്: ട്രാൻസിറ്റ് ↗")}
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            /* Option 2: Train — Palakkad & Kochi */
                            div { class: "planner-flow-stage",
                                div { class: "planner-subfilter-row",
                                    span { class: "subfilter-label", {tr(lang, "Choose Rail Hub:", "റെയിൽവേ സ്റ്റേഷൻ തിരഞ്ഞെടുക്കുക:")} }
                                    div { class: "subfilter-pills",
                                        button {
                                            r#type: "button",
                                            class: if train_gateway() == TrainGateway::Palakkad { "subfilter-pill is-active" } else { "subfilter-pill" },
                                            onclick: move |_| train_gateway.set(TrainGateway::Palakkad),
                                            {tr(lang, "Palakkad Junction (PGT • 35 km)", "പാലക്കാട് ജംഗ്ഷൻ (PGT • 35 കി.മീ)")}
                                        }
                                        button {
                                            r#type: "button",
                                            class: if train_gateway() == TrainGateway::Kochi { "subfilter-pill is-active" } else { "subfilter-pill" },
                                            onclick: move |_| train_gateway.set(TrainGateway::Kochi),
                                            {tr(lang, "Kochi / Ernakulam (ERS / ERN) & Thrissur", "കൊച്ചി (ERS / ERN) & തൃശ്ശൂർ")}
                                        }
                                    }
                                }

                                if train_gateway() == TrainGateway::Palakkad {
                                    div { class: "planner-chain-strip",
                                        span { class: "chain-stop is-origin", {tr(lang, "🚆 Your Departure City", "🚆 നിങ്ങളുടെ നഗരം")} }
                                        span { class: "chain-leg", {tr(lang, "Direct Express →", "എക്സ്പ്രസ് ട്രെയിൻ →")} }
                                        span { class: "chain-stop is-mid", {tr(lang, "🚉 Palakkad Jn (PGT)", "🚉 പാലക്കാട് ജംഗ്ഷൻ (PGT)")} }
                                        span { class: "chain-leg", {tr(lang, "35 km • 40 mins →", "35 കി.മീ • 40 മി. →")} }
                                        span { class: "chain-stop is-dest", {tr(lang, "✟ Wedding Church (3:00 PM)", "✟ വിവാഹ പള്ളി (3:00 PM)")} }
                                    }

                                    div { class: "planner-legs-grid",
                                        article { class: "planner-leg-card",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge", {tr(lang, "Step 1 • How to Reach Palakkad", "ഘട്ടം 1 • പാലക്കാട് എങ്ങനെ എത്താം")} }
                                                span { class: "travel-distance-pill", {tr(lang, "Station Code: PGT", "സ്റ്റേഷൻ കോഡ്: PGT")} }
                                            }
                                            h3 { {tr(lang, "Reaching Palakkad Junction (PGT)", "പാലക്കാട് ജംഗ്ഷനിലേക്ക് (PGT) എത്താൻ")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "Palakkad Junction (PGT) is a major main-line rail hub with direct Vande Bharat, Shatabdi & overnight express trains from Bengaluru, Chennai, Coimbatore, Hyderabad, Mumbai, Delhi & Kochi.",
                                                    "ബെംഗളൂരു, ചെന്നൈ, കോയമ്പത്തൂർ, ഹൈദരാബാദ്, മുംബൈ, ഡൽഹി, കൊച്ചി എന്നിവിടങ്ങളിൽ നിന്ന് നേരിട്ട് വന്ദേ ഭാരത്, എക്സ്പ്രസ് ട്രെയിനുകൾ എത്തുന്ന പ്രധാന റെയിൽവേ ഹബ്ബാണ് പാലക്കാട് ജംഗ്ഷൻ (PGT).",
                                                )}
                                            }
                                            div { class: "city-origin-box",
                                                span { class: "city-origin-label", {tr(lang, "Check Train Routes on Google Maps to Palakkad (PGT):", "പാലക്കാട്ടേക്കുള്ള (PGT) ട്രെയിൻ സമയം ഗൂഗിൾ മാപ്പിൽ കാണാം:")} }
                                                div { class: "city-chips-row",
                                                    a {
                                                        href: "https://www.google.com/maps/dir/?api=1&destination=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&travelmode=transit",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "city-route-chip is-highlight",
                                                        {tr(lang, "📍 My Location → PGT ↗", "📍 എന്റെ ലൊക്കേഷൻ → PGT ↗")}
                                                    }
                                                    a {
                                                        href: "https://www.google.com/maps/dir/?api=1&origin=Bengaluru,+Karnataka&destination=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&travelmode=transit",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "city-route-chip",
                                                        {tr(lang, "Bengaluru → PGT ↗", "ബെംഗളൂരു → PGT ↗")}
                                                    }
                                                    a {
                                                        href: "https://www.google.com/maps/dir/?api=1&origin=Chennai+Central+Railway+Station,+Chennai&destination=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&travelmode=transit",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "city-route-chip",
                                                        {tr(lang, "Chennai → PGT ↗", "ചെന്നൈ → PGT ↗")}
                                                    }
                                                    a {
                                                        href: "https://www.google.com/maps/dir/?api=1&origin=Coimbatore+Junction,+Coimbatore&destination=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&travelmode=transit",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "city-route-chip",
                                                        {tr(lang, "Coimbatore → PGT ↗", "കോയമ്പത്തൂർ → PGT ↗")}
                                                    }
                                                    a {
                                                        href: "https://www.google.com/maps/dir/?api=1&origin=Ernakulam+Junction,+Kochi,+Kerala&destination=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&travelmode=transit",
                                                        target: "_blank",
                                                        rel: "noopener noreferrer",
                                                        class: "city-route-chip",
                                                        {tr(lang, "Kochi (ERS) → PGT ↗", "കൊച്ചി (ERS) → PGT ↗")}
                                                    }
                                                }
                                            }
                                        }

                                        article { class: "planner-leg-card is-church-accent",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge is-rose", {tr(lang, "Step 2 • Palakkad to Church", "ഘട്ടം 2 • പാലക്കാട് → വിവാഹ പള്ളി")} }
                                                span { class: "travel-distance-pill", {tr(lang, "35 km • 40–45 mins", "35 കി.മീ • 40–45 മിനിറ്റ്")} }
                                            }
                                            h3 { {tr(lang, "Palakkad (PGT) → Lourde Matha Forane Church", "പാലക്കാട് (PGT) → ലൂർദ് മാതാ ഫൊറോന പള്ളി")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "From Palakkad Junction, Vadakkencherry is an easy 40-minute westward ride:",
                                                    "പാലക്കാട് ജംഗ്ഷനിൽ നിന്ന് 40 മിനിറ്റ് യാത്ര ചെയ്താൽ വടക്കഞ്ചേരി പള്ളിയിലെത്താം:",
                                                )}
                                            }
                                            ul { class: "planner-option-list",
                                                li {
                                                    strong { {tr(lang, "Pre-paid Station Taxi / Uber / Rental: ", "പ്രീപെയ്ഡ് ടാക്സി / ഊബർ / റെന്റൽ കാർ: ")} }
                                                    {tr(
                                                        lang,
                                                        "24×7 taxi counter outside PGT Platform 1 drops you straight at Lourde Matha Forane Church in 40 mins.",
                                                        "പാലക്കാട് ജംഗ്ഷന് പുറത്തുള്ള പ്രീപെയ്ഡ് ടാക്സി / ഊബർ വഴി 40 മിനിറ്റിൽ നേരിട്ട് പള്ളിയിലെത്താം.",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "Direct KSRTC / Fast Bus: ", "നേരിട്ടുള്ള കെ.എസ്.ആർ.ടി.സി / ഫാസ്റ്റ് ബസ്: ")} }
                                                    {tr(
                                                        lang,
                                                        "Every Thrissur-bound bus from Palakkad stops at Vadakkencherry (45 mins; 5 mins from the church).",
                                                        "പാലക്കാട് നിന്ന് തൃശ്ശൂർ ഭാഗത്തേക്കുള്ള എല്ലാ ബസുകളും വടക്കഞ്ചേരിയിൽ നിർത്തും (45 മിനിറ്റ്).",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "Staying in Palakkad? ", "പാലക്കാട് താമസിക്കുന്നവർക്ക്: ")} }
                                                    {tr(
                                                        lang,
                                                        "Palakkad town hotels are 10 mins from PGT—freshen up and depart by 2:00 PM for the 3:00 PM wedding.",
                                                        "സ്റ്റേഷന് അടുത്തുള്ള ഹോട്ടലിൽ താമസിച്ച് ഉച്ചയ്ക്ക് 2:00 PM-ന് പള്ളിയിലേക്ക് പുറപ്പെടാം.",
                                                    )}
                                                }
                                            }
                                            div { class: "leg-map-actions",
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-solid-rose",
                                                    {tr(lang, "Google Maps: Palakkad → Church (Drive) ↗", "ഗൂഗിൾ മാപ്പ്: പാലക്കാട് → പള്ളി (ക്യാബ്) ↗")}
                                                }
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=transit",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-outline",
                                                    {tr(lang, "Bus / Transit ↗", "ബസ് / ട്രാൻസിറ്റ് ↗")}
                                                }
                                            }
                                        }
                                    }

                                    div { class: "planner-master-bar",
                                        span { class: "master-bar-note",
                                            {tr(
                                                lang,
                                                "Complete rail-arrival map including evening reception:",
                                                "വിരുന്ന് വേദി ഉൾപ്പെടെയുള്ള പൂർണ്ണ ഗൂഗിൾ മാപ്പ്:",
                                            )}
                                        }
                                        a {
                                            href: "https://www.google.com/maps/dir/?api=1&origin=Palakkad+Junction+Railway+Station,+Palakkad,+Kerala&destination=TMK+Arena,+Vadakkencherry,+Kerala+678683&waypoints=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                            target: "_blank",
                                            rel: "noopener noreferrer",
                                            class: "gmap-master-link",
                                            {tr(
                                                lang,
                                                "Open Full Google Map (Palakkad PGT → Church → TMK Arena) ↗",
                                                "പൂർണ്ണ ഗൂഗിൾ മാപ്പ് തുറക്കുക (പാലക്കാട് PGT → പള്ളി → TMK അരീന) ↗",
                                            )}
                                        }
                                    }
                                } else {
                                    /* Kochi / Ernakulam (ERS / ERN) & Thrissur (TCR) */
                                    div { class: "planner-chain-strip",
                                        span { class: "chain-stop is-origin", {tr(lang, "🚉 Kochi (ERS / ERN)", "🚉 കൊച്ചി (ERS / ERN)")} }
                                        span { class: "chain-leg", {tr(lang, "Train 1 hr 15m →", "ട്രെയിൻ 1 മണി. 15 മി. →")} }
                                        span { class: "chain-stop is-mid", {tr(lang, "⌂ Thrissur (TCR • Hotels)", "⌂ തൃശ്ശൂർ (TCR • ഹോട്ടലുകൾ)")} }
                                        span { class: "chain-leg", {tr(lang, "30 km • 35 mins →", "30 കി.മീ • 35 മി. →")} }
                                        span { class: "chain-stop is-dest", {tr(lang, "✟ Wedding Church (3:00 PM)", "✟ വിവാഹ പള്ളി (3:00 PM)")} }
                                    }

                                    div { class: "planner-legs-grid",
                                        article { class: "planner-leg-card",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge", {tr(lang, "Step 1 • Kochi Rail & Thrissur Link", "ഘട്ടം 1 • കൊച്ചി & തൃശ്ശൂർ ട്രെയിൻ")} }
                                                span { class: "travel-distance-pill", {tr(lang, "ERS / ERN → TCR", "ERS / ERN → TCR")} }
                                            }
                                            h3 { {tr(lang, "Arriving at Kochi (ERS / ERN) → Thrissur", "കൊച്ചിയിൽ (ERS / ERN) നിന്ന് തൃശ്ശൂരിലേക്ക്")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "Reaching Ernakulam Junction (ERS) or Town (ERN)? Connect effortlessly to Thrissur (TCR), where most guest hotels & Airbnbs are located:",
                                                    "എറണാകുളം ജംഗ്ഷനിലോ (ERS) നോർത്തിലോ (ERN) എത്തുന്നവർക്ക് ഹോട്ടലുകൾ കേന്ദ്രീകരിച്ചിരിക്കുന്ന തൃശ്ശൂരിലേക്ക് (TCR) എളുപ്പത്തിൽ എത്താം:",
                                                )}
                                            }
                                            ul { class: "planner-option-list",
                                                li {
                                                    strong { {tr(lang, "Connecting Train (ERS → TCR, 1 hr 15 mins): ", "തുടർ ട്രെയിൻ (ERS → TCR, 1 മണിക്കൂർ 15 മിനിറ്റ്): ")} }
                                                    {tr(
                                                        lang,
                                                        "Trains depart every 30–45 mins from Kochi to Thrissur. (Tip: Check if your train already stops directly at Thrissur TCR!)",
                                                        "കൊച്ചിയിൽ നിന്ന് തൃശ്ശൂരിലേക്ക് തുടർച്ചയായി ട്രെയിനുകളുണ്ട്. (സൂചന: നിങ്ങളുടെ ട്രെയിനിന് തൃശ്ശൂരിൽ നേരിട്ട് സ്റ്റോപ്പുണ്ടോ എന്നും നോക്കാം!)",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "Uber / Cab / Rental Car: ", "ഊബർ / ക്യാബ് / റെന്റൽ കാർ: ")} }
                                                    {tr(
                                                        lang,
                                                        "72 km (1 hr 35 mins) from Kochi to Thrissur hotels, or 85 km (1 hr 50 mins) straight to Vadakkencherry.",
                                                        "കൊച്ചിയിൽ നിന്ന് തൃശ്ശൂരിലേക്ക് 72 കി.മീ, അല്ലെങ്കിൽ വടക്കഞ്ചേരിയിലേക്ക് നേരിട്ട് 85 കി.മീ.",
                                                    )}
                                                }
                                            }
                                            div { class: "leg-map-actions",
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&destination=Ernakulam+Junction,+Kochi,+Kerala&travelmode=transit",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-outline",
                                                    {tr(lang, "📍 Trains to Kochi (ERS) ↗", "📍 കൊച്ചിയിലേക്കുള്ള ട്രെയിൻ ↗")}
                                                }
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Ernakulam+Junction,+Kochi,+Kerala&destination=Thrissur+Railway+Station,+Thrissur,+Kerala&travelmode=transit",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-solid",
                                                    {tr(lang, "Google Maps: Kochi → Thrissur Train ↗", "ഗൂഗിൾ മാപ്പ്: കൊച്ചി → തൃശ്ശൂർ ട്രെയിൻ ↗")}
                                                }
                                            }
                                        }

                                        article { class: "planner-leg-card is-church-accent",
                                            div { class: "leg-card-header",
                                                span { class: "leg-step-badge is-rose", {tr(lang, "Step 2 • Thrissur (TCR) to Church", "ഘട്ടം 2 • തൃശ്ശൂർ → വിവാഹ പള്ളി")} }
                                                span { class: "travel-distance-pill", {tr(lang, "30 km • 35–40 mins", "30 കി.മീ • 35–40 മിനിറ്റ്")} }
                                            }
                                            h3 { {tr(lang, "Thrissur (TCR) → Lourde Matha Forane Church", "തൃശ്ശൂർ (TCR) → ലൂർദ് മാതാ ഫൊറോന പള്ളി")} }
                                            p { class: "leg-lead",
                                                {tr(
                                                    lang,
                                                    "From Thrissur Railway Station or your Thrissur hotel, reach Vadakkencherry smoothly:",
                                                    "തൃശ്ശൂർ സ്റ്റേഷനിൽ നിന്നോ ഹോട്ടലിൽ നിന്നോ വടക്കഞ്ചേരി പള്ളിയിലേക്ക് സുഗമമായി എത്താം:",
                                                )}
                                            }
                                            ul { class: "planner-option-list",
                                                li {
                                                    strong { {tr(lang, "Uber / Station Pre-paid Cab: ", "ഊബർ / പ്രീപെയ്ഡ് ടാക്സി: ")} }
                                                    {tr(
                                                        lang,
                                                        "35-minute drive straight to Lourde Matha Forane Church (and 3.5 km onward to TMK Arena).",
                                                        "35 മിനിറ്റിൽ ലൂർദ് മാതാ പള്ളിയിലും, തുടർന്ന് 3.5 കി.മീ അകലെയുള്ള TMK അരീനയിലും എത്താം.",
                                                    )}
                                                }
                                                li {
                                                    strong { {tr(lang, "KSRTC / Fast Bus: ", "കെ.എസ്.ആർ.ടി.സി / ഫാസ്റ്റ് ബസ്: ")} }
                                                    {tr(
                                                        lang,
                                                        "Frequent buses from Thrissur Stand (adjacent to TCR station) drop at Vadakkencherry.",
                                                        "തൃശ്ശൂർ സ്റ്റേഷന് തൊട്ടടുത്തുള്ള സ്റ്റാൻഡിൽ നിന്ന് വടക്കഞ്ചേരിയിലേക്ക് എപ്പോഴും ബസുകൾ ലഭ്യമാണ്.",
                                                    )}
                                                }
                                            }
                                            div { class: "leg-map-actions",
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Thrissur+Railway+Station,+Thrissur,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-solid-rose",
                                                    {tr(lang, "Google Maps: TCR → Church (Drive) ↗", "ഗൂഗിൾ മാപ്പ്: TCR → പള്ളി (ക്യാബ്) ↗")}
                                                }
                                                a {
                                                    href: "https://www.google.com/maps/dir/?api=1&origin=Ernakulam+Junction,+Kochi,+Kerala&destination=Lourde+Matha+Forane+Church,+Vadakkencherry,+Kerala+678683&waypoints=Thrissur,+Kerala&travelmode=driving",
                                                    target: "_blank",
                                                    rel: "noopener noreferrer",
                                                    class: "gmap-btn is-outline",
                                                    {tr(lang, "Full Map: Kochi → Thrissur → Church ↗", "മാപ്പ്: കൊച്ചി → തൃശ്ശൂർ → പള്ളി ↗")}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    /* Return / Exit Journey Planner */
                    div { class: "planner-body is-exit-mode",
                        div { class: "exit-selectors-grid",
                            div { class: "exit-selector-group",
                                span { class: "subfilter-label", {tr(lang, "1. Leaving From:", "1. എവിടെ നിന്ന് തിരിക്കുന്നു:")} }
                                div { class: "subfilter-pills",
                                    button {
                                        r#type: "button",
                                        class: if exit_origin() == ExitOrigin::TmkArena { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_origin.set(ExitOrigin::TmkArena),
                                        {tr(lang, "TMK Arena (After Reception)", "TMK അരീന (വിരുന്നിന് ശേഷം)")}
                                    }
                                    button {
                                        r#type: "button",
                                        class: if exit_origin() == ExitOrigin::Church { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_origin.set(ExitOrigin::Church),
                                        {tr(lang, "Wedding Church", "വിവാഹ പള്ളി")}
                                    }
                                    button {
                                        r#type: "button",
                                        class: if exit_origin() == ExitOrigin::ThrissurStay { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_origin.set(ExitOrigin::ThrissurStay),
                                        {tr(lang, "Thrissur Hotels (Next Day)", "തൃശ്ശൂർ ഹോട്ടലുകൾ")}
                                    }
                                }
                            }

                            div { class: "exit-selector-group",
                                span { class: "subfilter-label", {tr(lang, "2. Exit Destination:", "2. മടങ്ങേണ്ട സ്ഥലം:")} }
                                div { class: "subfilter-pills",
                                    button {
                                        r#type: "button",
                                        class: if exit_target() == ExitTarget::KochiAirport { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_target.set(ExitTarget::KochiAirport),
                                        {tr(lang, "✈ Kochi Airport (COK • Nearest)", "✈ കൊച്ചി എയർപോർട്ട് (COK)")}
                                    }
                                    button {
                                        r#type: "button",
                                        class: if exit_target() == ExitTarget::PalakkadStation { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_target.set(ExitTarget::PalakkadStation),
                                        {tr(lang, "🚆 Palakkad Station (PGT)", "🚆 പാലക്കാട് സ്റ്റേഷൻ (PGT)")}
                                    }
                                    button {
                                        r#type: "button",
                                        class: if exit_target() == ExitTarget::ThrissurKochiRail { "subfilter-pill is-active" } else { "subfilter-pill" },
                                        onclick: move |_| exit_target.set(ExitTarget::ThrissurKochiRail),
                                        {tr(lang, "🚆 Thrissur (TCR) / Kochi (ERS)", "🚆 തൃശ്ശൂർ (TCR) / കൊച്ചി")}
                                    }
                                }
                            }
                        }

                        div { class: "planner-chain-strip is-exit-chain",
                            span { class: "chain-stop is-dest", "{origin_short}" }
                            span { class: "chain-leg", "{exit_distance} →" }
                            span { class: "chain-stop is-origin", "{target_short}" }
                        }

                        article { class: "planner-leg-card is-single-wide is-exit-card",
                            div { class: "leg-card-header",
                                span { class: "leg-step-badge", {tr(lang, "Return / Exit Route", "മടക്കയാത്രാ മാർഗ്ഗം")} }
                                span { class: "travel-distance-pill", "{exit_distance}" }
                            }
                            h3 { "{exit_heading}" }
                            ul { class: "planner-option-list",
                                li { "{exit_opt_primary}" }
                                li { "{exit_opt_secondary}" }
                            }
                            div { class: "exit-tip-banner",
                                "{exit_tip}"
                            }
                            div { class: "leg-map-actions",
                                a {
                                    href: "{exit_drive_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "gmap-btn is-solid",
                                    {tr(lang, "Open Return Route in Google Maps (Drive / Cab) ↗", "മടക്കയാത്ര ഗൂഗിൾ മാപ്പിൽ (ക്യാബ് / കാർ) ↗")}
                                }
                                a {
                                    href: "{exit_transit_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "gmap-btn is-outline",
                                    {tr(lang, "Open Return Transit (Train / Bus) ↗", "മടക്കയാത്ര (ട്രെയിൻ / ബസ്) ↗")}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FamilySide {
    Groom,
    Bride,
}

#[component]
fn RsvpSection() -> Element {
    let lang = use_context::<Signal<Lang>>()();
    let mut guest_name = use_signal(String::new);
    let mut family_side: Signal<Option<FamilySide>> = use_signal(|| None);
    let mut is_attending = use_signal(|| true);
    let mut guest_count = use_signal(|| 2u8);
    let mut ev_church = use_signal(|| true);
    let mut ev_reception = use_signal(|| true);
    let mut arrival_idx = use_signal(|| 2usize);
    let mut wishes = use_signal(String::new);
    let mut submitted = use_signal(|| false);
    let copied_rsvp = use_signal(|| false);
    let sheet_sync = use_signal(|| SheetSyncState::Idle);

    let trimmed_name = guest_name().trim().to_string();
    let trimmed_wishes = wishes().trim().to_string();
    let is_form_open = family_side().is_some();
    let side_label = match family_side() {
        Some(FamilySide::Bride) => tr(lang, "Sharon • Kalloor Family", "ഷാരോൺ • കല്ലൂർ കുടുംബം"),
        _ => tr(lang, "Liju • Kadankavil Family", "ലിജു • കടങ്കാവിൽ കുടുംബം"),
    };
    let side_short = match family_side() {
        Some(FamilySide::Bride) => tr(lang, "Guest of Sharon", "ഷാരോണിന്റെ അതിഥി"),
        _ => tr(lang, "Guest of Liju", "ലിജുവിന്റെ അതിഥി"),
    };
    let attendance_label = if is_attending() {
        tr(
            lang,
            "Joyfully Celebrating With You",
            "സന്തോഷപൂർവ്വം പങ്കെടുക്കുന്നു",
        )
    } else {
        tr(lang, "Sending Love & Prayers", "സ്നേഹവും പ്രാർത്ഥനകളും നേരുന്നു")
    };

    let selected_arrival = match lang {
        Lang::En => ARRIVAL_SLOTS[arrival_idx().min(ARRIVAL_SLOTS.len() - 1)],
        Lang::Ml => ARRIVAL_SLOTS_ML[arrival_idx().min(ARRIVAL_SLOTS_ML.len() - 1)],
    };

    let selected_events = {
        let mut chosen = Vec::new();
        if ev_church() {
            chosen.push(tr(
                lang,
                "Wedding at Church (3:00 PM)",
                "പള്ളിയിലെ വിവാഹം (3:00 PM)",
            ));
        }
        if ev_reception() {
            chosen.push(tr(
                lang,
                "Evening Reception (7:00 PM)",
                "സായാഹ്ന വിരുന്ന് (7:00 PM)",
            ));
        }
        if chosen.is_empty() {
            tr(lang, "None selected", "ഒന്നും തിരഞ്ഞെടുത്തിട്ടില്ല").to_string()
        } else {
            chosen.join(", ")
        }
    };

    let summary_text = if lang == Lang::Ml {
        if is_attending() {
            if trimmed_wishes.is_empty() {
                format!(
                    "ലിജു & ഷാരോൺ വിവാഹം RSVP (24 ജനുവരി 2027)\nഅതിഥി: {trimmed_name}\nക്ഷണിച്ചത്: {side_label}\nസ്ഥിതി: {attendance_label}\nഅതിഥികളുടെ എണ്ണം: {}\nചടങ്ങുകൾ: {selected_events}\nഎത്തുന്ന സമയം: {selected_arrival}",
                    guest_count()
                )
            } else {
                format!(
                    "ലിജു & ഷാരോൺ വിവാഹം RSVP (24 ജനുവരി 2027)\nഅതിഥി: {trimmed_name}\nക്ഷണിച്ചത്: {side_label}\nസ്ഥിതി: {attendance_label}\nഅതിഥികളുടെ എണ്ണം: {}\nചടങ്ങുകൾ: {selected_events}\nഎത്തുന്ന സമയം: {selected_arrival}\nകുറിപ്പ്: {trimmed_wishes}",
                    guest_count()
                )
            }
        } else if trimmed_wishes.is_empty() {
            format!(
                "ലിജു & ഷാരോൺ വിവാഹം RSVP (24 ജനുവരി 2027)\nഅതിഥി: {trimmed_name}\nക്ഷണിച്ചത്: {side_label}\nസ്ഥിതി: {attendance_label}"
            )
        } else {
            format!(
                "ലിജു & ഷാരോൺ വിവാഹം RSVP (24 ജനുവരി 2027)\nഅതിഥി: {trimmed_name}\nക്ഷണിച്ചത്: {side_label}\nസ്ഥിതി: {attendance_label}\nകുറിപ്പ്: {trimmed_wishes}"
            )
        }
    } else if is_attending() {
        if trimmed_wishes.is_empty() {
            format!(
                "RSVP for Liju & Sharon's Wedding (24 Jan 2027)\nGuest: {trimmed_name}\nGuest of: {side_label}\nStatus: {attendance_label}\nParty Size: {}\nCelebrations: {selected_events}\nWhen Reaching: {selected_arrival}",
                guest_count()
            )
        } else {
            format!(
                "RSVP for Liju & Sharon's Wedding (24 Jan 2027)\nGuest: {trimmed_name}\nGuest of: {side_label}\nStatus: {attendance_label}\nParty Size: {}\nCelebrations: {selected_events}\nWhen Reaching: {selected_arrival}\nNote: {trimmed_wishes}",
                guest_count()
            )
        }
    } else if trimmed_wishes.is_empty() {
        format!(
            "RSVP for Liju & Sharon's Wedding (24 Jan 2027)\nGuest: {trimmed_name}\nGuest of: {side_label}\nStatus: {attendance_label}"
        )
    } else {
        format!(
            "RSVP for Liju & Sharon's Wedding (24 Jan 2027)\nGuest: {trimmed_name}\nGuest of: {side_label}\nStatus: {attendance_label}\nNote: {trimmed_wishes}"
        )
    };

    let whatsapp_url = format!(
        "https://wa.me/{WHATSAPP_NUMBER}?text={}",
        percent_encode(&summary_text)
    );

    rsx! {
        section { id: "rsvp", class: "chapter-rsvp",
            GhibliBreezeLayer {}

            div { class: "section-container attire-z",
                div { class: "section-header", "data-reveal": "",
                    h2 { class: "section-heading", "{tr(lang, \"RSVP & Arrival Details\", \"RSVP & വരവ് വിവരങ്ങൾ\")}" }
                    div { class: "filigree-divider" }
                    p { class: "section-subcopy",
                        "{tr(lang, \"Let us know if you can celebrate with Liju & Sharon and when you plan to reach Vadakkencherry.\", \"ലിജുവിനോടും ഷാരോണിനോടുമൊപ്പം ആഘോഷത്തിൽ പങ്കുചേരാനാകുമോ എന്നും വടക്കഞ്ചേരിയിൽ എപ്പോഴാണ് എത്തുന്നതെന്നും ഞങ്ങളെ അറിയിക്കൂ.\")}"
                    }
                }

                if submitted() {
                    div { class: "keepsake-pass",
                        div { class: "ticket-notch notch-left", "aria-hidden": "true" }
                        div { class: "ticket-notch notch-right", "aria-hidden": "true" }

                        div { class: "ticket-header",
                            span { class: "bento-kicker kicker-rose", "{tr(lang, \"RSVP Keepsake\", \"RSVP ഓർമ്മക്കുറിപ്പ്\")}" }
                            h3 { "{tr(lang, \"Thank You,\", \"നന്ദി,\")} {trimmed_name}!" }
                            p { class: "ticket-sub",
                                "{tr(lang, \"We’re so happy to hear from you—tap below to share your note with the family.\", \"നിങ്ങളുടെ മറുപടിയിൽ ഞങ്ങൾക്കേറെ സന്തോഷം—കുടുംബവുമായി പങ്കിടാൻ താഴെ ടാപ്പ് ചെയ്യൂ.\")}"
                            }
                            match sheet_sync() {
                                SheetSyncState::Saving => rsx! {
                                    p { class: "sheet-sync-pill is-saving",
                                        "{tr(lang, \"Saving to Liju & Sharon's guest list...\", \"അതിഥി പട്ടികയിലേക്ക് സേവ് ചെയ്യുന്നു...\")}"
                                    }
                                },
                                SheetSyncState::Saved => rsx! {
                                    p { class: "sheet-sync-pill is-saved",
                                        "{tr(lang, \"Saved to guest list ✓\", \"അതിഥി പട്ടികയിൽ സേവ് ചെയ്തു ✓\")}"
                                    }
                                },
                                SheetSyncState::Failed => rsx! {
                                    p { class: "sheet-sync-pill is-failed",
                                        "{tr(lang, \"Auto-save paused — please tap Send on WhatsApp ↗\", \"ഓട്ടോ-സേവ് തടസ്സപ്പെട്ടു — വാട്സ്ആപ്പിൽ അയക്കുക ↗\")}"
                                    }
                                },
                                SheetSyncState::Idle => rsx! {},
                            }
                        }

                        div { class: "ticket-divider" }

                        div { class: "ticket-details",
                            div { class: "ticket-row",
                                span { class: "meta-label", "{tr(lang, \"Guest Of\", \"അതിഥി\")}" }
                                span { class: "meta-value", "{side_label}" }
                            }
                            div { class: "ticket-row",
                                span { class: "meta-label", "{tr(lang, \"Response\", \"മറുപടി\")}" }
                                span { class: "meta-value", "{attendance_label}" }
                            }
                            if is_attending() {
                                div { class: "ticket-row",
                                    span { class: "meta-label", "{tr(lang, \"Party Size\", \"അതിഥികൾ\")}" }
                                    span { class: "meta-value",
                                        if lang == Lang::Ml {
                                            "{guest_count()} പേർ"
                                        } else {
                                            "{guest_count()} Guest(s)"
                                        }
                                    }
                                }
                                div { class: "ticket-row",
                                    span { class: "meta-label", "{tr(lang, \"Events\", \"ചടങ്ങുകൾ\")}" }
                                    span { class: "meta-value", "{selected_events}" }
                                }
                                div { class: "ticket-row",
                                    span { class: "meta-label", "{tr(lang, \"When Reaching\", \"എത്തുന്ന സമയം\")}" }
                                    span { class: "meta-value", "{selected_arrival}" }
                                }
                            }
                            if !trimmed_wishes.is_empty() {
                                div { class: "ticket-row",
                                    span { class: "meta-label", "{tr(lang, \"Note\", \"കുറിപ്പ്\")}" }
                                    span { class: "meta-value", "“{trimmed_wishes}”" }
                                }
                            }
                        }

                        div { class: "ticket-actions",
                            a {
                                href: "{whatsapp_url}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn btn-rose",
                                "{tr(lang, \"Send on WhatsApp ↗\", \"വാട്സ്ആപ്പിൽ അയക്കുക ↗\")}"
                            }
                            button {
                                r#type: "button",
                                class: if copied_rsvp() { "btn btn-ivory-outline is-copied" } else { "btn btn-ivory-outline" },
                                onclick: {
                                    let summary_clone = summary_text.clone();
                                    move |_| copy_with_feedback(summary_clone.clone(), copied_rsvp)
                                },
                                if copied_rsvp() {
                                    "{tr(lang, \"Summary Copied ✓\", \"വിവരങ്ങൾ പകർത്തി ✓\")}"
                                } else {
                                    "{tr(lang, \"Copy RSVP Summary\", \"RSVP വിവരങ്ങൾ പകർത്തുക\")}"
                                }
                            }
                            button {
                                r#type: "button",
                                class: "btn btn-link-muted",
                                onclick: move |_| submitted.set(false),
                                "{tr(lang, \"Edit Details\", \"തിരുത്തുക\")}"
                            }
                        }
                    }
                } else {
                    form {
                        class: "rsvp-panel",
                        "data-reveal": "",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            if family_side().is_some() && !guest_name().trim().is_empty() {
                                let side_en = match family_side() {
                                    Some(FamilySide::Bride) => "Guest of Sharon (Kalloor Family)",
                                    _ => "Guest of Liju (Kadankavil Family)",
                                };
                                let attending_en = if is_attending() { "Yes" } else { "No" };
                                let count_en = if is_attending() {
                                    guest_count().to_string()
                                } else {
                                    "0".to_string()
                                };
                                let events_en = if is_attending() {
                                    let mut evs = Vec::new();
                                    if ev_church() {
                                        evs.push("Wedding at Church (3:00 PM)");
                                    }
                                    if ev_reception() {
                                        evs.push("Evening Reception (7:00 PM)");
                                    }
                                    if evs.is_empty() {
                                        "None".to_string()
                                    } else {
                                        evs.join(", ")
                                    }
                                } else {
                                    "—".to_string()
                                };
                                let arrival_en = if is_attending() {
                                    ARRIVAL_SLOTS[arrival_idx().min(ARRIVAL_SLOTS.len() - 1)].to_string()
                                } else {
                                    "—".to_string()
                                };
                                let lang_en = match lang {
                                    Lang::En => "English",
                                    Lang::Ml => "Malayalam",
                                };
                                let payload = vec![
                                    GOOGLE_SHEETS_WEBHOOK_URL.to_string(),
                                    guest_name().trim().to_string(),
                                    side_en.to_string(),
                                    attending_en.to_string(),
                                    count_en,
                                    events_en,
                                    arrival_en,
                                    wishes().trim().to_string(),
                                    lang_en.to_string(),
                                ];
                                submitted.set(true);
                                submit_rsvp_to_sheets(payload, sheet_sync);
                            }
                        },

                        div { class: "rsvp-side-picker-header",
                            span {
                                class: if is_form_open {
                                    "rsvp-step-pill is-selected-pill"
                                } else {
                                    "rsvp-step-pill"
                                },
                                if is_form_open {
                                    if lang == Lang::Ml {
                                        "✓ {side_short} • മാറ്റാൻ ടാപ്പ് ചെയ്യുക"
                                    } else {
                                        "✓ {side_short} • Tap to Switch"
                                    }
                                } else {
                                    "{tr(lang, \"Tap Below to Open RSVP\", \"RSVP തുറക്കാൻ താഴെ തിരഞ്ഞെടുക്കൂ\")}"
                                }
                            }
                            h3 { class: "rsvp-picker-title",
                                if is_form_open {
                                    "{tr(lang, \"We Can’t Wait to Celebrate With You!\", \"നിങ്ങളോടൊപ്പം ആഘോഷിക്കാൻ ഞങ്ങൾ കാത്തിരിക്കുന്നു!\")}"
                                } else {
                                    "{tr(lang, \"Joining Us to Celebrate?\", \"ഞങ്ങളോടൊപ്പം ആഘോഷിക്കാൻ എത്തുന്നുണ്ടോ?\")}"
                                }
                            }
                        }

                        div {
                            class: "side-portrait-grid",
                            role: "radiogroup",
                            aria_label: tr(lang, "Select Guest of Liju or Guest of Sharon to open RSVP", "ലിജുവിന്റെയോ ഷാരോണിന്റെയോ അതിഥിയായി തിരഞ്ഞെടുക്കുക"),

                            /* 1. Guest of Liju (Mandatory Groom First!) */
                            button {
                                r#type: "button",
                                role: "radio",
                                aria_checked: if family_side() == Some(FamilySide::Groom) { "true" } else { "false" },
                                class: match family_side() {
                                    Some(FamilySide::Groom) => "side-portrait-btn is-groom-btn is-selected",
                                    Some(FamilySide::Bride) => "side-portrait-btn is-groom-btn is-dimmed",
                                    None => "side-portrait-btn is-groom-btn",
                                },
                                onclick: move |_| {
                                    let was_closed = family_side().is_none();
                                    family_side.set(Some(FamilySide::Groom));
                                    if was_closed {
                                        let _ = document::eval(
                                            "setTimeout(() => { const el = document.getElementById('guest-name'); if (el) el.focus({ preventScroll: true }); }, 280);",
                                        );
                                    }
                                },

                                div { class: "side-portrait-stage",
                                    img {
                                        src: RSVP_LIJU_FOUNTAIN,
                                        alt: "Animated Studio Ghibli scene of Liju in sunglasses smiling by the whale fountain",
                                        class: "portrait-layer-fountain",
                                        decoding: "async",
                                    }
                                    div { class: "ghibli-shimmer-veil", "aria-hidden": "true" }
                                    span { class: "venue-firefly-mote fm-1", "aria-hidden": "true" }
                                    span { class: "venue-firefly-mote fm-3", "aria-hidden": "true" }
                                    span { class: "side-portrait-tag", "{tr(lang, \"Liju • The Groom\", \"ലിജു • വരൻ\")}" }
                                }

                                div { class: "side-portrait-copy",
                                    span { class: "side-portrait-title", "{tr(lang, \"Guest of Liju\", \"ലിജുവിന്റെ അതിഥി\")}" }
                                    span { class: "side-portrait-sub", "{tr(lang, \"Kadankavil Family\", \"കടങ്കാവിൽ കുടുംബം\")}" }
                                    span { class: "side-cta-pill",
                                        if family_side() == Some(FamilySide::Groom) {
                                            "{tr(lang, \"✓ Guest of Liju\", \"✓ ലിജുവിന്റെ അതിഥി\")}"
                                        } else if is_form_open {
                                            "{tr(lang, \"Switch to Liju\", \"ലിജുവിലേക്ക് മാറ്റുക\")}"
                                        } else {
                                            "{tr(lang, \"RSVP with Liju →\", \"ലിജുവിനൊപ്പം RSVP →\")}"
                                        }
                                    }
                                }
                            }

                            /* 2. Guest of Sharon (Solo Female Portrait & Transition!) */
                            button {
                                r#type: "button",
                                role: "radio",
                                aria_checked: if family_side() == Some(FamilySide::Bride) { "true" } else { "false" },
                                class: match family_side() {
                                    Some(FamilySide::Bride) => "side-portrait-btn is-bride-btn is-selected",
                                    Some(FamilySide::Groom) => "side-portrait-btn is-bride-btn is-dimmed",
                                    None => "side-portrait-btn is-bride-btn",
                                },
                                onclick: move |_| {
                                    let was_closed = family_side().is_none();
                                    family_side.set(Some(FamilySide::Bride));
                                    if was_closed {
                                        let _ = document::eval(
                                            "setTimeout(() => { const el = document.getElementById('guest-name'); if (el) el.focus({ preventScroll: true }); }, 280);",
                                        );
                                    }
                                },

                                div { class: "side-portrait-stage",
                                    img {
                                        src: RSVP_SHARON_FOUNTAIN,
                                        alt: "Animated Studio Ghibli scene of Sharon smiling by the whale fountain",
                                        class: "portrait-layer-fountain",
                                        decoding: "async",
                                    }
                                    div { class: "ghibli-shimmer-veil", "aria-hidden": "true" }
                                    span { class: "venue-mini-petal vp-1", "aria-hidden": "true" }
                                    span { class: "venue-mini-petal vp-2", "aria-hidden": "true" }
                                    span { class: "side-portrait-tag", "{tr(lang, \"Sharon • The Bride\", \"ഷാരോൺ • വധു\")}" }
                                }

                                div { class: "side-portrait-copy",
                                    span { class: "side-portrait-title", "{tr(lang, \"Guest of Sharon\", \"ഷാരോണിന്റെ അതിഥി\")}" }
                                    span { class: "side-portrait-sub", "{tr(lang, \"Kalloor Family\", \"കല്ലൂർ കുടുംബം\")}" }
                                    span { class: "side-cta-pill",
                                        if family_side() == Some(FamilySide::Bride) {
                                            "{tr(lang, \"✓ Guest of Sharon\", \"✓ ഷാരോണിന്റെ അതിഥി\")}"
                                        } else if is_form_open {
                                            "{tr(lang, \"Switch to Sharon\", \"ഷാരോണിലേക്ക് മാറ്റുക\")}"
                                        } else {
                                            "{tr(lang, \"RSVP with Sharon →\", \"ഷാരോണിനൊപ്പം RSVP →\")}"
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            class: if is_form_open {
                                "rsvp-expandable-drawer is-open"
                            } else {
                                "rsvp-expandable-drawer"
                            },
                            aria_hidden: if is_form_open { "false" } else { "true" },

                            div { class: "rsvp-drawer-inner",
                                div { class: "rsvp-drawer-divider" }

                                div { class: "form-block",
                                    label { class: "field-label", r#for: "guest-name", "{tr(lang, \"Your Full Name\", \"നിങ്ങളുടെ മുഴുവൻ പേര്\")}" }
                                    input {
                                        id: "guest-name",
                                        class: "warm-input",
                                        r#type: "text",
                                        required: is_form_open,
                                        disabled: !is_form_open,
                                        placeholder: tr(lang, "e.g. Thomas & Mary Joseph", "ഉദാ: തോമസ് & മേരി ജോസഫ്"),
                                        value: "{guest_name}",
                                        oninput: move |evt| guest_name.set(evt.value()),
                                    }
                                }

                        div { class: "form-block",
                            span { class: "field-label", "{tr(lang, \"Will You Be Joining Us?\", \"വിവാഹത്തിൽ പങ്കെടുക്കുന്നുണ്ടോ?\")}" }
                            div { class: "choice-grid",
                                button {
                                    r#type: "button",
                                    class: if is_attending() { "choice-card is-selected" } else { "choice-card" },
                                    aria_pressed: if is_attending() { "true" } else { "false" },
                                    onclick: move |_| is_attending.set(true),
                                    span { class: "choice-title", "{tr(lang, \"Joyfully Celebrating With You\", \"സന്തോഷപൂർവ്വം പങ്കെടുക്കുന്നു\")}" }
                                    span { class: "choice-sub", "{tr(lang, \"Can’t wait to be there!\", \"അവിടെ എത്താൻ കാത്തിരിക്കുന്നു!\")}" }
                                }
                                button {
                                    r#type: "button",
                                    class: if !is_attending() { "choice-card is-selected" } else { "choice-card" },
                                    aria_pressed: if !is_attending() { "true" } else { "false" },
                                    onclick: move |_| is_attending.set(false),
                                    span { class: "choice-title", "{tr(lang, \"Sending Love & Prayers\", \"സ്നേഹവും പ്രാർത്ഥനകളും നേരുന്നു\")}" }
                                    span { class: "choice-sub", "{tr(lang, \"Cheering for you from afar\", \"ദൂരെയിരുന്ന് ആശംസകൾ നേരുന്നു\")}" }
                                }
                            }
                        }

                        if is_attending() {
                            div { class: "attending-extras",
                                div { class: "form-block stepper-row",
                                    div {
                                        span { class: "field-label", "{tr(lang, \"Number of Guests\", \"അതിഥികളുടെ എണ്ണം\")}" }
                                        p { class: "field-hint", "{tr(lang, \"Including yourself (up to 10)\", \"നിങ്ങളടക്കം (പരമാവധി 10)\")}" }
                                    }
                                    div { class: "stepper-control",
                                        button {
                                            r#type: "button",
                                            class: "stepper-btn",
                                            aria_label: tr(lang, "Decrease guests", "അതിഥികളുടെ എണ്ണം കുറയ്ക്കുക"),
                                            onclick: move |_| {
                                                let current = guest_count();
                                                if current > 1 {
                                                    guest_count.set(current - 1);
                                                }
                                            },
                                            "−"
                                        }
                                        span { class: "stepper-value", "{guest_count()}" }
                                        button {
                                            r#type: "button",
                                            class: "stepper-btn",
                                            aria_label: tr(lang, "Increase guests", "അതിഥികളുടെ എണ്ണം കൂട്ടുക"),
                                            onclick: move |_| {
                                                let current = guest_count();
                                                if current < 10 {
                                                    guest_count.set(current + 1);
                                                }
                                            },
                                            "+"
                                        }
                                    }
                                }

                                div { class: "form-block",
                                    span { class: "field-label", "{tr(lang, \"Events You’ll Attend\", \"പങ്കെടുക്കുന്ന ചടങ്ങുകൾ\")}" }
                                    div { class: "chip-selector",
                                        button {
                                            r#type: "button",
                                            class: if ev_church() { "event-chip is-checked" } else { "event-chip" },
                                            aria_pressed: if ev_church() { "true" } else { "false" },
                                            onclick: move |_| ev_church.set(!ev_church()),
                                            span { class: "chip-check", if ev_church() { "✓" } else { "+" } }
                                            {tr(lang, "Wedding at Church (3:00 PM)", "പള്ളിയിലെ വിവാഹം (3:00 PM)")}
                                        }
                                        button {
                                            r#type: "button",
                                            class: if ev_reception() { "event-chip is-checked" } else { "event-chip" },
                                            aria_pressed: if ev_reception() { "true" } else { "false" },
                                            onclick: move |_| ev_reception.set(!ev_reception()),
                                            span { class: "chip-check", if ev_reception() { "✓" } else { "+" } }
                                            {tr(lang, "Evening Reception (7:00 PM)", "സായാഹ്ന വിരുന്ന് (7:00 PM)")}
                                        }
                                    }
                                }

                                div { class: "form-block",
                                    span { class: "field-label", "{tr(lang, \"When Will You Reach?\", \"എപ്പോഴാണ് എത്തുന്നത്?\")}" }
                                    div { class: "chip-selector",
                                        for (idx, slot) in ARRIVAL_SLOTS.iter().enumerate() {
                                            {
                                                let is_selected = arrival_idx() == idx;
                                                let label = match lang {
                                                    Lang::En => *slot,
                                                    Lang::Ml => ARRIVAL_SLOTS_ML[idx],
                                                };
                                                rsx! {
                                                    button {
                                                        r#type: "button",
                                                        key: "{slot}",
                                                        class: if is_selected { "event-chip is-checked" } else { "event-chip" },
                                                        aria_pressed: if is_selected { "true" } else { "false" },
                                                        onclick: move |_| arrival_idx.set(idx),
                                                        span { class: "chip-check", if is_selected { "✓" } else { "○" } }
                                                        "{label}"
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "form-block",
                            label { class: "field-label", r#for: "guest-wishes",
                                "{tr(lang, \"Warm Wishes, Stay Location or Commute Notes\", \"ആശംസകൾ, താമസസ്ഥലം അല്ലെങ്കിൽ യാത്രാ വിവരങ്ങൾ\")}"
                            }
                            textarea {
                                id: "guest-wishes",
                                class: "warm-input warm-textarea",
                                rows: "3",
                                placeholder: tr(lang, "Share where you’re staying (Thrissur / Palakkad / Vadakkencherry), train/flight time, or a sweet wish for Liju & Sharon...", "താമസിക്കുന്ന സ്ഥലം (തൃശ്ശൂർ / പാലക്കാട് / വടക്കഞ്ചേരി), ട്രെയിൻ/ഫ്ലൈറ്റ് സമയം, അല്ലെങ്കിൽ ലിജുവിനും ഷാരോണിനുമുള്ള ആശംസകൾ പങ്കുവെക്കൂ..."),
                                value: "{wishes}",
                                oninput: move |evt| wishes.set(evt.value()),
                            }
                        }

                                button {
                                    r#type: "submit",
                                    disabled: !is_form_open,
                                    class: "btn btn-rose submit-btn",
                                    "{tr(lang, \"Create RSVP Note →\", \"RSVP കുറിപ്പ് തയ്യാറാക്കുക →\")}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    let lang = use_context::<Signal<Lang>>()();

    rsx! {
        footer { class: "site-footer",
            div { class: "footer-inner",
                a {
                    href: "#top",
                    class: if lang == Lang::Ml { "footer-script footer-script-ml" } else { "footer-script" },
                    "{tr(lang, \"Liju & Sharon\", \"ലിജു & ഷാരോൺ\")}"
                }
                p { class: "footer-date",
                    "{tr(lang, \"Sunday, 24 January 2027 • Vadakkencherry, Kerala\", \"ഞായർ, 24 ജനുവരി 2027 • വടക്കഞ്ചേരി, കേരളം\")}"
                }
                p { class: "footer-families",
                    "{tr(lang, \"Kadankavil & Kalloor Families\", \"കടങ്കാവിൽ & കല്ലൂർ കുടുംബങ്ങൾ\")}"
                }
                a { href: "#top", class: "footer-top-link", "{tr(lang, \"Back to Top ↑\", \"മുകളിലേക്ക് ↑\")}" }
            }
        }
    }
}
