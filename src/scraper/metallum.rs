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

        let artist_fragment: Html = Html::parse_fragment(release.get(0).ok_or(Error::NoItem)?);
        let (artist, artist_link) = if let Some(el) = artist_fragment.select(&selector).next() {
            let artist = el.text().collect::<Vec<_>>().join("");
            let artist_link = el.value().attr("href").ok_or(Error::NoItem)?.to_string();
            (artist, artist_link)
        } else {
            return Err(Error::NoItem);
        };

        let album_fragment = Html::parse_fragment(release.get(1).ok_or(Error::NoItem)?);
        let (album, album_link) = if let Some(el) = album_fragment.select(&selector).next() {
            let album = el.text().collect::<Vec<_>>().join("");
            let album_link = el.value().attr("href").ok_or(Error::NoItem)?.to_string();
            (album, album_link)
        } else {
            return Err(Error::NoItem);
        };

        let release_date = release.get(4).ok_or(Error::NoItem)?.to_string();
        let release_date = release_date
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
					(5, vec![
						Release::new("Nangsan", "Altered Senses"),
                        Release::new("Jordskaelv", "Grauw is mijn Vaandel"),
                        Release::new("Castle of Solitude", "Futility of Existence"),
                        Release::new("Forgot", "Грянул гром"),
                        Release::new("Saatane", "Manifestations of the Black Sun"),
                        Release::new("From a Spectral Life", "s There Any Real World?"),
                        Release::new("A Karmic Gray", "Global Homie!"),
                        Release::new("A Karmic Gray	", "Divinely Lit (Christ)"),
                        Release::new("Skurk", "Satanic Power"),
                        Release::new("Phyllomedusa", "When It Hurts, It Feels Right"),
                        Release::new("Catharsis", "M-VIII B.C."),
                        Release::new("Devouring Famine", "Blind Eyes Wide Open"),
                        Release::new("Avliv", "Nostalgic"),
                        Release::new("Void of Nothingness", "The Eternal Now"),
                        Release::new("Odious", "Equilibrium Tool"),
					]),
                    (6, vec![
						Release::new("Aeonian Sorrow", "From the Shadows"),
						Release::new("Faidra", "Dies Irae"),
					]),
					(10, vec![
						Release::new("The Halo Effect", "March of the Unheard"),
					]),
					(17, vec![
						Release::new("Grave Digger", "Bone Collector"),
                        Release::new("Tokyo Blade", "Time Is the Fire"),
                        Release::new("Pestilent Scars", "Meadows of Misfortune"),
					]),
                    (24, vec![
						Release::new("Harakiri for the Sky", "Scorched Earth"),
                        Release::new("Avatarium", "Between You, God, the Devil and the Dead"),
                        Release::new("Wardruna", "Birna"),
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
						Release::new("Nuke / Evil Army / Whipstriker / Chain Wolf", "Metal Punk, Vol. I"),
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
						Release::new("Kill the Lord / Fessus", "Decrowned II: Trinity Ablaze / Pilgrims of Morbidity"),
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
						Release::new("Deheubarth", "Grand Cosmic Funeral"),
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
						Release::new("Dominum to Remain", "The Dead Don't Die"),
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
				(Month::January, Releases::from([
					(1, vec![
						Release::new("Death Cult 69", "The Way of All Flesh"),
						Release::new("Estuarine", "Corporeal Furnace"),
						Release::new("Hazzerd", "The 3rd Dimension"),
					]),
					(3, vec![
						Release::new("Aeonian Sorrow", "From the Shadows"),
						Release::new("Faidra", "Dies Irae"),
					]),
					(10, vec![
						Release::new("The Halo Effect", "March of the Unheard"),
					]),
					(17, vec![
						Release::new("Grave Digger", "Bone Collector"),
						Release::new("Tokyo Blade", "Time Is the Fire"),
						Release::new("Pestilent Scars", "Meadows of Misfortune"),
					]),
					(24, vec![
						Release::new("Harakiri for the Sky", "Scorched Earth"),
						Release::new("Avatarium", "Between You, God, the Devil and the Dead"),
						Release::new("Wardruna", "Birna"),
					]),
				])),
				(Month::February, Releases::from([
					(14, vec![
						Release::new("Atlas Ashes", "New World"),
						Release::new("Lacuna Coil", "Sleepless Empire"),
					]),
					(21, vec![
						Release::new("Defiled Serenity", "Within the Slumber of the Mind"),
					]),
					(28, vec![
						Release::new("Dimman", "Consciousness"),
						Release::new("Timecode", "La Ruptura Del Equilibrio"),
					]),
				])),
				(Month::March, Releases::from([
					(28, vec![
						Release::new("Arch Enemy", "Blood Dynasty"),
					]),
				])),
            ]),
        };
        compare_calendars(got, want);
        Ok(())
    }
}
