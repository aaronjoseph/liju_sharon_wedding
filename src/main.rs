use dioxus::document;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");
const HERO_GHIBLI_BG: Asset = asset!("/assets/hero_ghibli_garden.jpg");
const COUPLE_GHIBLI_CAMEO: Asset = asset!("/assets/couple_ghibli_cameo.jpg");
const MEMORY_SELFIE_GHIBLI: Asset = asset!("/assets/memory_selfie_ghibli.jpg");

const ROUTE_COK_AIRPORT: Asset = asset!("/assets/route_cok_airport.jpg");
const ROUTE_THRISSUR: Asset = asset!("/assets/route_thrissur.jpg");
const CHURCH_GHIBLI: Asset = asset!("/assets/route_church_ghibli.jpg");
const RECEPTION_GHIBLI: Asset = asset!("/assets/route_tmk_arena_ghibli.jpg");

const ATTIRE_GUESTS_GHIBLI: Asset = asset!("/assets/attire_guests_ghibli.jpg");

const RSVP_LIJU_FOUNTAIN: Asset = asset!("/assets/rsvp_liju_fountain.jpg");
const RSVP_SHARON_FOUNTAIN: Asset = asset!("/assets/rsvp_sharon_fountain.jpg");

const WEDDING_ICS: Asset = asset!("/assets/liju_sharon_wedding.ics");
const RECEPTION_ICS: Asset = asset!("/assets/liju_sharon_reception.ics");
const WEDDING_BGM: Asset = asset!("/assets/wedding_bgm.m4a");

const CHURCH_NAME: &str = "Lourde Matha Forane Church";
const CHURCH_ADDRESS: &str = "Vadakkencherry, Kerala 678683";
const CHURCH_MAP_URL: &str = "https://maps.app.goo.gl/3imLUGfDBieyiQkd6?g_st=iw";

const RECEPTION_NAME: &str = "TMK Arena";
const RECEPTION_ADDRESS: &str = "Vadakkencherry, Kerala 678683";
const RECEPTION_MAP_URL: &str = "https://maps.app.goo.gl/6Qqgu8MbnhjLqPpQ6?g_st=iw";

const LIJU_WHATSAPP_NUMBER: &str = "919037569110";
const SHARON_WHATSAPP_NUMBER: &str = "919844348041";
const GOOGLE_SHEETS_WEBHOOK_URL: &str = "https://script.google.com/macros/s/AKfycbwc5AE_05hsVLx0QKlHUwUGxtgN8wAbkuWMrQ41i7DwVnHyaZZzgzWYwPVxRtIL9KGNGg/exec";
const MEMORY_WALL_URL: &str = "https://photos.app.goo.gl/1gW4Lzezf9qKHPKJ6";

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
        { id: "memories", en: "Memory Wall", ml: "ഓർമ്മകളുടെ ചുവർ" },
        { id: "rsvp", en: "RSVP & Arrival", ml: "RSVP & വരവ്" },
        { id: "palette", en: "What to Wear", ml: "വസ്ത്രധാരണം" },
        { id: "venue", en: "Visual Route", ml: "യാത്രാ മാർഗ്ഗം" },
        { id: "stay-travel", en: "Where to Stay", ml: "താമസ സൗകര്യങ്ങൾ" },
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

    const bgmBtn = document.getElementById("bgm-toggle-btn");
    const bgmAudio = document.getElementById("wedding-bgm");
    if (bgmBtn && bgmAudio && !bgmBtn.__slBound) {
        bgmBtn.__slBound = true;
        let userStopped = false;
        bgmAudio.volume = 0.55;

        const tryStartBgm = () => {
            if (userStopped || !bgmAudio.paused) return Promise.resolve(true);
            bgmAudio.volume = 0.55;
            const p = bgmAudio.play();
            if (p && typeof p.then === "function") {
                return p.then(() => true).catch(() => false);
            }
            return Promise.resolve(!bgmAudio.paused);
        };

        const unlockOnGesture = (evt) => {
            if (userStopped) {
                cleanupUnlock();
                return;
            }
            if (evt && evt.target && bgmBtn.contains(evt.target)) {
                return;
            }
            tryStartBgm().then((ok) => {
                if (ok) cleanupUnlock();
            });
        };

        const cleanupUnlock = () => {
            window.removeEventListener("pointerdown", unlockOnGesture, true);
            window.removeEventListener("touchstart", unlockOnGesture, true);
            window.removeEventListener("touchend", unlockOnGesture, true);
            window.removeEventListener("click", unlockOnGesture, true);
            window.removeEventListener("keydown", unlockOnGesture, true);
        };

        // Attempt immediate autoplay; if browser blocks unmuted autoplay, unlock on first gesture anywhere
        tryStartBgm().then((ok) => {
            if (!ok && !userStopped) {
                window.addEventListener("pointerdown", unlockOnGesture, { capture: true, passive: true });
                window.addEventListener("touchstart", unlockOnGesture, { capture: true, passive: true });
                window.addEventListener("touchend", unlockOnGesture, { capture: true, passive: true });
                window.addEventListener("click", unlockOnGesture, { capture: true, passive: true });
                window.addEventListener("keydown", unlockOnGesture, { capture: true, passive: true });
            }
        });

        bgmBtn.addEventListener("click", () => {
            if (bgmAudio.paused) {
                userStopped = false;
                tryStartBgm().then((ok) => {
                    if (ok) cleanupUnlock();
                });
            } else {
                userStopped = true;
                cleanupUnlock();
                bgmAudio.pause();
                bgmAudio.currentTime = 0;
            }
        });
    }

    function waitForMainStylesheet() {
        return new Promise((resolve) => {
            const start = Date.now();
            function check() {
                const hero = document.querySelector(".hero-sanctuary");
                if (hero) {
                    const cs = window.getComputedStyle(hero);
                    if (cs && cs.position === "relative" && parseFloat(cs.minHeight || "0") > 100) {
                        resolve();
                        return;
                    }
                }
                if (Date.now() - start > 5000) {
                    resolve();
                    return;
                }
                window.requestAnimationFrame(check);
            }
            check();
        });
    }

    function waitForImage(img) {
        if (!img) return Promise.resolve();
        const decodeImg = () =>
            typeof img.decode === "function"
                ? img.decode().catch(() => {})
                : Promise.resolve();
        if (img.complete) {
            return img.naturalWidth > 0 ? decodeImg() : Promise.resolve();
        }
        return new Promise((resolve) => {
            const done = () => {
                img.removeEventListener("load", onLoad);
                img.removeEventListener("error", onErr);
                resolve();
            };
            const onLoad = () => decodeImg().finally(done);
            const onErr = () => done();
            img.addEventListener("load", onLoad, { once: true });
            img.addEventListener("error", onErr, { once: true });
        });
    }

    if (!root.classList.contains("app-ready") && !window.__slBooting) {
        window.__slBooting = true;
        const timeout = (ms) => new Promise((r) => setTimeout(r, ms));

        waitForMainStylesheet()
            .then(() => {
                const heroBg = document.querySelector(".hero-ghibli-img");
                const cameoImg = document.querySelector(".couple-cameo-img");
                const fontsPromise =
                    document.fonts && document.fonts.ready
                        ? document.fonts.ready.catch(() => {})
                        : Promise.resolve();
                return Promise.race([
                    Promise.all([
                        fontsPromise,
                        waitForImage(heroBg),
                        waitForImage(cameoImg),
                    ]),
                    timeout(2400),
                ]);
            })
            .finally(() => {
                tickCountdown();
                updateScrollState();
                window.requestAnimationFrame(() => {
                    window.requestAnimationFrame(() => {
                        if (window.__slBootFailSafe) {
                            clearTimeout(window.__slBootFailSafe);
                            window.__slBootFailSafe = null;
                        }
                        root.classList.add("app-ready");
                        const curtain = document.getElementById("app-boot-curtain");
                        if (curtain) {
                            setTimeout(() => {
                                if (curtain.parentNode) {
                                    curtain.parentNode.removeChild(curtain);
                                }
                            }, 700);
                        }
                    });
                });
            });
    }
})();
"##;

struct Swatch {
    name: &'static str,
    name_ml: &'static str,
    hex: &'static str,
}

struct EventInfo {
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
        badge: "Sunday • 3:00 PM",
        badge_ml: "ഞായർ • 3:00 PM",
        title: "Wedding",
        title_ml: "വിവാഹം",
        subtitle: "Solemn Nuptial Mass, sacred vows & joyful blessings with family",
        subtitle_ml: "തിരുവിവാഹ കുർബാന, വിശുദ്ധ പ്രതിജ്ഞകൾ & കുടുംബത്തോടൊപ്പമുള്ള പ്രാർത്ഥനാശീർവാദം",
        date: "Sunday, 24 January 2027",
        date_ml: "ഞായർ, 24 ജനുവരി 2027",
        time: "3:00 PM",
        time_ml: "ഉച്ചയ്ക്ക് ശേഷം 3:00 PM",
        venue_area: "Lourde Matha Forane Church, Vadakkencherry",
        venue_area_ml: "ലൂർദ് മാതാ ഫൊറോന പള്ളി, വടക്കഞ്ചേരി",
        map_url: CHURCH_MAP_URL,
        calendar_ics: WEDDING_ICS,
    },
    EventInfo {
        badge: "Sunday • 6:30 PM",
        badge_ml: "ഞായർ • 6:30 PM",
        title: "Reception",
        title_ml: "വിരുന്ന്",
        subtitle: "Celebratory dinner, music, cake cutting & happy fellowship",
        subtitle_ml: "സ്നേഹവിരുന്ന്, സംഗീതം, കേക്ക് മുറിക്കൽ & സന്തോഷസംഗമം",
        date: "Sunday, 24 January 2027",
        date_ml: "ഞായർ, 24 ജനുവരി 2027",
        time: "6:30 PM onwards",
        time_ml: "വൈകുന്നേരം 6:30 PM മുതൽ",
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
    "Jan 24 • By 6:30 PM (Reception)",
];

const ARRIVAL_SLOTS_ML: &[&str] = &[
    "ജനുവരി 23 (തലേദിവസം)",
    "ജനുവരി 24 രാവിലെ",
    "ജനുവരി 24 • 3:00 PM (വിവാഹം)",
    "ജനുവരി 24 • 6:30 PM (വിരുന്ന്)",
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
        document::Stylesheet { href: MAIN_CSS }

        Navbar {}
        BackgroundMusicFab {}
        main { class: if current_lang == Lang::Ml { "page-shell lang-ml" } else { "page-shell" },
            HeroInvitationScene {}
            EventsSection {}
            MemoryWallSection {}
            RsvpSection {}
            AttireSection {}
            VenueAndTravelSection {}
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
                    a { href: "#memories", onclick: move |_| menu_open.set(false), "{tr(current, \"Memory Wall\", \"ഓർമ്മകളുടെ ചുവർ\")}" }
                    a {
                        href: "#rsvp",
                        class: "popover-rsvp-link",
                        onclick: move |_| menu_open.set(false),
                        "{tr(current, \"RSVP & Arrival →\", \"RSVP & വരവ് →\")}"
                    }
                    a { href: "#palette", onclick: move |_| menu_open.set(false), "{tr(current, \"What to Wear\", \"വസ്ത്രധാരണം\")}" }
                    a { href: "#venue", onclick: move |_| menu_open.set(false), "{tr(current, \"Visual Route\", \"യാത്രാ മാർഗ്ഗം\")}" }
                    a { href: "#stay-travel", onclick: move |_| menu_open.set(false), "{tr(current, \"Where to Stay\", \"താമസ സൗകര്യങ്ങൾ\")}" }
                }
            }
        }
    }
}

#[component]
fn BackgroundMusicFab() -> Element {
    let lang = use_context::<Signal<Lang>>()();
    let mut is_playing = use_signal(|| false);
    let playing = is_playing();

    rsx! {
        div { class: "bgm-dock",
            audio {
                id: "wedding-bgm",
                src: WEDDING_BGM,
                autoplay: true,
                r#loop: true,
                preload: "auto",
                "playsinline": "true",
                "webkit-playsinline": "true",
                onplay: move |_| is_playing.set(true),
                onpause: move |_| is_playing.set(false),
                onended: move |_| is_playing.set(false),
            }
            button {
                id: "bgm-toggle-btn",
                r#type: "button",
                class: if playing {
                    "bgm-floating-pill is-playing"
                } else {
                    "bgm-floating-pill"
                },
                aria_pressed: if playing { "true" } else { "false" },
                aria_label: if playing {
                    tr(lang, "Mute background music", "സംഗീതം നിർത്തുക")
                } else {
                    tr(lang, "Play background music", "സംഗീതം കേൾക്കാം")
                },
                span { class: "bgm-disc", "aria-hidden": "true",
                    svg {
                        class: "bgm-icon-svg",
                        view_box: "0 0 24 24",
                        fill: "none",
                        stroke: "currentColor",
                        "stroke-width": "2",
                        "stroke-linecap": "round",
                        "stroke-linejoin": "round",
                        path { d: "M9 18V5l12-2v13" }
                        circle { cx: "6", cy: "18", r: "3" }
                        circle { cx: "18", cy: "16", r: "3" }
                        if !playing {
                            line { class: "bgm-slash", x1: "3", y1: "3", x2: "21", y2: "21" }
                        }
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
                    loading: "eager",
                    decoding: "async",
                    "fetchpriority": "high",
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
                    loading: "eager",
                    decoding: "async",
                    "fetchpriority": "high",
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
                            {tr(lang, "SUNDAY • 24 JANUARY 2027 • 3:00 PM WEDDING & 6:30 PM RECEPTION", "ഞായർ • 24 ജനുവരി 2027 • 3:00 PM വിവാഹം & 6:30 PM വിരുന്ന്")}
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
                        "{tr(lang, \"Join Liju & Sharon on Sunday, 24 January 2027 in Vadakkencherry for the Holy Matrimony followed by the Reception.\", \"2027 ജനുവരി 24 ഞായറാഴ്ച വടക്കഞ്ചേരിയിൽ നടക്കുന്ന തിരുവിവാഹത്തിലും തുടർന്നുള്ള വിരുന്നിലും ലിജുവിനോടും ഷാരോണിനോടുമൊപ്പം പങ്കുചേരൂ.\")}"
                    }
                }

                div {
                    class: "arch-events-grid is-double",
                    "data-reveal": "",
                    for event in EVENTS {
                        article { class: "arch-event-card", key: "{event.title}",
                            div { class: "card-top-row",
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
                                span { class: "lookbook-pill is-rose", "{tr(lang, \"Sarees\", \"സാരികൾ\")}" }
                                span { class: "lookbook-pill is-rose", "{tr(lang, \"Lehenga\", \"ലെഹംഗ\")}" }
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
                        text { x: "0", y: "3", "text-anchor": "middle", fill: "#8f4252", "font-size": "10.5", "font-weight": "700", "font-family": "Nunito, sans-serif", {tr(lang, "3:00 PM → 6:30 PM (3.5 km)", "3:00 PM → 6:30 PM (3.5 കി.മീ)")} }
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
                        p { class: "node-meta", "{tr(lang, \"~80 km to TMK Arena • ~1 hr 30 mins\", \"TMK അരീനയിലേക്ക് ~80 കി.മീ • ~1.5 മണിക്കൂർ\")}" }
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
                            span { class: "venue-time-ribbon ribbon-champagne", "{tr(lang, \"Stay Hub\", \"താമസ കേന്ദ്രം\")}" }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-stay", "⌂" }
                            span { class: "node-leg-pill", "{tr(lang, \"Step 02 • Stay Hub\", \"ഘട്ടം 02 • താമസ കേന്ദ്രം\")}" }
                        }
                        span { class: "node-kicker", "{tr(lang, \"Stay & Rail Hub\", \"താമസം & റെയിൽ ഹബ്\")}" }
                        h4 { "{tr(lang, \"Thrissur\", \"തൃശ്ശൂർ\")}" }
                        p { class: "node-meta", "{tr(lang, \"~32 km to TMK Arena • ~35 mins\", \"TMK അരീനയിലേക്ക് ~32 കി.മീ • ~35 മിനിറ്റ്\")}" }
                        p { class: "node-desc",
                            "{tr(lang, \"Ideal base for Airbnbs & family hotels, ~35 minutes from TMK Arena.\", \"TMK അരീനയിൽ നിന്ന് ~35 മിനിറ്റ് അകലെ എയർബിഎൻബികൾക്കും ഹോട്ടലുകൾക്കും ഏറ്റവും അനുയോജ്യമായ സ്ഥലം.\")}"
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
                        p { class: "node-meta", "{tr(lang, \"Vadakkencherry • 3.5 km to TMK Arena\", \"വടക്കഞ്ചേരി • TMK അരീനയിലേക്ക് 3.5 കി.മീ\")}" }
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

                    /* 4. SECOND EVENT (6:30 PM): TMK Arena — Animated Ghibli Twilight Scene */
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
                            span { class: "venue-time-ribbon ribbon-champagne", {tr(lang, "6:30 PM Reception", "6:30 PM വിരുന്ന്")} }
                        }
                        div { class: "node-pin-row",
                            span { class: "node-pin pin-reception", "★" }
                            span { class: "node-leg-pill pill-warm", "{tr(lang, \"3.5 km (6 mins) from Church\", \"പള്ളിയിൽ നിന്ന് 3.5 കി.മീ (6 മിനിറ്റ്)\")}" }
                        }
                        h4 { "{tr(lang, RECEPTION_NAME, \"ടി.എം.കെ അരീന (TMK Arena)\")}" }
                        p { class: "node-meta", "{tr(lang, \"Vadakkencherry\", \"വടക്കഞ്ചേരി\")}" }
                        p { class: "node-desc",
                            {tr(lang, "Gather from 6:30 PM onwards for dinner & evening celebration.", "6:30 PM മുതൽ സായാഹ്ന വിരുന്നിനും ആഘോഷങ്ങൾക്കുമായി ഒത്തുചേരാം.")}
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
                span { class: "callout-pill", {tr(lang, "3:00 PM Church → 6:30 PM Reception", "3:00 PM വിവാഹം → 6:30 PM വിരുന്ന്")} }
                if lang == Lang::Ml {
                    p {
                        "രണ്ട് വേദികളും തമ്മിൽ "
                        strong { "3.5 കി.മീ (5–8 മിനിറ്റ്) ദൂരം മാത്രം" }
                        "—3:00 PM-ന് ലൂർദ് മാതാ ഫൊറോന പള്ളിയിൽ വിവാഹച്ചടങ്ങും, തുടർന്ന് 6:30 PM മുതൽ TMK അരീനയിൽ സായാഹ്ന വിരുന്നും."
                    }
                } else {
                    p {
                        "Both venues are just "
                        strong { "3.5 km (5–8 minutes) apart" }
                        "—celebrate the wedding vows at Lourde Matha Forane Church at 3:00 PM, followed by the evening reception at TMK Arena (6:30 PM onwards)."
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
                        {tr(lang, "Follow the journey from Kochi and Thrissur to the 3:00 PM Church Ceremony and 6:30 PM Reception.", "കൊച്ചിയിൽ നിന്നും തൃശ്ശൂരിൽ നിന്നും 3:00 PM പള്ളിയിലെ വിവാഹത്തിലേക്കും 6:30 PM വിരുന്നിലേക്കുമുള്ള യാത്രാ മാർഗ്ഗം.")}
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
                                        "Smoothly travel from your Airbnb/hotel to Lourde Matha Forane Church and onward to TMK Arena."
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
            chosen.push(tr(lang, "Wedding (3:00 PM)", "വിവാഹം (3:00 PM)"));
        }
        if ev_reception() {
            chosen.push(tr(lang, "Reception (6:30 PM)", "വിരുന്ന് (6:30 PM)"));
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

    let whatsapp_number = match family_side() {
        Some(FamilySide::Bride) => SHARON_WHATSAPP_NUMBER,
        _ => LIJU_WHATSAPP_NUMBER,
    };
    let whatsapp_url = format!(
        "https://wa.me/{whatsapp_number}?text={}",
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
                                        evs.push("Wedding (3:00 PM)");
                                    }
                                    if ev_reception() {
                                        evs.push("Reception (6:30 PM)");
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
                                            {tr(lang, "Wedding (3:00 PM)", "വിവാഹം (3:00 PM)")}
                                        }
                                        button {
                                            r#type: "button",
                                            class: if ev_reception() { "event-chip is-checked" } else { "event-chip" },
                                            aria_pressed: if ev_reception() { "true" } else { "false" },
                                            onclick: move |_| ev_reception.set(!ev_reception()),
                                            span { class: "chip-check", if ev_reception() { "✓" } else { "+" } }
                                            {tr(lang, "Reception (6:30 PM)", "വിരുന്ന് (6:30 PM)")}
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
fn MemoryWallSection() -> Element {
    let lang = use_context::<Signal<Lang>>()();
    let link_copied = use_signal(|| false);

    let whatsapp_share_msg = match lang {
        Lang::En => format!(
            "Liju & Sharon — Wedding Memory Wall 📸✨\nDrop & view wedding photos here: {MEMORY_WALL_URL}"
        ),
        Lang::Ml => format!(
            "ലിജു & ഷാരോൺ — വെഡ്ഡിംഗ് മെമ്മറി വാൾ 📸✨\nവിവാഹ ചിത്രങ്ങൾ ഇവിടെ ചേർക്കാനും കാണാനും: {MEMORY_WALL_URL}"
        ),
    };
    let whatsapp_share_url = format!(
        "https://wa.me/?text={}",
        percent_encode(&whatsapp_share_msg)
    );

    rsx! {
        section { id: "memories", class: "chapter-memories",
            div { class: "section-container",
                div { class: "section-header header-compact", "data-reveal": "",
                    span { class: "section-eyebrow",
                        "{tr(lang, \"Shared Google Photos Album\", \"പങ്കുവെച്ച ഗൂഗിൾ ഫോട്ടോസ് ആൽബം\")}"
                    }
                    h2 { class: "section-heading",
                        "{tr(lang, \"The Memory Wall\", \"ഓർമ്മകളുടെ ചുവർ\")}"
                    }
                    div { class: "filigree-divider" }
                }

                div { class: "memory-wall-board", "data-reveal": "",
                    /* Left: Single Clickable Couple Selfie Polaroid */
                    div { class: "memory-polaroid-grid is-single",
                        a {
                            href: "{MEMORY_WALL_URL}",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            class: "memory-polaroid polaroid-tilt-left",
                            span { class: "polaroid-pin pin-rose", "aria-hidden": "true" }
                            div { class: "polaroid-photo-frame",
                                img {
                                    src: MEMORY_SELFIE_GHIBLI,
                                    alt: "Liju & Sharon selfie — open Google Photos Memory Wall",
                                    loading: "lazy",
                                    decoding: "async",
                                }
                            }
                            span { class: "polaroid-caption",
                                "{tr(lang, \"Liju & Sharon\", \"ലിജു & ഷാരോൺ\")}"
                            }
                        }
                    }

                    /* Right: Compact Shared Album Drop & View Hub */
                    div { class: "memory-drop-hub",
                        h3 { class: "memory-hub-title",
                            "{tr(lang, \"Add Your Favourite Photos & Videos\", \"നിങ്ങളുടെ പ്രിയപ്പെട്ട ചിത്രങ്ങളും വീഡിയോകളും ചേർക്കൂ\")}"
                        }
                        p { class: "memory-hub-copy",
                            "{tr(lang, \"Add any pictures you like, favourite throwback moments, candid clicks, or short clips straight into our shared Google Photos album.\", \"നിങ്ങൾക്കിഷ്ടപ്പെട്ട ചിത്രങ്ങളും, പ്രിയപ്പെട്ട ഓർമ്മകളും, ചെറു വീഡിയോകളും നമ്മുടെ പങ്കുവെച്ച ഗൂഗിൾ ഫോട്ടോസ് ആൽബത്തിലേക്ക് നേരിട്ട് ചേർക്കാം.\")}"
                        }

                        div { class: "memory-cta-stack",
                            a {
                                href: "{MEMORY_WALL_URL}",
                                target: "_blank",
                                rel: "noopener noreferrer",
                                class: "btn btn-rose memory-primary-cta",
                                "{tr(lang, \"Open Google Photos Memory Wall ↗\", \"ഗൂഗിൾ ഫോട്ടോസ് മെമ്മറി വാൾ തുറക്കുക ↗\")}"
                            }

                            div { class: "memory-secondary-actions",
                                button {
                                    r#type: "button",
                                    class: if link_copied() {
                                        "memory-util-btn is-copied"
                                    } else {
                                        "memory-util-btn"
                                    },
                                    onclick: move |_| {
                                        copy_with_feedback(MEMORY_WALL_URL.to_string(), link_copied);
                                    },
                                    if link_copied() {
                                        "{tr(lang, \"✓ Album Link Copied!\", \"✓ ആൽബം ലിങ്ക് പകർത്തി!\")}"
                                    } else {
                                        "{tr(lang, \"Copy Album Link\", \"ആൽബം ലിങ്ക് പകർത്തുക\")}"
                                    }
                                }

                                a {
                                    href: "{whatsapp_share_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "memory-util-btn",
                                    "{tr(lang, \"Share Link on WhatsApp ↗\", \"വാട്സ്ആപ്പിൽ പങ്കുവെക്കുക ↗\")}"
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
                    "{tr(lang, \"With love, Liya, Aaron & Hrudya\", \"സ്നേഹത്തോടെ, ലിയ, ആരൺ & ഹൃദ്യ\")}"
                }
                a { href: "#top", class: "footer-top-link", "{tr(lang, \"Back to Top ↑\", \"മുകളിലേക്ക് ↑\")}" }
            }
        }
    }
}
