use std::str::FromStr;

use scraper::{Html, Selector};
use serde::Deserialize;

use crate::{
    calendar::{Calendar, Release},
    error::{Error, Result},
};

use super::client::Client;

#[derive(Deserialize)]
pub struct MetallumReleases {
    #[serde(rename = "iTotalRecords")]
    pub total_records: i32,
    #[serde(rename = "iTotalDisplayRecords")]
    pub total_display_records: i32,
    #[serde(rename = "aaData")]
    pub data: Vec<Vec<String>>,
}

struct MetallumReleaseParts {
    artist: String,
    artist_link: String,
    album: String,
    album_link: String,
    release_type: String,
    genre: String,
    release_date: time::Date,
}

impl MetallumReleaseParts {
    fn from_release(release: Vec<String>) -> Result<Self> {
        let selector = Selector::parse("a").map_err(|_| Error::ScraperFail)?;

        let artists = Html::parse_fragment(release.get(0).ok_or(Error::NoItem)?)
            .select(&selector)
            .map(|el| {
                let artist = el.text().collect::<Vec<_>>().join("");
                let artist_link = el.value().attr("href").unwrap_or("").to_string();
                (artist, artist_link)
            })
            .collect::<Vec<_>>();
        let artist = artists
            .clone()
            .into_iter()
            .map(|(name, _)| name)
            .collect::<Vec<_>>()
            .join(" / ");
        let artist_link = artists.first().cloned().unwrap().1;

        let (album, album_link) = Html::parse_fragment(release.get(1).ok_or(Error::NoItem)?)
            .select(&selector)
            .map(|el| {
                let album = el.text().collect::<Vec<_>>().join("");
                let album_link = el.value().attr("href").unwrap_or("").to_string();
                (album, album_link)
            })
            .collect::<Vec<_>>()
            .first()
            .cloned()
            .unwrap();

        let release_date = release
            .get(4)
            .ok_or(Error::NoItem)?
            .to_string()
            .replace("nd", "")
            .replace("st", "")
            .replace("rd", "")
            .replace("th", "")
            .replace(",", "");
        let mut release_date = release_date.split_whitespace();
        let month = release_date.next().ok_or(Error::ParseFail)?;
        let month = time::Month::from_str(&month).map_err(|_| Error::ParseFail)?;
        let day = release_date
            .next()
            .ok_or(Error::ParseFail)?
            .parse()
            .map_err(|_| Error::ParseFail)?;
        let year = release_date
            .next()
            .ok_or(Error::ParseFail)?
            .parse()
            .map_err(|_| Error::ParseFail)?;
        let release_date =
            time::Date::from_calendar_date(year, month, day).map_err(|_| Error::ParseFail)?;

        Ok(Self {
            artist,
            artist_link,
            album,
            album_link,
            release_type: release.get(2).unwrap_or(&String::new()).to_string(),
            genre: release.get(3).unwrap_or(&String::new()).to_string(),
            release_date,
        })
    }
}

pub fn scrape(client: &impl Client, year: i32) -> Result<Calendar> {
    let mut calendar = Calendar::new(year);
    let mut page = 0;

    loop {
        match client.fetch_metallum(page) {
            Some(releases) => {
                for release in releases.data {
                    let parts = MetallumReleaseParts::from_release(release)?;
                    calendar.add_release(
                        parts.release_date.month(),
                        parts.release_date.day(),
                        Release::new(parts.artist, parts.album),
                    );
                }
            }
            None => break,
        }

        page += 1;
    }

    Ok(calendar)
}

#[cfg(test)]
mod tests {
    use time::Month;

    use crate::{
        calendar::{CalendarData, Releases},
        scraper::{client::tests::MockClient, test_utils::compare_calendars},
    };

    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_2024_calendar_ok() -> Result<()> {
        let client = MockClient::new();

        let got = scrape(&client, 2024)?;

        let want = Calendar {
            year: 2024,
            data: CalendarData::from([
                (Month::October, Releases::from([
					(9, vec![
						Release::new("Threshold", "Concert in London | London Astoria 2 | 1999"),
						Release::new("Fyrgast", "Frozen in time"),
						Release::new("Templar", "I Natt"),
						Release::new("Morbid Invocation", "Opus I"),
						Release::new("Phyllomedusa", "Hope Floats"),
						Release::new("Hazzerd", "Deathbringer"),
						Release::new("Död Sol", "På drift i v​ä​st"),
					]),
                    (10, vec![
						Release::new("Rise of Kronos", "Imperium"),
						Release::new("Aydra", "Leave to Nowhere"),
						Release::new("Pandemmy", "Faithless"),
						Release::new("Pyracanda", "Losing Faith"),
						Release::new("Demon Sacrifice", "Under the Blacklight of Divine Live"),
						Release::new("Patxa", "Just Heavy Metal"),
						Release::new("Regicide", "Eternal Siege"),
						Release::new("Dream Ocean", "A Chilling Show"),
						Release::new("Griefsoul", "Extreme Northern Griefmetal"),
						Release::new("Epäkristus", "Satan, Sex and War - The Demo Compilation"),
						Release::new("Dustborn", "Unconcealed Atrocities"),
					]),
					(11, vec![
						Release::new("Knightsune", "Fearless"),
						Release::new("Andy Gillion", "Exilium"),
						Release::new("Vomit Forth", "Terrified of God"),
						Release::new("Kozoria", "The Source"),
						Release::new("Malasorte", "Apex Sovereignty"),
						Release::new("Rifftera", "Coda"),
						Release::new("Hell Is Other People", "Moirae"),
						Release::new("Ad Infinitum", "Abyss"),
						Release::new("Dragony", "Hic Svnt Dracones"),
						Release::new("Oranssi Pazuzu", "Muuntautuja"),
						Release::new("Nightmare", "Waiting for the Power - The Early Years"),
						Release::new("English Dogs", "Mad Punx & English Dogs 1983-1985"),
						Release::new("Decayed Existence", "The Beginning of Sorrows"),
						Release::new("Ana Argan List", "Irrbloss"),
						Release::new("The Mist from the Mountains", "Portal - The Gathering of Storms"),
						Release::new("Barathrum", "Überkill"),
						Release::new("Amputate", "Abysmal Ascent"),
						Release::new("The Crown", "Crown of Thorns"),
						Release::new("Speedrush", "Division Mortality"),
						Release::new("Master Boot Record", "Hardwarez"),
						Release::new("A Constant Knowledge of Death", "Appendix I: Revisions & Annotations"),
						Release::new("My Dearest Wound", "The Weight of Life Was Greater"),
						Release::new("Maitreya", "Auxesis"),
						Release::new("Fupa Goddess", "Fuckyourface"),
						Release::new("Krvl", "Donkere paden"),
						Release::new("Thanateros", "Tranceforming"),
						Release::new("The Sword", "Live at LEVITATION"),
						Release::new("Alarum", "Imperative"),
						Release::new("Alias Noone", "Weight of the World"),
						Release::new("Psychiatric Regurgitation", "Death Scriptures"),
						Release::new("Bazooka Troopaz", "The Booze Hounds of Hades"),
						Release::new("Motörhead", "The Bomber Demos"),
						Release::new("Membrance", "Undead Remains"),
						Release::new("Star Rider", "Outta Time"),
						Release::new("Ornamentos del Miedo", "Sueños"),
						Release::new("Wrathprayer", "Enkoimeterion"),
						Release::new("Roots of Disease", "Saligia Speculum"),
						Release::new("Arcania", "Lost Generation"),
						Release::new("Scumripper", "For a Few Fixes More"),
						Release::new("Cytotoxin", "Hope Terminator"),
						Release::new("Oda", "Bloodstained"),
						Release::new("Konatus", "Psikoz"),
						Release::new("Epiklesis", "La Santa Iglesia Cat​ó​lica"),
						Release::new("Klynt", "Thunderous"),
						Release::new("Druid Stone", "\"Missing Girl\" b/w \"Satellite\""),
						Release::new("Timo Tolkki", "Stratovarius: 4th Dimension Demos"),
					]),
                    (12, vec![
						Release::new("Rüsty Diamönds", "Stormbringer"),
						Release::new("For the Storms", "Losing What's Left of Us"),
						Release::new("Godsin", "Blind Faith"),
						Release::new("Delta", "Gemini"),
						Release::new("Genital Grinder", "Anthology: Tricennium Reckoning"),
						Release::new("Vitriolic", "Black Steel Vengeance"),
						Release::new("Skull Servant", "Sepulcher of Barbarians"),
					]),
					(13, vec![
						Release::new("Scars of Solitude", "Under Disheartening Skies"),
						Release::new("Bál", "Nagyobb n\u{200b}á\u{200b}lam"),
						Release::new("Morbus Kinski", "Blunt Force Boogey"),
					]),
					(15, vec![
						Release::new("Blood Red Fog / Verge", "Prism of Darkness / Second Mortification"),
						Release::new("War Dogs", "Only the Stars Are Left"),
						Release::new("Bloodrust / Regicide", "Through Death We Reign"),
						Release::new("Mortem", "Ilusión de sangre Pre-demo 1988"),
					]),
					(17, vec![
						Release::new("Korkvak", "The Hermetic Ritual"),
						Release::new("Ghostheart Nebula", "Blackshift"),
						Release::new("Winter Lantern", "Hymne to a Dismal Starre"),
						Release::new("Harvestman", "Triptych: Part Three"),
						Release::new("Dianne", "Flameborn"),
						Release::new("Oannes", "Spiders Crawl in the Abode of Enki (An Key to Absu; The Threshold of Mystery)"),
					]),
					(18, vec![
						Release::new("Obnoxious Youth", "Burning Savage"),
						Release::new("Ensiferum", "Winter Storm"),
						Release::new("The Resilient Dream Project", "Te recordaré"),
						Release::new("Funeral", "Gospel of Bones"),
						Release::new("Capilla Ardiente", "Where Gods Live and Men Die"),
						Release::new("Cortez", "Thieves and Charlatans"),
						Release::new("Jewel Throne", "Blood Vultures"),
						Release::new("Grand Magus", "Sunraven"),
						Release::new("Jerry Cantrell", "I Want Blood"),
						Release::new("Camos", "Hide from the Light"),
						Release::new("Astral Doors", "The End of It All"),
						Release::new("Feral", "To Usurp the Thrones"),
						Release::new("Crest of Darkness", "My Ghost"),
						Release::new("Chrysalïd", "Breaking the Chains"),
						Release::new("Veonity", "The Final Element"),
						Release::new("Fate", "Reconnect 'n Ignite"),
						Release::new("Frozen Crown", "War Hearts"),
						Release::new("Nolove", "Corpse Bride"),
						Release::new("Swallow the Sun", "Shining"),
						Release::new("Bonjour Tristesse", "The World Without Us"),
						Release::new("Carnosus", "Wormtales"),
						Release::new("Immortal Bird", "Sin Querencia"),
						Release::new("Mother of Graves", "The Periapt of Absence"),
						Release::new("Ashen Tomb", "Ecstatic Death Reign"),
						Release::new("Harsh Realm", "Death Carries On"),
						Release::new("Five the Hierophant", "Apeiron"),
						Release::new("The Hypothesis", "Evolve"),
						Release::new("Wreck-Defy", "Dissecting the Leech"),
						Release::new("Oryx", "Primordial Sky"),
						Release::new("Vanik", "IV"),
						Release::new("Gorebringer", "Condemned to Suffer"),
						Release::new("DGM", "Endless"),
						Release::new("Destruktor", "Indomitable"),
						Release::new("Clot", "Dehiscence"),
						Release::new("Persecutory", "The Glorious Persecution"),
						Release::new("Deathrite", "Flames Licking Fever"),
						Release::new("Contrition", "Pariahs"),
						Release::new("Seid", "Hymns to the Norse"),
						Release::new("Deserts of Mars", "Dead Planet Exodus"),
						Release::new("Fórn", "Repercussions of the Self"),
						Release::new("Aethyrick / Marras", "A Union of Spectres"),
						Release::new("Porenut", "Zaklęcie"),
						Release::new("Weeping", "Spiritual Barbarism"),
						Release::new("Dawnwalker", "The Unknowing"),
						Release::new("Onslaught Kommand", "Malignancy"),
						Release::new("Maatkare", "Rise to Power"),
						Release::new("Synthwailer", "Cry Waterfalls"),
						Release::new("Torn from the Womb", "Final Improvement Operation Symposium: Terminal Epicrise, vol. I - IV"),
						Release::new("Disentomb", "Nothing Above"),
						Release::new("Kreyl", "Obscure Rise of Ancient Eulogy"),
						Release::new("Damnations Domain", "The God of Blood"),
						Release::new("Admire the Grim", "Crescent Moon"),
						Release::new("Royal Glam", "Shields Ain't Gunna Save Ya"),
						Release::new("Deivos", "Apophenia"),
						Release::new("Ixion", "Regeneration"),
						Release::new("Wormrot", "Left to Rot"),
						Release::new("Valac", "Under the Ophidians Curse"),
						Release::new("Cursed Cemetery", "Magma Transmigration"),
						Release::new("Djevel", "Under nattens fane i Fandens prakt"),
						Release::new("Outworld", "Way of the Samurai"),
						Release::new("Distrüster", "Obscurum Per Obscurius"),
					]),
					(19, vec![
						Release::new("Inche Kai Che", "Transmutar"),
					]),
					(20, vec![
						Release::new("Silhouette", "Les dires de l'âme"),
						Release::new("Infernal Cult", "Necessity of Unreal"),
						Release::new("Bewitcher", "The Warrior Trail"),
						Release::new("Silent Requiem", "Alice: Un Cuento de Luz y Sombras"),
						Release::new("Kiko Loureiro", "Theory of Mind"),
					]),
					(21, vec![
						Release::new("Reckless Manslaughter", "Sinking into Filth"),
						Release::new("Midnightmares", "Shadow People"),
						Release::new("The Fall of Creation", "Enlightenment"),
						Release::new("Sissourlet", "Rituals in the Catacombs"),
					]),
					(22, vec![
						Release::new("Anialator", "Death Is Calling"),
						Release::new("Tarfania", "Where No Wolf Howls..."),
						Release::new("Soupir", "Ecoute s'il pleut"),
					]),
					(23, vec![
						Release::new("Clandestined", "Dead.....Forever"),
						Release::new("Bellfast", "The Warrior Celt"),
					]),
					(24, vec![
						Release::new("Lankester Merrin", "Dark Mother's Child"),
						Release::new("Bind Torture Kill", "Sauvagerie"),
					]),
					(25, vec![
						Release::new("Harpyie", "Voodoo"),
						Release::new("Burial Remains", "Adversarial"),
						Release::new("The Spirit", "Songs Against Humanity"),
						Release::new("Entheos", "An End to Everything"),
						Release::new("Triumpher", "Spirit Invictus"),
						Release::new("Loudblast", "Altering Fates and Destinies"),
						Release::new("Gaerea", "Coma"),
						Release::new("Elephant Tree / Lowrider", "The Long Forever"),
						Release::new("Haliphron", "Anatomy of Darkness"),
						Release::new("Blackevil", "Praise the Communion Fire for the Unhallowed Sacrament"),
						Release::new("Challenger", "Force of Nature"),
						Release::new("Emasculator", "The Disfigured and the Divine"),
						Release::new("Ghosts of Glaciers", "Eternal"),
						Release::new("Nuclear", "Violent DNA"),
						Release::new("Paysage d'Hiver", "Die Berge"),
						Release::new("Leviticus", "MMXXIV"),
						Release::new("Schammasch", "The Maldoror Chants: Old Ocean"),
						Release::new("Autumn's Grief", "Dead Among the Living"),
						Release::new("Ancient Curse", "Dimension 5"),
						Release::new("Mindless Sinner", "Metal Merchants"),
						Release::new("Disarray", "Religious Disease"),
						Release::new("Psychonaut 4", "...of Mourning"),
						Release::new("Gigan", "Anomalous Abstractigate Infinitessimus"),
						Release::new("Sentient Horror", "In Service of the Dead"),
						Release::new("Hatchet", "Leave No Soul"),
						Release::new("Athena XIX", "Everflow Part 1: Frames of Humanity"),
						Release::new("Iotunn", "Kinship"),
						Release::new("Gigan", "The Gigan Cassette Box Set"),
						Release::new("Devin Townsend", "PowerNerd"),
						Release::new("Turkey Vulture", "On The List"),
						Release::new("Smoke / Doomsday Profit", "Smoke // Doomsday Profit"),
						Release::new("Taking the Head of Goliath", "Futility of the Flesh"),
						Release::new("Antipope", "Doors of the Dead"),
						Release::new("Alex Nunziati", "Impending Catastrophe"),
						Release::new("Vokonis", "Transitions"),
						Release::new("Mercyless", "Those Who Reign Below"),
						Release::new("Sedimentum", "Derri​è​re les portes d'une arcane transcendante"),
						Release::new("Adamantra", "Act III: Pareidolia of Depravity"),
						Release::new("Stilverlight", "Dead Souls"),
						Release::new("Perfidious", "Savouring His Flesh"),
						Release::new("Bloodletter / Grozov / Acid Mass / Ninth Realm", "Faster than the Devil III"),
						Release::new("Ataraxie", "Le déclin"),
						Release::new("Thaw", "Fading Backwards"),
						Release::new("Behemoth", "XXX Years ov Blasphemy"),
						Release::new("Bütcher", "On Fowl of Tyrant Wing"),
						Release::new("Blasphemous", "To Lay Siege and Conquer"),
						Release::new("Living Gate", "Suffer as One"),
						Release::new("Avtotheism", "Reflections of Execrable Stillness"),
						Release::new("Vananidr", "In Silence Descent"),
						Release::new("Nitrogods", "Valley of the Gods"),
						Release::new("Symphony of Heaven", "Ordo Aurum Archeia"),
						Release::new("Motörhead", "We Take No Prisoners (The Singles 1995-2006)"),
						Release::new("Pounder", "Thunderforged"),
						Release::new("Grand Harvest", "Till Förruttnelsen"),
						Release::new("Black Curse", "Burning in Celestial Poison"),
						Release::new("Deadform", "Entrenched in Hell"),
						Release::new("Upiór", "Predator of Fear"),
						Release::new("Bog Wizard", "Journey Through the Dying Lands"),
						Release::new("Visions of Disfigurement", "Vile Mutation"),
						Release::new("Leatherhead", "Leatherhead"),
						Release::new("Centinex", "As You Die"),
						Release::new("Weep", "The Constant Strain of Life"),
						Release::new("Zagan", "Total Suffering"),
						Release::new("Recently Vacated Graves: True Zombie Metal", "Musk of Death"),
						Release::new("Sordide", "Ainsi finit le jour"),
						Release::new("Extermination Dismemberment", "Butcher Basement (Revamp)"),
						Release::new("Sallow Moth", "Vial"),
						Release::new("Draconicon", "A Symphony of Pestilence"),
						Release::new("Mordran", "One​-​and​-​Ninety Years of Darkness"),
						Release::new("The Holy Flesh", "Advocate, Martyr and Redeemer"),
						Release::new("Intöxicated", "Under the Sign of the Red Light"),
						Release::new("Lóstregos", "Nai"),
						Release::new("Solarnaut", "There's A Light In The Blur"),
						Release::new("The Contagion", "Swept into Nothing"),
						Release::new("Traktat", "Dogmatic Accusations"),
					]),
					(26, vec![
						Release::new("Helldrifter", "Dark Descent"),
						Release::new("Weight Shift", "Haled from Aether"),
						Release::new("Darkspell", "Victorious Reminiscent of Darkness"),
						Release::new("Messe Noire", "Ceremonial Death"),
					]),
					(27, vec![
						Release::new("Sukkubys", "Ma'am, Your Son Is Dead"),
						Release::new("Snowman", "Dragon's Heart"),
						Release::new("Pratanallis", "雨色Gentiana"),
					]),
					(28, vec![
						Release::new("Kaivs", "After the Flesh"),
						Release::new("Rotgod", "Polemics and Obscenity - Part 2"),
						Release::new("Mental Torment", "Dead Shot Revival"),
						Release::new("High Inquisitor Woe", "Painted Vision of an Era Forlorn"),
						Release::new("Imagine a Boot", "Fearless Werewolf Killers"),
					]),
					(29, vec![
						Release::new("Vulgar Mephitis", "Demo 2024"),
					]),
					(30, vec![
						Release::new("Necromoon", "War and Obedience"),
						Release::new("Lay of the Autumn", "Of Love and Sorrow"),
						Release::new("Weltschmerz", "III: Non Sequitur"),
						Release::new("DeadRipper", "Nightmare"),
						Release::new("Delusions of Godhood", "Salvation's Withdrawal"),
					]),
					(31, vec![
						Release::new("Thine Inner Sanctum", "The Coming of the Dawn"),
						Release::new("Misantropical Painforest / W.A.I.L.", "Dare to Venture Down to Earth, Father! Perish into Nothingness"),
						Release::new("Aelvica", "Aelvica V: Vengeance"),
						Release::new("Asgrauw", "Oorsprong"),
						Release::new("Sleepless", "Through Endless Black"),
						Release::new("Summoning Death", "Tombs of the Blind Dead"),
						Release::new("Goreatorium", "Vile​-​Lence"),
						Release::new("Alien Carcass", "Entropic Visions of a Celestial Heaven"),
						Release::new("Slechtvalk", "At Death's Gate"),
						Release::new("Sorry...", "Drowned in Misery"),
						Release::new("Shaarimoth", "Devildom"),
						Release::new("Tryblith", "Draconis Maleficium"),
						Release::new("Dead Icarus", "Zealot"),
						Release::new("Dead Nexus", "Call of the Void"),
						Release::new("Mälevolent", "Dark Tranquil Night"),
						Release::new("Kre^u / Ticinum / Strja / Vrim", "Voces Antiqui Sanguinis"),
						Release::new("Holy Death", "Sad But True"),
						Release::new("Foul Body Autopsy", "The Discovery of Witches"),
						Release::new("Sausage Wallet", "Vagpire"),
						Release::new("Nox Terror", "Frostbound Realm of the Dead"),
						Release::new("Picha", "Hecho picha"),
						Release::new("Visonfethacsis", "Waltzes in Daguerreotype"),
						Release::new("Kerbmaldarr", "V")
					]),
				])),
				(Month::November, Releases::from([
					(1, vec![
						Release::new("Firemage", "Ignis"),
						Release::new("Timo Tolkki", "Classical Variations and Themes 2: Ultima Thule"),
						Release::new("Frostbite", "Relentless Grief"),
						Release::new("Brothers of Metal", "Fimbulvinter"),
						Release::new("Bombus", "Your Blood"),
						Release::new("Black Aeons", "Entering the Shadows"),
						Release::new("Slaughter the Giant", "Abomination"),
						Release::new("Carved Memories", "The Moirai"),
						Release::new("Vampirska", "A Liminal Heart Paints the Deepest Shade of Serenity"),
						Release::new("Tommy Concrete", "Unrelapsed"),
						Release::new("Nachtmystium", "Blight Privilege"),
						Release::new("Invictus", "Despair"),
						Release::new("Stahlkeller", "Huckepack"),
						Release::new("Wampyric Rites", "Summoning the Beasts in the Night of Lycanthropic Moon"),
						Release::new("Mánþiel", "Odes Past & Mysticism from the Southern Lands"),
						Release::new("Paganizer", "Flesh Requiem"),
						Release::new("Dragoncorpse", "The Fall of House Abbarath"),
						Release::new("Gravekvlt", "Full Moon Fever"),
						Release::new("Everto Signum", "Beastiary"),
						Release::new("Tribulation", "Sub Rosa in Aeternum"),
						Release::new("Viikate", "Hiljainen"),
						Release::new("Burning Sky", "Despair Of The Damned"),
						Release::new("Skullovich", "The Age of Steel"),
						Release::new("Thyrathen", "Lakonic"),
						Release::new("Necrotic Divinity", "Morbid Fascination"),
						Release::new("Splendidula", "Behind My Semblance"),
						Release::new("Anomalie", "Riverchild"),
						Release::new("Cień", "Maledicto"),
						Release::new("Vessel", "The Somnifer"),
						Release::new("Assassin", "Skullblast"),
						Release::new("Vimbulnatt", "Der dunklen Tugenden. Der Urgrund"),
						Release::new("InnerWish", "Ash of Eternal Flame"),
						Release::new("Epitaph", "Path to Oblivion"),
						Release::new("Anthesis", "Tension Between Rot and Genesis"),
						Release::new("Qaalm", "Grave Impressions of an Unbroken Arc"),
						Release::new("From the Vastland", "Tenebrous Shadow"),
						Release::new("DreamLongDead", "Derelict"),
						Release::new("Rotborn", "Shrapnels of a Panic Spiral"),
						Release::new("Mitochondrion", "Vitriseptome"),
						Release::new("The Bottle Doom Lazy Band", "Clans of the Alphane Moon"),
						Release::new("Dying Hydra", "Strange and Beautiful Things"),
						Release::new("Nolove", "Alone / Forgive me..."),
						Release::new("Cryptic Brood", "Necrotic Flesh Bacteria"),
						Release::new("Methchrist", "Acephalic Thanatocracy"),
						Release::new("Crucifixion Ritual", "Desecration of the Angels"),
						Release::new("Angantyr", "Indsigt"),
						Release::new("Trillion Ton Beryllium Ships", "The Mind Like Fire Unbound"),
						Release::new("Ian Highhill", "Death Sentence"),
						Release::new("Children of the Frost", "Last Winter's Child"),
						Release::new("The Fallen Prophets", "Primordial Instinct"),
						Release::new("Frankenbok", "Demon Tantrum"),
						Release::new("Putridarium", "Necrologia del sadismo: Excerpts from a Deranged Mind"),
						Release::new("Recidivist", "Madness Malformed"),
						Release::new("Drift of Genes", "Room"),
					]),
					(2, vec![
						Release::new("Ethereal", "Downfall"),
						Release::new("Lenguaje de Viboras", "Kira"),
						Release::new("Raptore", "Renaissance"),
						Release::new("Abomination Impurity", "Crawling In The Depth"),
					]),
					(3, vec![
						Release::new("Deadspace", "The Dark Enlightenment"),
						Release::new("Steam Slicer", "Beyond the Rivers"),
					]),
					(6, vec![
						Release::new("Naoki Morioka", "Absolutes"),
					]),
					(7, vec![
						Release::new("Suidakra", "Darkanakrad"),
					]),
					(8, vec![
						Release::new("Tungsten", "The Grand Inferno"),
						Release::new("Distant Past", "Solaris"),
						Release::new("Earthburner", "Permanent Dawn"),
						Release::new("Klone", "The Unseen"),
						Release::new("Molder", "Catastrophic Reconfiguration"),
						Release::new("Make Them Suffer", "Make Them Suffer"),
						Release::new("Sólstafir", "Hin helga kv​ö​l"),
						Release::new("Yoth Iria", "Blazing Inferno"),
						Release::new("Valontuoja", "Luonnon armoilla"),
						Release::new("Ad Vitam Infernal", "Le ballet des anges"),
						Release::new("The Body", "The Crying Out of Things"),
						Release::new("Delain", "Dance with the Devil"),
						Release::new("Witchpit", "Forever Spoken"),
						Release::new("Chaos Invocation", "Wherever We Roam..."),
						Release::new("Paragon", "Metalation"),
						Release::new("Massacre", "Necrolution"),
						Release::new("Witnesses", "Joy"),
						Release::new("Impellitteri", "War Machine"),
						Release::new("Stranger Vision", "Faust - Act​​ I Prelude to Darkness"),
						Release::new("Codespeaker", "Scavenger"),
						Release::new("Alarum", "Recontinue"),
						Release::new("Ershetu", "Yomi"),
						Release::new("Disparaged", "Down the Heavens"),
						Release::new("Moss upon the Skull", "Quest for the Secret Fire"),
						Release::new("Ploughshare", "Second Wound"),
						Release::new("Shrykull", "Beyond Subconscious Realms"),
						Release::new("Valkyrie's Fire", "Ascension"),
						Release::new("Seven Kingdoms", "The Square"),
						Release::new("Legendarium", "For Eternal Glory"),
						Release::new("Tenebrisme", "Sisyphe"),
						Release::new("Nurcry", "Renacer"),
					]),
					(9, vec![
						Release::new("Morgue Walker", "No One Left Alive"),
					]),
					(10, vec![
						Release::new("Hamerhaai", "Tand om Tand"),
					]),
					(11, vec![
						Release::new("Blaze the Thunder", "The Bewildered Herd"),
						Release::new("Forja", "Món oblidat"),
						Release::new("Super Monster Party", "Rage Quit"),
						Release::new("Succumbence", "Succumbence"),
					]),
					(12, vec![
						Release::new("Apocryphal", "Facing the End"),
						Release::new("Gauntlet Rule", "After the Kill"),
						Release::new("Space Mirrors", "Nexus Between Space and Art"),
						Release::new("A la Carte", "Born to Entertain"),
						Release::new("Doubting Thompson", "Lizard Brain Directives"),
					]),
					(13, vec![
						Release::new("Incisor", "Harvester Of Indecent Letany"),
					]),
					(14, vec![
						Release::new("Kromlek", "III-III & Upphaf"),
						Release::new("Lying Figures", "Inheritance"),
					]),
					(15, vec![
						Release::new("Warfarer", "A Tale Beyond the Pale"),
						Release::new("Odyrmos", "The Neverending Journey"),
						Release::new("Aptorian Demon", "Liv tar slutt"),
						Release::new("Thy Catafalque", "XII: A gyönyörü álmok ezután jönnek"),
						Release::new("Toxaemia", "Rejected Souls of Kerberus"),
						Release::new("Veilburner", "The Duality of Decapitation and Wisdom"),
						Release::new("Starchaser", "Into the Great Unknown"),
						Release::new("The Foreshadowing", "New Wave Order"),
						Release::new("The Mosaic Window", "Hemasanctum"),
						Release::new("Mammoth Grinder", "Undying Spectral Resonance"),
						Release::new("Tribal Gaze / Deadbody", "Deadbody / Tribal Gaze"),
						Release::new("Monolithe", "Black Hole District"),
						Release::new("As I Lay Dying", "Through Storms Ahead"),
						Release::new("Faüst", "Death Galore"),
						Release::new("Worm Shepherd", "Hunger"),
						Release::new("Thanatos", "Four Decades of Death"),
						Release::new("Synthwailer", "Cruciform"),
						Release::new("Nolove", "La mort nous a séparés"),
						Release::new("Empires of Eden", "Guardians of Time"),
						Release::new("Time Lurker", "Emprise"),
						Release::new("Trollcave", "Adoration of the Abyssal Trespasser"),
						Release::new("Wasted Youth", "Young and Bored - The Complete Wasted Youth"),
						Release::new("Primal Code", "Opaque Fixation"),
						Release::new("Opus Irae", "Into the Endless Night"),
						Release::new("Spider God", "Possess the Devil"),
						Release::new("Thunder and Lightning", "Of Wrath and Ruin"),
						Release::new("Apocalypse", " Pandæmonium"),
						Release::new("Oriska", "Oriska"),
						Release::new("Violent Definition", "Progressive Obsoletion"),
						Release::new("Sergeant Thunderhoof", "The Ghost of Badon Hill"),
					]),
					(19, vec![
						Release::new("Miseri Silentium", "Live at Darkness Conspiracy 2024"),
						Release::new("Stenched", "Purulence Gushing from the Coffin"),
						Release::new("Chain Wolf / Nuke / Evil Army / Whipstriker", "Metal Punk, Vol. I"),
					]),
					(20, vec![
						Release::new("Sunrot / Body Void", "SUNROT // BODY VOID"),
						Release::new("Machete Tactics", "Infinite Terror"),
					]),
					(21, vec![
						Release::new("Accu§er", "Rebirthless"),
					]),
					(22, vec![
						Release::new("Exuvial", "The Hive Mind Chronicles Part I - Parasitica"),
						Release::new("Opeth", "The Last Will and Testament"),
						Release::new("Fellowship", "The Skies Above Eternity"),
						Release::new("Maat", "From Origin to Decay"),
						Release::new("Defeated Sanity", "Chronicles of Lunacy"),
						Release::new("Silent Winter", "Utopia"),
						Release::new("10,000 Years", "All Quiet on the Final Frontier"),
						Release::new("Sign of the Jackal", "Heavy Metal Survivors"),
						Release::new("Ante-Inferno", "Death's Soliloquy"),
						Release::new("Dawn of Destiny", "IX"),
						Release::new("High Warden", "Astral Iron"),
						Release::new("Iniquitous Savagery", "Edifice of Vicissitudes"),
						Release::new("Orso", "Caffè?"),
						Release::new("Aeon Gods", "King of Gods"),
						Release::new("Tyrannic", "Tyrannic Desolation"),
						Release::new("Panzerfaust", "The Suns of Perdition - Chapter IV: To Shadow Zion"),
						Release::new("Artery", "Last Chance"),
						Release::new("Slať", "Elegie propastná"),
						Release::new("Repuked", "Club Squirting Blood"),
						Release::new("Carnal Savagery", "Graveworms, Cadavers, Coffins and Bones"),
						Release::new("Xandria", "Universal Tales"),
						Release::new("DeadlySins", "Age of Revelation"),
						Release::new("Gutless", "High Impact Violence"),
						Release::new("KingCrown", "Nova Atlantis"),
						Release::new("Golgothan Remains", "Bearer of Light, Matriarch of Death"),
						Release::new("Kruelty", "Profane Usurpation"),
					]),
					(24, vec![
						Release::new("Echoes of Gehenna", "The Dreaming Void"),
					]),
					(25, vec![
						Release::new("Grapeshot", "Oblivion"),
					]),
					(26, vec![
						Release::new("Soulskinner", "Gloryfied by the Light"),
					]),
					(27, vec![
						Release::new("Old Wainds", "Stormheart"),
						Release::new("Empty Throne", "Unholy"),
						Release::new("Endemic", "Fetid Plagues"),
						Release::new("Fessus / Kill the Lord", "Decrowned II: Trinity Ablaze / Pilgrims of Morbidity"),
					]),
					(28, vec![
						Release::new("Völva", "Desires Profane"),
					]),
					(29, vec![
						Release::new("Brimstone", "Brimstone"),
						Release::new("Frostmoon Eclipse", "Funerals of Old"),
						Release::new("Hidden Mothers", "Erosion / Avulsion"),
						Release::new("Noitasapatti", "Sankarin matka"),
						Release::new("Ritual Fog", "But Merely Flesh"),
						Release::new("Vargålder", "Framåt skrider dödens tider"),
						Release::new("Konkhra", "Sad Plight of Lucifer"),
						Release::new("Mezmerizer", "Whispers of Leviathan"),
						Release::new("The Gates of Slumber", "The Gates of Slumber"),
						Release::new("Fire Action", "Until the Heat Dies"),
						Release::new("Nolove", "Nostalgia"),
						Release::new("Inverted Cross", "Eternal Flames of Hell"),
						Release::new("Festergore", "Constellation of Endless Blight"),
						Release::new("Mefitis", "The Skorian // The Greyleer"),
						Release::new("Cryptorium", "Descent into Lunacy"),
						Release::new("Havoc", "The Demos"),
						Release::new("Steel Inferno", "Rush of Power"),
						Release::new("Dark Embrace", "Land of Witches"),
						Release::new("Feral Forms", "Through Demonic Spell"),
						Release::new("Pestilent Hex", "Sorceries of Sanguine & Shadow"),
						Release::new("Vidres a la Sang", "Virtut del desencís"),
						Release::new("Starmonger", "Occultation"),
						Release::new("Nebelkrähe", "Entfremdet (2024)"),
						Release::new("Eard", "Melancholia"),
						Release::new("Scythrow", "Blameless Severed Extremities"),
						Release::new("Filii Nigrantium Infernalium", "Pérfida Contracção do Aço"),
						Release::new("Necronomicon Ex Mortis", "The Mother of Death"),
						Release::new("Thy Legion", "Grand Cosmic Funeral"),
						Release::new("Droid Killer", "Delete Everything"),
						Release::new("Gorgon", "For Those Who Stay"),
					]),
					(30, vec![
						Release::new("Funereality", "Through the Black Holes of the Dead"),
						Release::new("Heathen Deity", "Satan's Kingdom"),
						Release::new("Deheubarth", "Revel in Occult Chambers"),
						Release::new("Duister Maanlicht", "Werken van de duisternis"),
					]),
				])),
				(Month::December, Releases::from([
					(1, vec![
						Release::new("Nordic Twilight", "Nordic Twilight"),
						Release::new("Caixão", "Demos 2017 - 2024"),
					]),
					(5, vec![
						Release::new("Horrorborn", "Illuminating Doom"),
					]),
					(6, vec![
						Release::new("Fermentor", "Release Me"),
						Release::new("Asterise", "Tale of a Wandering Soul"),
						Release::new("Nanowar of Steel", "XX Years of Steel"),
						Release::new("The Old Dead Tree", "Second Thoughts"),
						Release::new("Pillar of Light", "Caldera"),
						Release::new("Nolove", "Nobody Can Save You"),
						Release::new("Tethra", "Withered Heart Standing"),
						Release::new("Night in Gales", "Shadowreaper"),
						Release::new("Within Silence", "The Eclipse of Worlds"),
						Release::new("Tarja", "Rocking Heels: Live at Hellfest"),
						Release::new("Aara", "Eiger"),
						Release::new("Panzerchrist", "Maleficium - Part 1"),
						Release::new("Tales of Blood", "Breath of Repugnance"),
						Release::new("Desert Near the End", "Tides of Time"),
					]),
					(7, vec![
						Release::new("STP", "Maoist Jihad: Death to Collaborators"),
					]),
					(12, vec![
						Release::new("New Mexican Doom Cult", "From the Crypt"),
						Release::new("Kombat / False Mutation", "Monument of Abomination"),
					]),
					(13, vec![
						Release::new("Mörk Gryning", "Fasornas tid"),
						Release::new("Injector", "Endless Scorn"),
						Release::new("A Dead Poem", "Abstract Existence"),
						Release::new("Magic Kingdom", "Blaze of Rage"),
						Release::new("Misanthropy", "The Ever​-​Crushing Weight of Stagnance"),
					]),
					(15, vec![
						Release::new("Grollheim", "Funebres Nuptiae"),
					]),
					(20, vec![
						Release::new("Vinodium", "¿En que mundo vivimos?"),
						Release::new("Lights to Remain", "Damnation"),
						Release::new("Hexenbrett", "Dritte Beschw​ö​rung: Dem Teufel eine Tochter"),
					]),
					(27, vec![
						Release::new("Bolvag", "Sad Dark Descent into the Dungeon Dream"),
						Release::new("Dominum", "The Dead Don't Die"),
					]),
					(28, vec![
						Release::new("Nicolas Waldo", "World on Fire"),
					]),
				])),
            ]),
        };
        compare_calendars(got, want);
        Ok(())
    }

    #[test]
    fn test_2025_calendar_ok() -> Result<()> {
        let client = MockClient::new();

        let got = scrape(&client, 2025)?;

        let want = Calendar {
            year: 2025,
            data: CalendarData::from([
                (
                    Month::January,
                    Releases::from([
                        (
                            1,
                            vec![
                                Release::new("Death Cult 69", "The Way of All Flesh"),
                                Release::new("Estuarine", "Corporeal Furnace"),
                                Release::new("Hazzerd", "The 3rd Dimension"),
                            ],
                        ),
                        (
                            3,
                            vec![
                                Release::new("Aeonian Sorrow", "From the Shadows"),
                                Release::new("Faidra", "Dies Irae"),
                            ],
                        ),
                        (
                            10,
                            vec![Release::new("The Halo Effect", "March of the Unheard")],
                        ),
                        (
                            17,
                            vec![
                                Release::new("Grave Digger", "Bone Collector"),
                                Release::new("Tokyo Blade", "Time Is the Fire"),
                                Release::new("Pestilent Scars", "Meadows of Misfortune"),
                            ],
                        ),
                        (
                            24,
                            vec![
                                Release::new("Harakiri for the Sky", "Scorched Earth"),
                                Release::new(
                                    "Avatarium",
                                    "Between You, God, the Devil and the Dead",
                                ),
                                Release::new("Wardruna", "Birna"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::February,
                    Releases::from([
                        (
                            14,
                            vec![
                                Release::new("Atlas Ashes", "New World"),
                                Release::new("Lacuna Coil", "Sleepless Empire"),
                            ],
                        ),
                        (
                            21,
                            vec![Release::new(
                                "Defiled Serenity",
                                "Within the Slumber of the Mind",
                            )],
                        ),
                        (
                            28,
                            vec![
                                Release::new("Dimman", "Consciousness"),
                                Release::new("Timecode", "La Ruptura Del Equilibrio"),
                            ],
                        ),
                    ]),
                ),
                (
                    Month::March,
                    Releases::from([(28, vec![Release::new("Arch Enemy", "Blood Dynasty")])]),
                ),
            ]),
        };
        compare_calendars(got, want);
        Ok(())
    }
}
