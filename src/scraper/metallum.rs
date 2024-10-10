use std::str::FromStr;

use scraper::{Html, Selector};
use serde::Deserialize;
use tracing::info;

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

pub struct MetallumReleaseParts {
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

        let artists = Html::parse_fragment(release.first().ok_or(Error::NoItem)?)
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
        let month = time::Month::from_str(month).map_err(|_| Error::ParseFail)?;
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
    info!("Scraping The Metal Archives");
    let mut calendar = Calendar::new(year);
    let mut page = 0;

    loop {
        info!("Fetching entries {} to {}", page * 100, page * 100 + 100);

        match client.fetch_metallum(page) {
            Some(releases) => {
                for release in releases.data {
                    let parts = MetallumReleaseParts::from_release(release)?;
                    calendar.add_release(
                        parts.release_date.month(),
                        parts.release_date.day(),
                        Release::new(parts.artist, parts.album).with_metallum(
                            parts.artist_link,
                            parts.album_link,
                            parts.release_type,
                            parts.genre,
                        ),
                    );
                }
            }
            None => break,
        }

        page += 1;
    }

    info!("Calendar created");
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
						Release::new("Blood Red Fog / Verge", "Prism of Darkness / Second Mortification").with_metallum("https://www.metal-archives.com/bands/Blood_Red_Fog/42404", "https://www.metal-archives.com/albums/Blood_Red_Fog_-_Verge/Prism_of_Darkness_-_Second_Mortification/1263837", "Split", "Black Metal | Black Metal"),
						Release::new("War Dogs", "Only the Stars Are Left").with_metallum("https://www.metal-archives.com/bands/War_Dogs/3540441681", "https://www.metal-archives.com/albums/War_Dogs/Only_the_Stars_Are_Left/1265853", "Full-length", "Heavy/Speed Metal"),
						Release::new("Bloodrust / Regicide", "Through Death We Reign").with_metallum("https://www.metal-archives.com/bands/Bloodrust/3540478263", "https://www.metal-archives.com/albums/Bloodrust_-_Regicide/Through_Death_We_Reign/1268630", "Split", "Death Metal | Thrash Metal/Hardcore"),
						Release::new("Mortem", "Ilusión de sangre Pre-demo 1988").with_metallum("https://www.metal-archives.com/bands/Mortem/4234", "https://www.metal-archives.com/albums/Mortem/Ilusi%C3%B3n_de_sangre_Pre-demo_1988/1269512", "Demo", "Death Metal"),
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
						Release::new("Reckless Manslaughter", "Sinking into Filth").with_metallum("https://www.metal-archives.com/bands/Reckless_Manslaughter/3540310538", "https://www.metal-archives.com/albums/Reckless_Manslaughter/Sinking_into_Filth/1262756", "Full-length", "Death Metal"),
						Release::new("Midnightmares", "Shadow People").with_metallum("https://www.metal-archives.com/bands/Midnightmares/3540537876", "https://www.metal-archives.com/albums/Midnightmares/Shadow_People/1268797", "Full-length", "Symphonic Gothic Metal"),
						Release::new("The Fall of Creation", "Enlightenment").with_metallum("https://www.metal-archives.com/bands/The_Fall_of_Creation/3540483665", "https://www.metal-archives.com/albums/The_Fall_of_Creation/Enlightenment/1275846", "Full-length", "Melodic Death/Groove Metal"),
						Release::new("Sissourlet", "Rituals in the Catacombs").with_metallum("https://www.metal-archives.com/bands/Sissourlet/3540545064", "https://www.metal-archives.com/albums/Sissourlet/Rituals_in_the_Catacombs/1279821", "Full-length", "Hardcore/Death Metal"),
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
						Release::new("Elephant Tree / Lowrider", "The Long Forever").with_metallum("https://www.metal-archives.com/bands/Elephant_Tree/3540386663", "https://www.metal-archives.com/albums/Elephant_Tree_-_Lowrider/The_Long_Forever/1257761", "Split", "Doom/Stoner Metal | Stoner Metal/Rock"),
						Release::new("Haliphron", "Anatomy of Darkness").with_metallum("https://www.metal-archives.com/bands/Haliphron/3540522764", "https://www.metal-archives.com/albums/Haliphron/Anatomy_of_Darkness/1258547", "Full-length", "Symphonic Black/Death Metal"),
						Release::new("Blackevil", "Praise the Communion Fire for the Unhallowed Sacrament").with_metallum("https://www.metal-archives.com/bands/Blackevil/3540390999", "https://www.metal-archives.com/albums/Blackevil/Praise_the_Communion_Fire_for_the_Unhallowed_Sacrament/1261696", "Full-length", "Black/Thrash Metal"),
						Release::new("Challenger", "Force of Nature").with_metallum("https://www.metal-archives.com/bands/Challenger/3540452597", "https://www.metal-archives.com/albums/Challenger/Force_of_Nature/1261964", "Full-length", "Heavy/Speed Metal"),
						Release::new("Emasculator", "The Disfigured and the Divine").with_metallum("https://www.metal-archives.com/bands/Emasculator/3540509792", "https://www.metal-archives.com/albums/Emasculator/The_Disfigured_and_the_Divine/1262625", "EP", "Brutal Death Metal"),
						Release::new("Ghosts of Glaciers", "Eternal").with_metallum("https://www.metal-archives.com/bands/Ghosts_of_Glaciers/3540399472", "https://www.metal-archives.com/albums/Ghosts_of_Glaciers/Eternal/1263334", "Full-length", "Blackened Post-Metal"),
						Release::new("Nuclear", "Violent DNA").with_metallum("https://www.metal-archives.com/bands/Nuclear/47481", "https://www.metal-archives.com/albums/Nuclear/Violent_DNA/1263388", "EP", "Thrash Metal"),
						Release::new("Paysage d'Hiver", "Die Berge").with_metallum("https://www.metal-archives.com/bands/Paysage_d'Hiver/13417", "https://www.metal-archives.com/albums/Paysage_d'Hiver/Die_Berge/1263444", "Full-length", "Black Metal, Ambient"),
						Release::new("Leviticus", "MMXXIV").with_metallum("https://www.metal-archives.com/bands/Leviticus/9513", "https://www.metal-archives.com/albums/Leviticus/MMXXIV/1263536", "EP", "Heavy Metal/Hard Rock"),
						Release::new("Schammasch", "The Maldoror Chants: Old Ocean").with_metallum("https://www.metal-archives.com/bands/Schammasch/3540316251", "https://www.metal-archives.com/albums/Schammasch/The_Maldoror_Chants%3A_Old_Ocean/1263598", "Full-length", "Black/Death Metal (early); Avant-garde/Black Metal (later)"),
						Release::new("Autumn's Grief", "Dead Among the Living").with_metallum("https://www.metal-archives.com/bands/Autumn%27s_Grief/3540496003", "https://www.metal-archives.com/albums/Autumn%27s_Grief/Dead_Among_the_Living/1263677", "Full-length", "Symphonic Metal"),
						Release::new("Ancient Curse", "Dimension 5").with_metallum("https://www.metal-archives.com/bands/Ancient_Curse/1699", "https://www.metal-archives.com/albums/Ancient_Curse/Dimension_5/1263717", "Full-length", "Progressive Heavy/Power Metal"),
						Release::new("Mindless Sinner", "Metal Merchants").with_metallum("https://www.metal-archives.com/bands/Mindless_Sinner/23990", "https://www.metal-archives.com/albums/Mindless_Sinner/Metal_Merchants/1264122", "Full-length", "Heavy Metal"),
						Release::new("Disarray", "Religious Disease").with_metallum("https://www.metal-archives.com/bands/Disarray/3540529039", "https://www.metal-archives.com/albums/Disarray/Religious_Disease/1265036", "Full-length", "Thrash Metal"),
						Release::new("Psychonaut 4", "...of Mourning").with_metallum("https://www.metal-archives.com/bands/Psychonaut_4/3540329261", "https://www.metal-archives.com/albums/Psychonaut_4/...of_Mourning/1265317", "Full-length", "Depressive Black Metal/Rock"),
						Release::new("Gigan", "Anomalous Abstractigate Infinitessimus").with_metallum("https://www.metal-archives.com/bands/Gigan/108235", "https://www.metal-archives.com/albums/Gigan/Anomalous_Abstractigate_Infinitessimus/1265799", "Full-length", "Progressive/Technical Death Metal"),
						Release::new("Sentient Horror", "In Service of the Dead").with_metallum("https://www.metal-archives.com/bands/Sentient_Horror/3540420358", "https://www.metal-archives.com/albums/Sentient_Horror/In_Service_of_the_Dead/1265850", "Full-length", "Death Metal"),
						Release::new("Hatchet", "Leave No Soul").with_metallum("https://www.metal-archives.com/bands/Hatchet/75808", "https://www.metal-archives.com/albums/Hatchet/Leave_No_Soul/1265863", "EP", "Thrash Metal"),
						Release::new("Athena XIX", "Everflow Part 1: Frames of Humanity").with_metallum("https://www.metal-archives.com/bands/Athena_XIX/707", "https://www.metal-archives.com/albums/Athena_XIX/Everflow_Part_1%3A_Frames_of_Humanity/1265971", "Full-length", "Progressive/Power Metal"),
						Release::new("Iotunn", "Kinship").with_metallum("https://www.metal-archives.com/bands/Iotunn/3540408017", "https://www.metal-archives.com/albums/Iotunn/Kinship/1266004", "Full-length", "Progressive Power Metal (early); Progressive Melodic Death Metal (later)"),
						Release::new("Gigan", "The Gigan Cassette Box Set").with_metallum("https://www.metal-archives.com/bands/Gigan/108235", "https://www.metal-archives.com/albums/Gigan/The_Gigan_Cassette_Box_Set/1266302", "Boxed set", "Progressive/Technical Death Metal"),
						Release::new("Devin Townsend", "PowerNerd").with_metallum("https://www.metal-archives.com/bands/Devin_Townsend/1245", "https://www.metal-archives.com/albums/Devin_Townsend/PowerNerd/1266334", "Full-length", "Progressive Metal/Rock, Ambient"),
						Release::new("Turkey Vulture", "On The List").with_metallum("https://www.metal-archives.com/bands/Turkey_Vulture/3540483095", "https://www.metal-archives.com/albums/Turkey_Vulture/On_The_List/1266491", "EP", "Stoner/Doom Metal"),
						Release::new("Smoke / Doomsday Profit", "Smoke // Doomsday Profit").with_metallum("https://www.metal-archives.com/bands/Smoke/3540502223", "https://www.metal-archives.com/albums/Smoke_-_Doomsday_Profit/Smoke_--_Doomsday_Profit/1266534", "Split", "Stoner/Doom Metal | Stoner/Doom Metal"),
						Release::new("Taking the Head of Goliath", "Futility of the Flesh").with_metallum("https://www.metal-archives.com/bands/Taking_the_Head_of_Goliath/3540424198", "https://www.metal-archives.com/albums/Taking_the_Head_of_Goliath/Futility_of_the_Flesh/1266668", "EP", "Brutal Death Metal"),
						Release::new("Antipope", "Doors of the Dead").with_metallum("https://www.metal-archives.com/bands/Antipope/85290", "https://www.metal-archives.com/albums/Antipope/Doors_of_the_Dead/1266671", "Full-length", "Progressive Black Metal (early); Progressive/Gothic/Industrial Metal (later)"),
						Release::new("Alex Nunziati", "Impending Catastrophe").with_metallum("https://www.metal-archives.com/bands/Alex_Nunziati/3540506323", "https://www.metal-archives.com/albums/Alex_Nunziati/Impending_Catastrophe/1266673", "Full-length", "Heavy Metal, Thrash Metal"),
						Release::new("Vokonis", "Transitions").with_metallum("https://www.metal-archives.com/bands/Vokonis/3540411114", "https://www.metal-archives.com/albums/Vokonis/Transitions/1267264", "Full-length", "Stoner/Doom Metal"),
						Release::new("Mercyless", "Those Who Reign Below").with_metallum("https://www.metal-archives.com/bands/Mercyless/7544", "https://www.metal-archives.com/albums/Mercyless/Those_Who_Reign_Below/1267629", "Full-length", "Death/Thrash Metal"),
						Release::new("Sedimentum", "Derri​è​re les portes d'une arcane transcendante").with_metallum("https://www.metal-archives.com/bands/Sedimentum/3540455227", "https://www.metal-archives.com/albums/Sedimentum/Derri%E2%80%8B%C3%A8%E2%80%8Bre_les_portes_d%27une_arcane_transcendante/1267941", "EP", "Death Metal"),
						Release::new("Adamantra", "Act III: Pareidolia of Depravity").with_metallum("https://www.metal-archives.com/bands/Adamantra/84533", "https://www.metal-archives.com/albums/Adamantra/Act_III%3A_Pareidolia_of_Depravity/1268265", "Full-length", "Progressive/Power Metal"),
						Release::new("Stilverlight", "Dead Souls").with_metallum("https://www.metal-archives.com/bands/Stilverlight/3540389416", "https://www.metal-archives.com/albums/Stilverlight/Dead_Souls/1268317", "Full-length", "Melodic Power Metal"),
						Release::new("Perfidious", "Savouring His Flesh").with_metallum("https://www.metal-archives.com/bands/Perfidious/3540395457", "https://www.metal-archives.com/albums/Perfidious/Savouring_His_Flesh/1268454", "Full-length", "Death Metal"),
						Release::new("Bloodletter / Grozov / Acid Mass / Ninth Realm", "Faster than the Devil III").with_metallum("https://www.metal-archives.com/bands/Bloodletter/3540386435", "https://www.metal-archives.com/albums/Bloodletter_-_Grozov_-_Acid_Mass", "Split", "Thrash Metal (early); Melodic Thrash Metal (later) | Black Metal (early); Black/Power/Thrash Metal (later) | Thrash Metal | Crossover/Thrash Metal"),
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
						Release::new("Kaivs", "After the Flesh").with_metallum("https://www.metal-archives.com/bands/Kaivs/3540519744", "https://www.metal-archives.com/albums/Kaivs/After_the_Flesh/1255256", "Full-length", "Death Metal"),
						Release::new("Rotgod", "Polemics and Obscenity - Part 2").with_metallum("https://www.metal-archives.com/bands/Rotgod/3540491454", "https://www.metal-archives.com/albums/Rotgod/Polemics_and_Obscenity_-_Part_2/1273061", "EP", "Thrash/Death Metal/Grindcore"),
						Release::new("Mental Torment", "Dead Shot Revival").with_metallum("https://www.metal-archives.com/bands/Mental_Torment/3540358661", "https://www.metal-archives.com/albums/Mental_Torment/Dead_Shot_Revival/1273415", "Full-length", "Death/Funeral Doom Metal"),
						Release::new("High Inquisitor Woe", "Painted Vision of an Era Forlorn").with_metallum("https://www.metal-archives.com/bands/High_Inquisitor_Woe/3540403857", "https://www.metal-archives.com/albums/High_Inquisitor_Woe/Painted_Vision_of_an_Era_Forlorn/1278786", "Full-length", "Doom Metal"),
						Release::new("Imagine a Boot", "Fearless Werewolf Killers").with_metallum("https://www.metal-archives.com/bands/Imagine_a_Boot/3540551374", "https://www.metal-archives.com/albums/Imagine_a_Boot/Fearless_Werewolf_Killers/1279292", "Demo", "Raw Black/Heavy Metal/Oi!"),
					]),
					(29, vec![
						Release::new("Vulgar Mephitis", "Demo 2024").with_metallum("https://www.metal-archives.com/bands/Vulgar_Mephitis/3540508775", "https://www.metal-archives.com/albums/Vulgar_Mephitis/Demo_2024/1279652", "Demo", "Brutal Death Metal"),
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
						Release::new("Ethereal", "Downfall").with_metallum("https://www.metal-archives.com/bands/Ethereal/7428", "https://www.metal-archives.com/albums/Ethereal/Downfall/1256633", "Full-length", "Progressive Gothic/Doom Metal"),
						Release::new("Lenguaje de Viboras", "Kira").with_metallum("https://www.metal-archives.com/bands/Lenguaje_de_Viboras/3540500217", "https://www.metal-archives.com/albums/Lenguaje_de_Viboras/Kira/1262099", "EP", "Sludge/Stoner Metal"),
						Release::new("Raptore", "Renaissance").with_metallum("https://www.metal-archives.com/bands/Raptore/3540383257", "https://www.metal-archives.com/albums/Raptore/Renaissance/1270237", "Full-length", "Heavy Metal"),
						Release::new("Abomination Impurity", "Crawling In The Depth").with_metallum("https://www.metal-archives.com/bands/Abomination_Impurity/3540424519", "https://www.metal-archives.com/albums/Abomination_Impurity/Crawling_In_The_Depth/1274915", "EP", "Brutal Death Metal"),
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
						Release::new("Incisor", "Harvester Of Indecent Letany").with_metallum("https://www.metal-archives.com/bands/Incisor/3540276836", "https://www.metal-archives.com/albums/Incisor/Harvester_Of_Indecent_Letany/1280142", "Compilation", "Death Metal"),
					]),
					(14, vec![
						Release::new("Kromlek", "III-III & Upphaf"),
						Release::new("Lying Figures", "Inheritance"),
					]),
					(15, vec![
						Release::new("Warfarer", "A Tale Beyond the Pale").with_metallum("https://www.metal-archives.com/bands/Warfarer/3540462212", "https://www.metal-archives.com/albums/Warfarer/A_Tale_Beyond_the_Pale/1256668", "Full-length", "Melodic Death/Folk Metal"),
						Release::new("Odyrmos", "The Neverending Journey").with_metallum("https://www.metal-archives.com/bands/Odyrmos/3540490499", "https://www.metal-archives.com/albums/Odyrmos/The_Neverending_Journey/1265060", "Demo", "Atmospheric Black Metal, Dark Ambient"),
						Release::new("Aptorian Demon", "Liv tar slutt").with_metallum("https://www.metal-archives.com/bands/Aptorian_Demon/45799", "https://www.metal-archives.com/albums/Aptorian_Demon/Liv_tar_slutt/1265538", "Full-length", "Black Metal"),
						Release::new("Thy Catafalque", "XII: A gyönyörü álmok ezután jönnek").with_metallum("https://www.metal-archives.com/bands/Thy_Catafalque/31620", "https://www.metal-archives.com/albums/Thy_Catafalque/XII%3A_A_gy%C3%B6ny%C3%B6r%C3%BC_%C3%A1lmok_ezut%C3%A1n_j%C3%B6nnek/1265915", "Full-length", "Avant-garde Metal"),
						Release::new("Toxaemia", "Rejected Souls of Kerberus").with_metallum("https://www.metal-archives.com/bands/Toxaemia/21464", "https://www.metal-archives.com/albums/Toxaemia/Rejected_Souls_of_Kerberus/1266096", "Full-length", "Death Metal"),
						Release::new("Veilburner", "The Duality of Decapitation and Wisdom").with_metallum("https://www.metal-archives.com/bands/Veilburner/3540385778", "https://www.metal-archives.com/albums/Veilburner/The_Duality_of_Decapitation_and_Wisdom/1269329", "Full-length", "Black/Death Metal"),
						Release::new("Starchaser", "Into the Great Unknown").with_metallum("https://www.metal-archives.com/bands/Starchaser/3540505939", "https://www.metal-archives.com/albums/Starchaser/Into_the_Great_Unknown/1269976", "Full-length", "Heavy Metal"),
						Release::new("The Foreshadowing", "New Wave Order").with_metallum("https://www.metal-archives.com/bands/The_Foreshadowing/108312", "https://www.metal-archives.com/albums/The_Foreshadowing/New_Wave_Order/1270024", "Full-length", "Gothic/Doom Metal"),
						Release::new("The Mosaic Window", "Hemasanctum").with_metallum("https://www.metal-archives.com/bands/The_Mosaic_Window/3540494835", "https://www.metal-archives.com/albums/The_Mosaic_Window/Hemasanctum/1270721", "Full-length", "Melodic Black Metal"),
						Release::new("Mammoth Grinder", "Undying Spectral Resonance").with_metallum("https://www.metal-archives.com/bands/Mammoth_Grinder/3540294181", "https://www.metal-archives.com/albums/Mammoth_Grinder/Undying_Spectral_Resonance/1270876", "EP", "Hardcore Punk/Sludge Metal (early); Death Metal/Hardcore (later)"),
						Release::new("Tribal Gaze / Deadbody", "Deadbody / Tribal Gaze").with_metallum("https://www.metal-archives.com/bands/Tribal_Gaze/3540483981", "https://www.metal-archives.com/albums/Tribal_Gaze_-_Deadbody/Deadbody_-_Tribal_Gaze/1272378", "Split", "Death Metal | Death Metal/Hardcore"),
						Release::new("Monolithe", "Black Hole District").with_metallum("https://www.metal-archives.com/bands/Monolithe/13707", "https://www.metal-archives.com/albums/Monolithe/Black_Hole_District/1272417", "Full-length", "Funeral Doom Metal (early); Melodic Death/Doom Metal (later)"),
						Release::new("As I Lay Dying", "Through Storms Ahead").with_metallum("https://www.metal-archives.com/bands/As_I_Lay_Dying/20825", "https://www.metal-archives.com/albums/As_I_Lay_Dying/Through_Storms_Ahead/1272713", "Full-length", "Metalcore"),
						Release::new("Faüst", "Death Galore").with_metallum("https://www.metal-archives.com/bands/Fa%C3%BCst/3540473797", "https://www.metal-archives.com/albums/Fa%C3%BCst/Death_Galore/1272986", "Full-length", "Thrash Metal"),
						Release::new("Worm Shepherd", "Hunger").with_metallum("https://www.metal-archives.com/bands/Worm_Shepherd/3540500546", "https://www.metal-archives.com/albums/Worm_Shepherd/Hunger/1274050", "Full-length", "Symphonic Deathcore"),
						Release::new("Thanatos", "Four Decades of Death").with_metallum("https://www.metal-archives.com/bands/Thanatos/293", "https://www.metal-archives.com/albums/Thanatos/Four_Decades_of_Death/1274598", "Compilation", "Death/Thrash Metal"),
						Release::new("Synthwailer", "Cruciform").with_metallum("https://www.metal-archives.com/bands/Synthwailer/3540486233", "https://www.metal-archives.com/albums/Synthwailer/Cruciform/1275480", "Full-length", "Symphonic Power/Heavy Metal"),
						Release::new("Nolove", "La mort nous a séparés").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/La_mort_nous_a_s%C3%A9par%C3%A9s/1275727", "Single", "Experimental/Depressive Black Metal, Post-Rock"),
						Release::new("Empires of Eden", "Guardians of Time").with_metallum("https://www.metal-archives.com/bands/Empires_of_Eden/3540282087", "https://www.metal-archives.com/albums/Empires_of_Eden/Guardians_of_Time/1277228", "Full-length", "Melodic Power Metal"),
						Release::new("Time Lurker", "Emprise").with_metallum("https://www.metal-archives.com/bands/Time_Lurker/3540419145", "https://www.metal-archives.com/albums/Time_Lurker/Emprise/1277910", "Full-length", "Atmospheric Black Metal"),
						Release::new("Trollcave", "Adoration of the Abyssal Trespasser").with_metallum("https://www.metal-archives.com/bands/Trollcave/3540499736", "https://www.metal-archives.com/albums/Trollcave/Adoration_of_the_Abyssal_Trespasser/1278354", "EP", "Funeral Doom/Death Metal"),
						Release::new("Wasted Youth", "Young and Bored - The Complete Wasted Youth").with_metallum("https://www.metal-archives.com/bands/Wasted_Youth/14351", "https://www.metal-archives.com/albums/Wasted_Youth/Young_and_Bored_-_The_Complete_Wasted_Youth/1278681", "Compilation", "Hardcore Punk (early); Thrash Metal (later)"),
						Release::new("Primal Code", "Opaque Fixation").with_metallum("https://www.metal-archives.com/bands/Primal_Code/3540509020", "https://www.metal-archives.com/albums/Primal_Code/Opaque_Fixation/1278865", "Full-length", "Death Metal"),
						Release::new("Opus Irae", "Into the Endless Night").with_metallum("https://www.metal-archives.com/bands/Opus_Irae/3540405662", "https://www.metal-archives.com/albums/Opus_Irae/Into_the_Endless_Night/1279021", "Full-length", "Symphonic Black Metal"),
						Release::new("Spider God", "Possess the Devil").with_metallum("https://www.metal-archives.com/bands/Spider_God/3540476120", "https://www.metal-archives.com/albums/Spider_God/Possess_the_Devil/1279119", "Full-length", "Melodic Black Metal"),
						Release::new("Thunder and Lightning", "Of Wrath and Ruin").with_metallum("https://www.metal-archives.com/bands/Thunder_and_Lightning/50661", "https://www.metal-archives.com/albums/Thunder_and_Lightning/Of_Wrath_and_Ruin/1279545", "Full-length", "Melodic Power Metal"),
						Release::new("Apocalypse", " Pandæmonium").with_metallum("https://www.metal-archives.com/bands/Apocalypse/3540449107", "https://www.metal-archives.com/albums/Apocalypse/Pand%C3%A6monium/1279607", "Full-length", "Black/Viking Metal, Death/Thrash Metal"),
						Release::new("Oriska", "Oriska").with_metallum("https://www.metal-archives.com/bands/Oriska/3540534992", "https://www.metal-archives.com/albums/Oriska/Oriska/1279811", "Full-length", "Post-Black/Doom Metal"),
						Release::new("Violent Definition", "Progressive Obsoletion").with_metallum("https://www.metal-archives.com/bands/Violent_Definition/3540303408", "https://www.metal-archives.com/albums/Violent_Definition/Progressive_Obsoletion/1280450", "Full-length", "Thrash Metal"),
						Release::new("Sergeant Thunderhoof", "The Ghost of Badon Hill").with_metallum("https://www.metal-archives.com/bands/Sergeant_Thunderhoof/3540379484", "https://www.metal-archives.com/albums/Sergeant_Thunderhoof/The_Ghost_of_Badon_Hill/1280491", "Full-length", "Psychedelic Stoner/Doom Metal"),
					]),
					(19, vec![
						Release::new("Miseri Silentium", "Live at Darkness Conspiracy 2024").with_metallum("https://www.metal-archives.com/bands/Miseri_Silentium/3540501270", "https://www.metal-archives.com/albums/Miseri_Silentium/Live_at_Darkness_Conspiracy_2024/1278607", "Live album", "Black Metal"),
						Release::new("Stenched", "Purulence Gushing from the Coffin").with_metallum("https://www.metal-archives.com/bands/Stenched/3540526785", "https://www.metal-archives.com/albums/Stenched/Purulence_Gushing_from_the_Coffin/1279043", "Full-length", "Death Metal"),
						Release::new("Chain Wolf / Nuke / Evil Army / Whipstriker", "Metal Punk, Vol. I").with_metallum("https://www.metal-archives.com/bands/Chain_Wolf/3540479708", "https://www.metal-archives.com/albums/Chain_Wolf_-_Nuke_-_Evil_Army_-_Whipstriker/Metal_Punk%2C_Vol._I/1279083", "Split", "Thrash Metal/Crossover | Speed Metal | Thrash Metal | Heavy/Speed Metal"),
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
						Release::new("Fermentor", "Release Me").with_metallum("https://www.metal-archives.com/bands/Fermentor/3540404731", "https://www.metal-archives.com/albums/Fermentor/Release_Me/1273180", "Full-length", "Death Metal"),
						Release::new("Asterise", "Tale of a Wandering Soul").with_metallum("https://www.metal-archives.com/bands/Fermentor/3540404731", "https://www.metal-archives.com/albums/Fermentor/Release_Me/1273180", "Full-length", "Death Metal"),
						Release::new("Nanowar of Steel", "XX Years of Steel").with_metallum("https://www.metal-archives.com/bands/Nanowar_of_Steel/3540261755", "https://www.metal-archives.com/albums/Nanowar_of_Steel/XX_Years_of_Steel/1274421", "Compilation", "Heavy/Power Metal/Hard Rock"),
						Release::new("The Old Dead Tree", "Second Thoughts").with_metallum("https://www.metal-archives.com/bands/The_Old_Dead_Tree/8297", "https://www.metal-archives.com/albums/The_Old_Dead_Tree/Second_Thoughts/1274538", "Full-length", "Gothic Metal"),
						Release::new("Pillar of Light", "Caldera").with_metallum("https://www.metal-archives.com/bands/Pillar_of_Light/3540548599", "https://www.metal-archives.com/albums/Pillar_of_Light/Caldera/1274923", "Full-length", "Doom/Sludge/Post-Metal"),
						Release::new("Nolove", "Nobody Can Save You").with_metallum("https://www.metal-archives.com/bands/Nolove/3540531420", "https://www.metal-archives.com/albums/Nolove/Nobody_Can_Save_You/1276304", "Full-length", "Experimental/Depressive Black Metal, Post-Rock"),
						Release::new("Tethra", "Withered Heart Standing").with_metallum("https://www.metal-archives.com/bands/Tethra/3540317188", "https://www.metal-archives.com/albums/Tethra/Withered_Heart_Standing/1276477", "Full-length", "Death/Doom Metal"),
						Release::new("Night in Gales", "Shadowreaper").with_metallum("https://www.metal-archives.com/bands/Night_in_Gales/817", "https://www.metal-archives.com/albums/Night_in_Gales/Shadowreaper/1276707", "Full-length", "Melodic Death Metal"),
						Release::new("Within Silence", "The Eclipse of Worlds").with_metallum("https://www.metal-archives.com/bands/Within_Silence/3540396066", "https://www.metal-archives.com/albums/Within_Silence/The_Eclipse_of_Worlds/1276946", "Full-length", "Power Metal"),
						Release::new("Tarja", "Rocking Heels: Live at Hellfest").with_metallum("https://www.metal-archives.com/bands/Tarja/110710", "https://www.metal-archives.com/albums/Tarja/Rocking_Heels%3A_Live_at_Hellfest/1277113", "Live album", "Symphonic Metal/Rock, Neoclassical"),
						Release::new("Aara", "Eiger").with_metallum("https://www.metal-archives.com/bands/Aara/3540451086", "https://www.metal-archives.com/albums/Aara/Eiger/1277867", "Full-length", "Atmospheric Black Metal"),
						Release::new("Panzerchrist", "Maleficium - Part 1").with_metallum("https://www.metal-archives.com/bands/Panzerchrist/2864", "https://www.metal-archives.com/albums/Panzerchrist/Maleficium_-_Part_1/1278810", "Full-length", "Death/Black Metal"),
						Release::new("Tales of Blood", "Breath of Repugnance").with_metallum("https://www.metal-archives.com/bands/Tales_of_Blood/10656", "https://www.metal-archives.com/albums/Tales_of_Blood/Breath_of_Repugnance/1278857", "Full-length", "Death Metal"),
						Release::new("Desert Near the End", "Tides of Time").with_metallum("https://www.metal-archives.com/bands/Desert_Near_the_End/3540370893", "https://www.metal-archives.com/albums/Desert_Near_the_End/Tides_of_Time/1280001", "Full-length", "Power/Thrash Metal"),
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
