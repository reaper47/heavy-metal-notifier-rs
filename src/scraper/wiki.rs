use std::collections::HashMap;

use scraper::{ElementRef, Html, Selector};

use crate::calendar::{Calendar, Month, Release};

pub fn extract_calendar(doc: Html) -> Calendar {
    let mut calendar = Calendar::new();

    let mut current_day: u8 = 1;
    let mut current_artist = "".to_string();

    let tables: HashMap<Month, &str> = HashMap::from([
        (Month::January, "#table_January"),
        (Month::February, "#table_February"),
        (Month::March, "#table_March"),
        (Month::April, "#table_April"),
        (Month::May, "#table_May"),
        (Month::June, "#table_June"),
        (Month::July, "#table_July"),
        (Month::August, "#table_August"),
        (Month::September, "#table_September"),
        (Month::October, "#table_October"),
        (Month::November, "#table_November"),
        (Month::December, "#table_December"),
    ]);
    tables.iter().for_each(|(&month, &table_id)| {
        let selector = &Selector::parse(&table_id).unwrap();
        let tables = doc.select(selector).collect::<Vec<_>>();
        match tables.len() {
            2 if month == Month::November => {
                process_table(
                    tables[0],
                    &mut calendar,
                    Month::October,
                    &mut current_day,
                    &mut current_artist,
                );
                process_table(
                    tables[1],
                    &mut calendar,
                    month,
                    &mut current_day,
                    &mut current_artist,
                );
            }
            1 => process_table(
                tables[0],
                &mut calendar,
                month,
                &mut current_day,
                &mut current_artist,
            ),
            _ => {}
        }
    });

    calendar
}

fn process_table(
    table: ElementRef,
    calendar: &mut Calendar,
    month: Month,
    current_day: &mut u8,
    current_artist: &mut String,
) {
    let selector = &Selector::parse("tbody tr").unwrap();
    table.select(selector).for_each(|row| {
        let cells = row.child_elements().collect::<Vec<_>>();
        match cells.len() {
            1 => {
                let album = cells[0].text().collect::<String>();
                calendar.add_release(
                    month,
                    *current_day,
                    Release::new(current_artist.clone(), album.trim()),
                )
            }
            2 => {
                let artist = cells[0].text().collect::<String>();
                let artist = artist.trim();
                *current_artist = String::from(artist);

                let album = cells[1].text().collect::<String>();
                let album = album.trim();

                calendar.add_release(month, *current_day, Release::new(artist, album))
            }
            3 => {
                let day: Result<u8, _> = cells[0].text().collect::<String>().trim().parse();
                if let Ok(day) = day {
                    *current_day = day;
                }

                let artist = cells[1].text().collect::<String>();
                let artist = artist.trim();
                *current_artist = String::from(artist);

                let album = cells[2].text().collect::<String>();
                let album = album.trim();

                if artist != "Artist" {
                    calendar.add_release(month, *current_day, Release::new(artist, album.trim()));
                }
            }
            _ => {}
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        calendar::{CalendarData, Releases},
        scraper::client::MockClient,
    };

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[tokio::test]
    async fn test_2022_calendar_ok() -> Result<()> {
        let client = MockClient {};

        let got = client.scrape(2022).await?;

        let want = Calendar {
            data: CalendarData::from([
                (Month::January, Releases::from([
					(7, vec![
						Release::new("Atrocity", "Unspoken Names (Demo 1991) (EP)"),
						Release::new("Infected Rain", "Ecdysis"),
						Release::new("Schwarzer Engel", "Sieben"),
                    ]),
					(14,vec![
						Release::new("Enterprise Earth", "The Chosen"),
						Release::new("Fit for an Autopsy", "Oh What the Future Holds"),
						Release::new("Ilium", "Quantum Evolution Event (EP)"),
						Release::new("Shadow of Intent", "Elegy"),
						Release::new("Skillet", "Dominion"),
						Release::new("Tony Martin", "Thorns"),
						Release::new("Underoath", "Voyeurist"),
                    ]),
					(21, vec![
						Release::new("Ashes of Ares", "Emperors and Fools"),
						Release::new("Asking Alexandria", "Never Gonna Learn (EP)"),
						Release::new("Battle Beast", "Circus of Doom"),
						Release::new("Boris", "W"),
						Release::new("Confess", "Revenge at All Costs"),
						Release::new("Giant", "Shifting Time"),
						Release::new("Iced Earth", "A Narrative Soundscape"),
						Release::new("Kissin' Dynamite", "Not the End of the Road"),
						Release::new("Sonata Arctica", "Acoustic Adventures – Volume One"),
						Release::new("Tokyo Blade", "Fury"),
					]),
					(28, vec![
						Release::new("Celeste", "Assassine(s)"),
						Release::new("Cloakroom", "Dissolution Wave"),
						Release::new("Dawn of Solace", "Flames of Perdition"),
						Release::new("Emerald Sun", "Kingdom of Gods"),
						Release::new("Krallice", "Crystalline Exhaustion"),
						Release::new("Lana Lane", "Neptune Blue"),
						Release::new("The Last Ten Seconds of Life", "The Last Ten Seconds of Life"),
						Release::new("Lawnmower Deth", "Blunt Cutters"),
						Release::new("Praying Mantis", "Katharsis"),
						Release::new("The Quill", "Live, New, Borrowed, Blue (compilation album)"),
						Release::new("Steve Vai", "Inviolate"),
					]),
                ])),
				(Month::February, Releases::from([
					(4, vec![
						Release::new("Abysmal Dawn", "Nightmare Frontier (EP)"),
						Release::new("Bevar Sea", "The Timeless Zone"),
						Release::new("Hed PE", "Califas Worldwide"),
						Release::new("Korn", "Requiem"),
						Release::new("Mystic Circle", "Mystic Circle"),
						Release::new("Persefone", "Metanoia"),
						Release::new("Rolo Tomassi", "Where Myth Becomes Memory"),
						Release::new("Saxon", "Carpe Diem"),
						Release::new("Venom Prison", "Erebos"),
					]),
					(11, vec![
						Release::new("Amorphis", "Halo"),
						Release::new("Author & Punisher", "Krüller"),
						Release::new("Cult of Luna", "The Long Road North"),
						Release::new("Girish and The Chronicles", "Hail to the Heroes"),
						Release::new("Napalm Death", "Resentment Is Always Seismic – A Final Throw of Throes (mini-album)"),
						Release::new("Once Human", "Scar Weaver"),
						Release::new("The Silent Wedding", "Ego Path"),
						Release::new("Slash feat. Myles Kennedy & the Conspirators", "4"),
						Release::new("Tersivel", "To the Orphic Void"),
						Release::new("Voivod", "Synchro Anarchy"),
						Release::new("Zeal & Ardor", "Zeal & Ardor"),
					]),
					(18, vec![
						Release::new("Annihilator", "Metal II"),
						Release::new("Bloodywood", "Rakshak"),
						Release::new("Dagoba", "By Night"),
						Release::new("Esprit D'Air", "Oceans"),
						Release::new("Immolation", "Acts of God"),
						Release::new("Matt Pike", "Pike vs. the Automaton"),
						Release::new("Nightrage", "Abyss Rising"),
						Release::new("Spirits of Fire", "Embrace the Unknown"),
						Release::new("Star One", "Revel in Time"),
					]),
					(25, vec![
						Release::new("Allegaeon", "Damnum"),
						Release::new("Bad Omens", "The Death of Peace of Mind"),
						Release::new("Blood Incantation", "Timewave Zero"),
						Release::new("Corey Taylor", "CMFB ...Sides (covers album)"),
						Release::new("Diablo", "When All the Rivers Are Silent"),
						Release::new("Eight Bells", "Legacy of Ruin"),
						Release::new("George \"Corpsegrinder\" Fisher", "Corpsegrinder"),
						Release::new("Guns N' Roses", "Hard Skool (EP)"),
						Release::new("HammerFall", "Hammer of Dawn"),
						Release::new("Metalucifer", "Heavy Metal Ninja (mini-album)"),
						Release::new("Scorpions", "Rock Believer"),
						Release::new("Shape of Despair", "Return to the Void"),
						Release::new("Svartsot", "Kumbl"),
						Release::new("Tygers of Pan Tang", "A New Heartbeat (EP)"),
					]),
				])),
				(Month::March, Releases::from([
					(4, vec![
						Release::new("10 Years", "Deconstructed"),
						Release::new("Crowbar", "Zero and Below"),
						Release::new("Eric Wagner", "In the Lonely Light of Mourning"),
						Release::new("Flaw", "Revival (covers album)"),
						Release::new("Sabaton", "The War to End All Wars"),
						Release::new("Sunflower Dead", "March of the Leper"),
						Release::new("Ty Tabor", "Shades"),
						Release::new("Vein.fm", "This World Is Going to Ruin You"),
						Release::new("Vio-lence", "Let the World Burn (EP)"),
						Release::new("Warrior Soul", "Out on Bail"),
					]),
					(5, vec![
						Release::new("King Gizzard & the Lizard Wizard", "Made in Timeland"),
						Release::new("Troglodyte", "The Hierarchical Ecological Succession: Welcome to the Food Chain"),
					]),
					(11, vec![
						Release::new("Black Pantera", "Ascensão"),
						Release::new("Brandon Boyd", "Echoes and Cocoons"),
						Release::new("Claustrofobia", "Unleeched"),
						Release::new("Cloven Hoof", "Time Assassin"),
						Release::new("Ghost", "Impera"),
						Release::new("Grim Reaper", "Reaping the Whirlwind (live album)"),
						Release::new("Kiss", "Off the Soundboard: Live in Virginia Beach (live album)"),
						Release::new("Love/Hate", "HELL, CA"),
						Release::new("New Horizon", "Gate of the Gods"),
						Release::new("Shaman's Harvest", "Rebelator"),
						Release::new("Wolves at the Gate", "Eulogies"),
					]),
					(12, vec![
						Release::new("Dog Fashion Disco", "Cult Classic"),
					]),
					(18, vec![
						Release::new("Agathodaimon", "The Seven"),
						Release::new("Dark Funeral", "We Are the Apocalypse"),
						Release::new("Dawn of Ashes", "Scars of the Broken"),
						Release::new("Manigance", "Le bal des ombres"),
						Release::new("Ronni Le Tekrø", "Bigfoot TV"),
						Release::new("Ronnie Atkins", "Make It Count"),
						Release::new("Stabbing Westward", "Chasing Ghosts"),
						Release::new("Týr", "A Night at the Nordic House (live album)"),
					]),
					(23, vec![
						Release::new("Deathspell Omega", "The Long Defeat"),
					]),
					(25, vec![
						Release::new("Abbath", "Dread Reaver"),
						Release::new("Animals as Leaders", "Parrhesia"),
						Release::new("Architects", "For Those That Wish to Exist at Abbey Road (live album)"),
						Release::new("BillyBio", "Leaders and Liars"),
						Release::new("Crystal Viper", "The Last Axeman (mini-album)"),
						Release::new("Eucharist", "I Am the Void"),
						Release::new("Hardcore Superstar", "Abrakadabra"),
						Release::new("Killing Joke", "Lord of Chaos (EP)"),
						Release::new("Michael Romeo", "War of the Worlds, Pt. 2"),
						Release::new("Pist.On", "Cold World EP (EP)"),
						Release::new("Reckless Love", "Turborider"),
					]),
				])),
				(Month::April, Releases::from([
					(1, vec![
						Release::new("Centinex", "The Pestilence (EP)"),
						Release::new("Kublai Khan", "Lowest Form of Animal (EP)"),
						Release::new("Lords of the Trident", "The Offering"),
						Release::new("Meshuggah", "Immutable"),
						Release::new("Nekrogoblikon", "The Fundamental Slimes and Humours"),
						Release::new("Satan", "Earth Infernal"),
						Release::new("Trick or Treat", "Creepy Symphonies"),
						Release::new("Wolf", "Shadowland"),
					]),
					(8, vec![
						Release::new("Destruction", "Diabolical"),
						Release::new("Hällas", "Isle of Wisdom"),
						Release::new("Incite", "Wake Up Dead"),
						Release::new("Inglorious", "MMXXI Live at the Phoenix (live album)"),
						Release::new("Mors Principium Est", "Liberate the Unborn Inhumanity (compilation album)"),
						Release::new("Papa Roach", "Ego Trip"),
						Release::new("Terzij de Horde", "In One of These, I Am Your Enemy"),
						Release::new("Treat", "The Endgame"),
					]),
					(14, vec![
						Release::new("Psychostick", "... and Stuff (compilation album)"),
					]),
					(15, vec![
						Release::new("Abated Mass of Flesh", "The Existence of Human Suffering"),
						Release::new("Axel Rudi Pell", "Lost XXIII"),
						Release::new("Cancer Bats", "Psychic Jailbreak"),
						Release::new("Grand Belial's Key", "Kohanic Charmers"),
						Release::new("JBO", "Planet Pink"),
						Release::new("Månegarm", "Ynglingaättens Öde"),
						Release::new("Monuments", "In Stasis"),
						Release::new("Nazareth", "Surviving the Law"),
						Release::new("Powerglove", "Flawless Victory (EP)"),
						Release::new("Ronnie Romero", "Raised on Radio (covers album)"),
						Release::new("Semblant", "Vermilion Eclipse"),
						Release::new("These Arms Are Snakes", "Duct Tape & Shivering Crows (compilation album)"),
					]),
					(22, vec![
						Release::new("Archgoat", "All Christianity Ends (EP)"),
						Release::new("Caliban", "Dystopia"),
						Release::new("Die Apokalyptischen Reiter", "Wilde Kinder"),
						Release::new("King Gizzard & the Lizard Wizard", "Omnium Gatherum"),
						Release::new("Märvel", "Graces Came with Malice"),
						Release::new("Miseration", "Black Miracles and Dark Wonders"),
						Release::new("Northlane", "Obsidian"),
						Release::new("Ocean Grove", "Up in the Air Forever"),
						Release::new("Primus", "Conspiranoid (EP)"),
						Release::new("Skull Fist", "Paid in Full"),
						Release::new("Somali Yacht Club", "The Space"),
						Release::new("Speckmann Project", "Fiends of Emptiness"),
						Release::new("Udo Dirkschneider", "My Way (covers album)"),
					]),
					(23, vec![
						Release::new("Charlie Benante", "Moving Pitchers (EP)"),
						Release::new("Kirk Hammett", "Portals (EP)"),
						Release::new("The Lord", "Forest Nocturne"),
					]),
					(29, vec![
						Release::new("Al-Namrood", "Worship the Degenerate"),
						Release::new("Crashdïet", "Automaton"),
						Release::new("The Gathering", "Beautiful Distortion"),
						Release::new("Helms Alee", "Keep This Be the Way"),
						Release::new("Heriot", "Profound Morality (EP)"),
						Release::new("Rammstein", "Zeit"),
						Release::new("Thunder", "Dopamine"),
						Release::new("Void of Vision", "Chronicles II: Heaven (EP)"),
						Release::new("Vulcano", "Stone Orange"),
						Release::new("Watain", "The Agony & Ecstasy of Watain"),
					]),
				])),
				(Month::May, Releases::from([
					(6, vec![
						Release::new("Depressed Mode", "Decade of Silence"),
						Release::new("Fozzy", "Boombox"),
						Release::new("Halestorm", "Back from the Dead"),
						Release::new("Ibaraki", "Rashomon"),
						Release::new("Jani Liimatainen", "My Father's Son"),
						Release::new("Jeff Scott Soto", "Complicated"),
						Release::new("Lord of the Lost", "The Heartbeat of the Devil (EP)"),
						Release::new("Puppy", "Pure Evil"),
						Release::new("Three Days Grace", "Explosions"),
						Release::new("Ufomammut", "Fenice"),
						Release::new("Upon a Burning Body", "Fury"),
						Release::new("Windwaker", "Love Language"),
					]),
					(13, vec![
						Release::new("Demiricous", "III: Chaotic Lethal"),
						Release::new("Graham Bonnet Band", "Day Out in Nowhere"),
						Release::new("Jungle Rot", "A Call to Arms"),
						Release::new("Misery Index", "Complete Control"),
						Release::new("Primitive Man", "Insurmountable (EP)"),
						Release::new("Visions of Atlantis", "Pirates"),
						Release::new("Zero Hour", "Agenda 21"),
					]),
					(18, vec![
						Release::new("Novelbright", "Assort"),
					]),
					(20, vec![
						Release::new("Anvil", "Impact Is Imminent"),
						Release::new("Blut Aus Nord", "Disharmonium – Undreamable Abysses"),
						Release::new("Cave In", "Heavy Pendulum"),
						Release::new("Chuck Wright's Sheltering Sky", "Chuck Wright's Sheltering Sky"),
						Release::new("Evergrey", "A Heartless Portrait (The Orphean Testament)"),
						Release::new("James LaBrie", "Beautiful Shade of Gray"),
						Release::new("Malevolence", "Malicious Intent"),
						Release::new("Ratos de Porão", "Necropolítica"),
						Release::new("Sadist", "Firescorched"),
						Release::new("Septicflesh", "Modern Primitive"),
						Release::new("Spheric Universe Experience", "Back Home"),
						Release::new("Zinny Zan", "Lullabies for the Masses"),
					]),
					(25, vec![
						Release::new("Man with a Mission", "Break and Cross the Walls II"),
					]),
					(27, vec![
						Release::new("Baest", "Justitia (EP)"),
						Release::new("Brutality", "Sempiternity"),
						Release::new("Cadaveria", "Emptiness"),
						Release::new("Crematory", "Inglorious Darkness"),
						Release::new("Decapitated", "Cancer Culture"),
						Release::new("Def Leppard", "Diamond Star Halos"),
						Release::new("Holocausto Canibal", "Crueza Ferina"),
						Release::new("Lord Belial", "Rapture"),
						Release::new("Michael Schenker Group", "Universal"),
						Release::new("Mournful Congregation", "The Exuviae of Gods – Part I (EP)"),
						Release::new("Odd Crew", "Dark Matters (Part 1)"),
						Release::new("Trollfest", "Flamingo Overlord"),
					]),
					(31, vec![
						Release::new("Ribspreader", "Crypt World"),
					]),
				])),
				(Month::June, Releases::from([
					(3, vec![
						Release::new("The Algorithm", "Data Renaissance"),
						Release::new("Astronoid", "Radiant Bloom"),
						Release::new("Battlelore", "The Return of the Shadow"),
						Release::new("Bleed from Within", "Shrine"),
						Release::new("Gwar", "The New Dark Ages"),
						Release::new("Killswitch Engage", "Live at the Palladium (live album)"),
						Release::new("Las Cruces", "Cosmic Tears"),
						Release::new("Memphis May Fire", "Remade in Misery"),
						Release::new("Origin", "Chaosmos"),
						Release::new("Red Handed Denial", "I'd Rather Be Asleep"),
						Release::new("Thornhill", "Heroine"),
					]),
					(5, vec![
						Release::new("Wolfsbane", "Genius"),
					]),
					(10, vec![
						Release::new("Billy Howerdel", "What Normal Was"),
						Release::new("Deadguy", "Buyer's Remorse: Live from the Decibel Magazine Metal & Beer Fest (live album)"),
						Release::new("downset.", "Maintain"),
						Release::new("Dragged Under", "Upright Animals"),
						Release::new("Kiss", "Off the Soundboard: Live at Donington 1996 (live album)"),
						Release::new("Kreator", "Hate Über Alles"),
						Release::new("Michael Monroe", "I Live Too Fast to Die Young"),
						Release::new("Motionless in White", "Scoring the End of the World"),
						Release::new("Satyricon", "Satyricon & Munch"),
						Release::new("Schandmaul", "Knüppel aus dem Sack"),
						Release::new("Secrets", "The Collapse"),
						Release::new("Seventh Wonder", "The Testament"),
						Release::new("Severe Torture", "Fisting the Sockets (EP)"),
						Release::new("Soreption", "Jord"),
						Release::new("Tierra Santa", "Destino"),
						Release::new("William DuVall", "11.12.21 Live-In-Studio Nashville"),
						Release::new("Wind Rose", "Warfront"),
					]),
					(13, vec![
						Release::new("Tombs", "Ex Oblivion (EP)"),
					]),
					(15, vec![
						Release::new("Dir En Grey", "Phalaris"),
						Release::new("Rings of Saturn", "Rings of Saturn"),
					]),
					(17, vec![
						Release::new("Civil War", "Invaders"),
						Release::new("Infanteria", "Patriarch"),
						Release::new("Jorn", "Over the Horizon Radar"),
						Release::new("Oni", "Loathing Light"),
						Release::new("Seven Kingdoms", "Zenith"),
						Release::new("Tungsten", "Bliss"),
					]),
					(22, vec![
						Release::new("Manowar", "The Revenge of Odysseus (Highlights) (EP)"),
						Release::new("Spiritbox", "Rotoscope (EP)"),
					]),
					(24, vec![
						Release::new("Alestorm", "Seventh Rum of a Seventh Rum"),
						Release::new("Betraying the Martyrs", "Silver Lining (EP)"),
						Release::new("Between the Buried and Me", "The Great Misdirect Live (live album)"),
						Release::new("Black River", "Generation aXe"),
						Release::new("Black Stone Cherry", "Live from the Royal Albert Hall... Y'All (live album)"),
						Release::new("Coheed and Cambria", "Vaxis – Act II: A Window of the Waking Mind"),
						Release::new("Darkane", "Inhuman Spirits"),
						Release::new("Dawn of Destiny", "Of Silence"),
						Release::new("Enphin", "End Cut"),
						Release::new("Khold", "Svartsyn"),
						Release::new("Paganizer", "Beyond the Macabre"),
						Release::new("Porcupine Tree", "Closure/Continuation"),
						Release::new("Projected", "Hypoxia"),
						Release::new("Victorius", "Dinosaur Warfare Pt. 2 – The Great Ninja War"),
					]),
					(30, vec![
						Release::new("Bleeding Through",  "Rage (EP)"),
					]),
				])),
				(Month::July, Releases::from([
					(1, vec![
						Release::new("Derek Sherinian", "Vortex[377]"),
						Release::new("Greg Puciato", "Mirrorcell[378]"),
						Release::new("Haunt", "Windows of Your Heart[379]"),
						Release::new("Holy Dragons", "Jörmungandr – The Serpent of the World[380]"),
						Release::new("Massacre", "Mythos (EP)[381]"),
						Release::new("Municipal Waste", "Electrified Brain[382]"),
						Release::new("Randy Holden", "Population III[383]"),
						Release::new("Saint Asonia", "Introvert (EP)[384]"),
						Release::new("Shinedown", "Planet Zero[385]"),
						Release::new("Superheist", "MMXX[386]"),
					]),
					(6, vec![
						Release::new("Coldrain", "Nonnegative"),
					]),
					(8, vec![
						Release::new("Altaria", "Wisdom"),
						Release::new("Blind Channel", "Lifestyles of the Sick & Dangerous"),
						Release::new("Powerwolf", "The Monumental Mass – A Cinematic Metal Event (live album)"),
						Release::new("Wormrot", "Hiss"),
					]),
					(13, vec![
						Release::new("Obituary", "Cause of Death – Live Infection (live album)"),
						Release::new("Obituary", "Slowly We Rot – Live & Rotting (live album)"),
					]),
					(15, vec![
						Release::new("Antigama", "Whiteout"),
						Release::new("Jack Starr's Burning Starr", "Souls of the Innocent"),
						Release::new("Mantar", "Pain Is Forever and This Is the End"),
						Release::new("Senses Fail", "Hell Is in Your Head"),
						Release::new("Sinner", "Brotherhood"),
					]),
					(22, vec![
						Release::new("Hatriot", "The Vale of Shadows"),
						Release::new("Imperial Triumphant", "Spirit of Ecstasy"),
						Release::new("Karl Sanders", "Saurian Apocalypse"),
						Release::new("Oceans of Slumber", "Starlight and Ash"),
						Release::new("Palisades", "Reaching Hypercritical"),
						Release::new("Scar for Life", "Sociophobia"),
						Release::new("Witchery", "Nightside"),
					]),
					(28, vec![
						Release::new("Bad Wolves", "Sacred Kiss (EP)"),
						Release::new("Incantation", "Tricennial of Blasphemy (compilation album)"),
					]),
					(29, vec![
						Release::new("Belphegor", "The Devils"),
						Release::new("Black Magnet", "Body Prophecy"),
						Release::new("Chat Pile", "God's Country"),
						Release::new("Krisiun", "Mortem Solis"),
						Release::new("Stick to Your Guns", "Spectre"),
						Release::new("Torture Killer", "Dead Inside (EP)"),
					]),
				])),
				(Month::August, Releases::from([
					(4, vec![
						Release::new("Tom Hunting", "Hunting Party (EP)"),
					]),
					(5, vec![
						Release::new("Abaddon Incarnate", "The Wretched Sermon"),
						Release::new("Amon Amarth", "The Great Heathen Army"),
						Release::new("Dub War", "Westgate Under Fire"),
						Release::new("Einherjer", "Norse and Dangerous (Live... from the Land of Legends) (live album)"),
						Release::new("H.E.A.T", "Force Majeure"),
						Release::new("Psycroptic", "Divine Council"),
						Release::new("Soulfly", "Totem"),
						Release::new("Toxik", "Dis Morta"),
						Release::new("Vanden Plas", "Live & Immortal (live album)"),
					]),
					(12, vec![
						Release::new("Arch Enemy", "Deceivers"),
						Release::new("Boris", "Heavy Rocks"),
						Release::new("The Halo Effect", "Days of the Lost"),
						Release::new("Hollywood Undead", "Hotel Kalifornia"),
						Release::new("Jackyl", "30 Coming in Hot (compilation album)"),
						Release::new("Locrian", "New Catastrophism"),
						Release::new("Locrian", "Ghost Frontiers (EP)"),
						Release::new("Norma Jean", "Deathrattle Sing for Me"),
						Release::new("Wolfbrigade", "Anti-Tank Dogs (EP)"),
					]),
					(14, vec![
						Release::new("Melvins", "Bad Mood Rising"),
					]),
					(19, vec![
						Release::new("Conan", "Evidence of Immortality"),
						Release::new("Five Finger Death Punch", "AfterLife"),
						Release::new("Heilung", "Drif"),
						Release::new("I Prevail", "True Power"),
						Release::new("Lillian Axe", "From Womb to Tomb"),
						Release::new("Parasite Inc.", "Cyan Night Dreams"),
						Release::new("Psyclon Nine", "Less to Heaven"),
						Release::new("Russian Circles", "Gnosis"),
						Release::new("Soilwork", "Övergivenheten"),
						Release::new("Spirit Adrift", "20 Centuries Gone (compilation album)"),
					]),
					(26, vec![
						Release::new("Becoming the Archetype", "Children of the Great Extinction"),
						Release::new("Brymir", "Voices in the Sky"),
						Release::new("Dynazty", "Final Advent"),
						Release::new("Grave Digger", "Symbol of Eternity"),
						Release::new("Lacrimas Profundere", "How to Shroud Yourself with Night"),
						Release::new("Long Distance Calling", "Eraser"),
						Release::new("Machine Head", "Of Kingdom and Crown"),
						Release::new("Santa Cruz", "The Return of the Kings"),
						Release::new("Sigh", "Shiki"),
						Release::new("Soil", "Play It Forward (covers album)"),
						Release::new("Tad Morose", "March of the Obsequious"),
					]),
					(27, vec![
						Release::new("Imperial Age", "New World"),
					]),
				])),
				(Month::September, Releases::from([
					(1, vec![
						Release::new("Oceans Ate Alaska", "Disparity"),
					]),
					(2, vec![
						Release::new("Blind Guardian", "The God Machine"),
						Release::new("The Callous Daoboys", "Celebrity Therapist"),
						Release::new("The Hu", "Rumble of Thunder"),
						Release::new("Mad Max", "Wings of Time"),
						Release::new("Mantic Ritual", "Heart Set Stone (EP)"),
						Release::new("King's X", "Three Sides of One"),
						Release::new("Megadeth", "The Sick, the Dying... and the Dead!"),
						Release::new("Mike Tramp", "For Første Gang"),
						Release::new("Miss May I", "Curse of Existence"),
						Release::new("Novelists", "Déjà Vu"),
					]),
					(9, vec![
						Release::new("Allen/Olzon", "Army of Dreamers"),
						Release::new("Bloodbath", "Survival of the Sickest"),
						Release::new("Fallujah", "Empyrean"),
						Release::new("Holy Fawn", "Dimensional Bleed"),
						Release::new("Kiss", "Off the Soundboard: Live in Des Moines 1977 (live album)"),
						Release::new("KMFDM", "Hyëna"),
						Release::new("Mezarkabul", "Makina Elektrika"),
						Release::new("Ozzy Osbourne", "Patient Number 9"),
						Release::new("Parkway Drive", "Darker Still"),
						Release::new("Revocation", "Netherheaven"),
						Release::new("Stray from the Path", "Euthanasia"),
						Release::new("Trauma", "Awakening"),
						Release::new("Ville Laihiala & Saattajat", "Ei Meillä Ole Kuin Loisemme"),
					]),
					(16, vec![
						Release::new("The 69 Eyes", "Drive (EP)"),
						Release::new("Behemoth", "Opvs Contra Natvram"),
						Release::new("Clutch", "Sunrise on Slaughter Beach"),
						Release::new("Destrage", "SO MUCH. too much."),
						Release::new("The Devil Wears Prada", "Color Decay"),
						Release::new("Edenbridge", "Shangri-La"),
						Release::new("Electric Callboy", "Tekkno"),
						Release::new("Epoch of Unlight", "At War with the Multiverse"),
						Release::new("Hartmann", "Get Over It"),
						Release::new("Hetroertzen", "Phosphorus, Vol. 1"),
						Release::new("House of Lords", "Saints and Sinners"),
						Release::new("Marco Mendoza", "New Direction"),
						Release::new("Omega Diatribe", "My Sphere (EP)"),
						Release::new("Spiritus Mortis", "The Great Seal"),
					]),
					(23, vec![
						Release::new("KEN mode", "Null"),
						Release::new("Moonspell", "From Down Below – Live 80 Meters Deep (live album)"),
						Release::new("OvO", "Ignoto"),
						Release::new("Razor", "Cycle of Contempt"),
						Release::new("Silent Knight", "Full Force"),
						Release::new("Stratovarius", "Survive"),
						Release::new("Venom Inc.", "There's Only Black"),
					]),
					(30, vec![
						Release::new("Autopsy", "Morbidity Triumphant"),
						Release::new("Drowning Pool", "Strike a Nerve"),
						Release::new("Rage", "Spreading the Plague (EP)"),
						Release::new("Raven", "Leave 'Em Bleeding (compilation album)"),
						Release::new("Sammy Hagar and the Circle", "Crazy Times"),
						Release::new("Sceptic", "Nailed to Ignorance"),
						Release::new("Slipknot", "The End, So Far"),
						Release::new("Sonata Arctica", "Acoustic Adventures – Volume Two"),
						Release::new("Tankard", "Pavlov's Dawgs"),
					]),
				])),
				(Month::October, Releases::from([
					(1, vec![
						Release::new("Acid Witch", "Rot Among Us"),
					]),
					(5, vec![
						Release::new("Liturgy", "As the Blood of God Bursts the Veins of Time (EP)"),
					]),
					(7, vec![
						Release::new("Blind Illusion", "Wrath of the Gods"),
						Release::new("Borealis", "Illusions"),
						Release::new("Charlotte Wessels", "Tales from Six Feet Under, Vol. II"),
						Release::new("Counterparts", "A Eulogy for Those Still Here"),
						Release::new("The Cult", "Under the Midnight Sun"),
						Release::new("Ellefson–Soto", "Vacation in the Underworld"),
						Release::new("Goatwhore", "Angels Hung from the Arches of Heaven"),
						Release::new("King Gizzard & the Lizard Wizard", "Ice, Death, Planets, Lungs, Mushrooms and Lava"),
						Release::new("Lamb of God", "Omens"),
						Release::new("Lost Society", "If the Sky Came Down"),
						Release::new("Queensrÿche", "Digital Noise Alliance"),
						Release::new("Wednesday 13", "Horrifier"),
					]),
					(12, vec![
						Release::new("King Gizzard & the Lizard Wizard", "Laminated Denim"),
					]),
					(14, vec![
						Release::new("Alter Bridge", "Pawns & Kings"),
						Release::new("Bloody Hammers", "Washed in the Blood"),
						Release::new("Dragonland", "The Power of the Nightstar"),
						Release::new("Eleine", "Acoustic in Hell (EP)"),
						Release::new("Gun", "The Calton Songs"),
						Release::new("Lorna Shore", "Pain Remains"),
						Release::new("Nothing More", "Spirits"),
						Release::new("Outline in Color", "Coast Is Clear"),
						Release::new("Skid Row", "The Gang's All Here"),
						Release::new("Sleeping with Sirens", "Complete Collapse"),
						Release::new("Varials", "Scars for You to Remember"),
						Release::new("We Came as Romans", "Darkbloom"),
					]),
					(21, vec![
						Release::new("Architects", "The Classic Symptoms of a Broken Spirit"),
						Release::new("Avantasia", "A Paranormal Evening with the Moonflower Society"),
						Release::new("Avatarium", "Death, Where Is Your Sting"),
						Release::new("Black Veil Brides", "The Mourning (EP)"),
						Release::new("Brutus", "Unison Life"),
						Release::new("Exhumed", "To the Dead"),
						Release::new("Gothminister", "Pandemonium"),
						Release::new("In This Moment", "Blood 1983 (EP)"),
						Release::new("Sahg", "Born Demon"),
						Release::new("Serj Tankian", "Perplex Cities (EP)"),
						Release::new("Stryper", "The Final Battle"),
						Release::new("Ugly Kid Joe", "Rad Wings of Destiny"),
						Release::new("WarCry", "Daimon"),
						Release::new("White Skull", "Metal Never Rusts"),
					]),
					(24, vec![
						Release::new("Galahad", "The Last Great Adventurer"),
					]),
					(26, vec![
						Release::new("Fear, and Loathing in Las Vegas", "Cocoon for the Golden Future"),
					]),
					(28, vec![
						Release::new("Brant Bjork", "Bougainvillea Suite"),
						Release::new("Darkthrone", "Astral Fortress"),
						Release::new("Dead Cross", "II"),
						Release::new("Defleshed", "Grind Over Matter"),
						Release::new("Demon Hunter", "Exile"),
						Release::new("Despised Icon", "Déterré (EP)"),
						Release::new("Dr. Acula", "Dr. Acula"),
						Release::new("Fear Factory", "Recoded (remix album)"),
						Release::new("Fire from the Gods", "Soul Revolution"),
						Release::new("Fit for a King", "The Hell We Create"),
						Release::new("Joe Lynn Turner", "Belly of the Beast"),
						Release::new("King Gizzard & the Lizard Wizard", "Changes"),
						Release::new("Royal Hunt", "Dystopia – Part II"),
						Release::new("Sodom", "40 Years at War – The Greatest Hell of Sodom (compilation album)"),
						Release::new("Therion", "Leviathan II"),
					]),
				])),
				(Month::November, Releases::from([
					(4, vec![
						Release::new("96 Bitter Beings", "Synergy Restored"),
						Release::new("Black Anvil", "Regenesis"),
						Release::new("Dayseeker", "Dark Sun"),
						Release::new("Depresszió", "Vissza a Földre"),
						Release::new("Devin Townsend", "Lightwork"),
						Release::new("Disillusion", "Ayam"),
						Release::new("Frank Bello", "Then I'm Gone (EP)"),
						Release::new("Ingested", "Ashes Lie Still"),
						Release::new("Voivod", "Ultraman (EP)"),
					]),
					(8, vec![
						Release::new("Vinnie Moore", "Double Exposure"),
					]),
					(11, vec![
						Release::new("Arallu", "Death Covenant"),
						Release::new("Chelsea Grin", "Suffer in Hell"),
						Release::new("Drudkh", "Всі належать ночі"),
						Release::new("Enuff Z'Nuff", "Finer Than Sin"),
						Release::new("Epica", "The Alchemy Project (EP)"),
						Release::new("He Is Legend", "Endless Hallway"),
						Release::new("Kampfar", "Til Klovers Takt"),
						Release::new("Last in Line", "A Day in the Life (EP)"),
						Release::new("Leatherwolf", "Kill the Hunted"),
						Release::new("Ring of Fire", "Gravity"),
						Release::new("Xentrix", "Seven Words"),
					]),
					(18, vec![
						Release::new("16", "Into Dust"),
						Release::new("Aurora Borealis", "Prophecy Is the Mold in Which History Is Poured"),
						Release::new("Autograph", "Beyond"),
						Release::new("Candlemass", "Sweet Evil Sun"),
						Release::new("Disturbed", "Divisive"),
						Release::new("Nickelback", "Get Rollin'"),
						Release::new("Ronnie Atkins", "Symphomaniac (EP)"),
						Release::new("Saint Asonia", "Extrovert (EP)"),
						Release::new("Soen", "Atlantis (live album)"),
						Release::new("Tallah", "The Generation of Danger"),
						Release::new("Threshold", "Dividing Lines"),
						Release::new("U.D.O.", "The Legacy (compilation album)"),
						Release::new("Wolves at the Gate", "Lowborn (EP)"),
					]),
					(25, vec![
						Release::new("Elder", "Innate Passage"),
						Release::new("Hibernus Mortis", "The Monoliths of Cursed Slumber"),
						Release::new("In the Woods...", "Diversum"),
						Release::new("Judicator", "The Majesty of Decay"),
						Release::new("The Last Ten Seconds of Life", "Disquisition on an Execution (EP)"),
						Release::new("Leather", "We Are the Chosen"),
						Release::new("Lee Aaron", "Elevate"),
						Release::new("Ofermod", "Ofermodian Litanies (mini-album)"),
						Release::new("Sword", "III"),
					]),
					(28, vec![
						Release::new("Necrodeath", "Singin' in the Pain"),
					]),
				])),
				(Month::December, Releases::from([
					(2, vec![
						Release::new("Amberian Dawn", "Take a Chance – A Metal Tribute to ABBA (covers album)"),
						Release::new("Deströyer 666", "Never Surrender"),
						Release::new("Eisregen", "Wiedergänger (EP)"),
						Release::new("Hammers of Misfortune", "Overtaker"),
					]),
					(9, vec![
						Release::new("Lionheart", "Welcome to the West Coast III"),
						Release::new("Ripper", "Return to Death Row (EP)"),
						Release::new("Serenity", "Memoria (live album)"),
					]),
					(14, vec![
						Release::new("Nemophila", "Seize the Fate"),
					]),
					(15, vec![
						Release::new("Rotting Christ", "The Apocryphal Spells, Vol. I (EP)"),
						Release::new("Rotting Christ", "The Apocryphal Spells, Vol. II (EP)"),
					]),
					(22, vec![
						Release::new("Rudra", "Eight Mahavidyas"),
					]),
					(25, vec![
						Release::new("Snowy Shaw", "This Is Heavy Metal, Plain & Simple (compilation album)"),
					]),
					(30, vec![
						Release::new("Lord of the Lost", "Blood & Glitter"),
						Release::new("Satanic Warmaster", "Aamongandr"),
					]),
				])),
            ])
        };
        pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_2023_calendar_ok() -> Result<()> {
        let client = MockClient {};

        let got = client.scrape(2023).await?;

        let want = Calendar {
			data: CalendarData::from([
				(Month::January, Releases::from([
					(13, vec![
						Release::new("Ahab", "The Coral Tombs"),
						Release::new("Beyond the Black", "Beyond the Black"),
						Release::new("Eisregen", "Grenzgänger"),
						Release::new("Obituary", "Dying of Everything"),
						Release::new("Turmion Kätilöt", "Omen X"),
						Release::new("VV", "Neon Noir"),
					]),
					(20, vec![
						Release::new("Atrocity", "Okkult III"),
						Release::new("Dark Princess", "Phoenix"),
						Release::new("Imperium Dekadenz", "Into Sorrow Evermore"),
						Release::new("Katatonia", " Sky Void of Stars"),
						Release::new("Riverside", " ID.Entity"),
						Release::new("Sabaton", "Heroes of the Great War (EP)"),
						Release::new("Twilight Force", "At the Heart of Wintervale"),
					]),
					(27, vec![
						Release::new("...And Oceans", "As in Gardens, So in Tombs"),
						Release::new("Ablaze My Sorrow", "The Loss of All Hope (EP)"),
						Release::new("Ronnie Romero", "Raised on Heavy Radio (covers album)"),
						Release::new("Steve Vai", "Vai/Gash"),
						Release::new("Uriah Heep", " Chaos & Colour"),
					]),
				])),
				(Month::February, Releases::from([
					(3, vec![
						Release::new("All Out War", "Celestial Rot"),
						Release::new("Korn", "Requiem Mass (EP)"),
						Release::new("Memoriam", "Rise to Power"),
						Release::new("Russkaja", "Turbo Polka Party"),
						Release::new("Victor Smolski", "Guitar Force"),
						Release::new("Xandria", "The Wonders Still Awaiting"),
					]),
					(10, vec![
						Release::new("Aphyxion", "Ad Astra"),
						Release::new("Delain", "Dark Waters"),
						Release::new("Dream Theater", "Distance Over Time Demos (2018) (compilation album)"),
						Release::new("In Flames", "Foregone"),
						Release::new("Narrow Head", "Moments of Clarity"),
						Release::new("Pierce the Veil", "The Jaws of Life"),
						Release::new("Wig Wam", "Out of the Dark"),
					]),
					(17, vec![
						Release::new("Avatar", "Dance Devil Dance"),
						Release::new("Bridear", "Aegis of London... Live! (live album)"),
						Release::new("Clint Lowery", "Ghostwriter (EP)"),
						Release::new("Code Orange", "What Is Really Underneath? (remix album)"),
						Release::new("Hellripper", "Warlocks Grim & Withered Hags"),
						Release::new("Jason Bieler", "Postcards from the Asylum"),
						Release::new("Man Must Die", "The Pain Behind It All"),
						Release::new("Oceanhoarse", "Heads Will Roll"),
						Release::new("Pigs Pigs Pigs Pigs Pigs Pigs Pigs", "Land of Sleeper"),
						Release::new("Robin McAuley", "Alive"),
						Release::new("See You Next Tuesday", "Distractions"),
						Release::new("Skinflint", "Hate Spell"),
					]),
					(22, vec![
						Release::new("Lovebites", "Judgement Day"),
					]),
					(24, vec![
						Release::new("Azaghal", "Alttarimme on Luista Tehty"),
						Release::new("Dope", "Blood Money, Part Zer0"),
						Release::new("Godsmack", " Lighting Up the Sky"),
						Release::new("Hammerhedd", "Nonetheless"),
						Release::new("Hed PE", "70's Hits from the Pit (covers album)"),
						Release::new("Heidevolk", "Werdekeer"),
						Release::new("Hypno5e", "Sheol"),
						Release::new("Insomnium", "Anno 1696"),
						Release::new("Kauan", "ATM Revised"),
						Release::new("Märvel", "Double Decade (compilation album)"),
						Release::new("Necrovation", "Storm the Void/Starving Grave (EP)"),
						Release::new("Steel Panther", "On the Prowl"),
						Release::new("Venomous Concept", "The Good Ship Lollipop"),
					]),
				])),
				(Month::March, Releases::from([
					(1, vec![
						Release::new("Galneryus", "Between Dread and Valor"),
						Release::new("Lynch", "Reborn"),
					]),
					(3, vec![
						Release::new("Earth Groans", "Tongue Tied (EP)"),
						Release::new("Enslaved", " Heimdal"),
						Release::new("Entheos", "Time Will Take Us All"),
						Release::new("Full of Hell & Primitive Man", "Suffocating Hallucination"),
						Release::new("Fury of Five", "Half Past Revenge (EP)"),
						Release::new("Haken", "Fauna"),
						Release::new("Sandrider", "Enveletration"),
						Release::new("Sortilège", "Apocalypso"),
					]),
					(9, vec![
						Release::new("Wes Borland", "Mutiny on the Starbarge"),
					]),
					(10, vec![
						Release::new("For the Fallen Dreams", "For the Fallen Dreams"),
						Release::new("Frozen Crown", "Call of the North"),
						Release::new("Gorod", "The Orb"),
						Release::new("Ice Age", "Waves of Loss and Power"),
						Release::new("Isole", "Anesidora"),
						Release::new("Minenwerfer", "Feuerwalze"),
						Release::new("Nanowar of Steel", "Dislike to False Metal"),
						Release::new("Otherwise", "Gawdzillionaire"),
						Release::new("Periphery", "Periphery V: Djent Is Not a Genre"),
						Release::new("Sacrificium", "Oblivion"),
						Release::new("Story of the Year", "Tear Me to Pieces"),
						Release::new("Suicide Silence", "Remember... You Must Die"),
						Release::new("Tulus", "Fandens Kall"),
					]),
					(17, vec![
						Release::new("Aftermath", "No Time to Waste"),
						Release::new("Babylon A.D.", "Live Lightning (live album)"),
						Release::new("Chelsea Grin", "Suffer in Heaven"),
						Release::new("Contrarian", "Sage of Shekhinah"),
						Release::new("Dark Sanctuary", "Cernunnos"),
						Release::new("Downfall of Gaia", "Silhouettes of Disgust"),
						Release::new("Elysion", "Bring Out Your Dead"),
						Release::new("Gideon", "More Power. More Pain."),
						Release::new("Invent Animate", "Heavener"),
						Release::new("John Diva and the Rockets of Love", "The Big Easy"),
						Release::new("Kamelot", "The Awakening"),
						Release::new("Mystic Circle", "Erzdämon"),
						Release::new("Narnia", "Ghost Town"),
						Release::new("Night Demon", "Outsider"),
						Release::new("Pop Evil", "Skeletons"),
						Release::new("Redemption", "I Am the Storm"),
						Release::new("Seven Spires", "Live at ProgPower USA XXI (live album)"),
						Release::new("Theory of a Deadman", "Dinosaur"),
					]),
					(24, vec![
						Release::new("Acid King", "Beyond Vision"),
						Release::new("August Burns Red", "Death Below"),
						Release::new("Babymetal", "The Other One"),
						Release::new("Cruachan", "The Living and the Dead"),
						Release::new("Dawn Ray'd", "To Know the Light"),
						Release::new("Excalion", "Once Upon a Time"),
						Release::new("Floor Jansen", "Paragon"),
						Release::new("Hatesphere", "Hatred Reborn"),
						Release::new("Ihsahn", "Fascination Street Sessions (EP)"),
						Release::new("Keep of Kalessin", "Katharsis"),
						Release::new("Liturgy", "93696"),
						Release::new("Ne Obliviscaris", "Exul"),
						Release::new("Project 86", "Omni, Part 1"),
						Release::new("Saxon", "More Inspirations (covers album)"),
						Release::new("Subway to Sally", "Himmelfahrt"),
						Release::new("Xysma", "No Place Like Alone"),
					]),
					(29, vec![
						Release::new("After the Burial", "Embrace the Infinity (EP)"),
					]),
					(31, vec![
						Release::new("Ad Infinitum", "Chapter III – Downfall"),
						Release::new("Alpha Wolf", "Shh (EP)"),
						Release::new("Attack Attack!", "Dark Waves (EP)"),
						Release::new("Bury Tomorrow", "The Seventh Sun"),
						Release::new("De La Tierra", "III"),
						Release::new("Last in Line", "Jericho"),
						Release::new("Lordi", "Screem Writers Guild"),
						Release::new("The Ongoing Concept", "Again"),
						Release::new("Rotten Sound", "Apocalypse"),
						Release::new("Visions of Atlantis", "Pirates Over Wacken (live album)"),
					]),
				])),
				(Month::April, Releases::from([
					(7, vec![
						Release::new("Angel Vivaldi", "Away with Words, Pt. 2 (EP)"),
						Release::new("Cultura Tres", "Camino de Brujos"),
						Release::new("Kiss", "Off the Soundboard: Poughkeepsie, New York, 1984 (live album)"),
						Release::new("Paul Gilbert", "The Dio Album (covers album)"),
						Release::new("Powerwolf", "Interludium"),
						Release::new("Rise of the Northstar", "Showdown"),
						Release::new("Tribulation", "Hamartia (EP)"),
					]),
					(14, vec![
						Release::new("Atreyu", "The Hope of a Spark (EP)"),
						Release::new("Dødheimsgard", "Black Medium Current"),
						Release::new("Holy Moses", "Invisible Queen"),
						Release::new("Infected Rain", "The Devil's Dozen (live album)"),
						Release::new("Jesus Piece", "...So Unknown"),
						Release::new("L.A. Guns", "Black Diamonds"),
						Release::new("Magnus Karlsson's Free Fall", "Hunt the Flame"),
						Release::new("Metallica", "72 Seasons"),
						Release::new("Mike Tramp", "Songs of White Lion (covers album)"),
						Release::new("Overkill", "Scorched"),
					]),
					(20, vec![
						Release::new("Portrayal of Guilt", "Devil Music"),
					]),
					(21, vec![
						Release::new("The 69 Eyes", "Death of Darkness"),
						Release::new("Angus McSix", "Angus McSix and the Sword of Power"),
						Release::new("Anthem", "Crimson & Jet Black"),
						Release::new("As Everything Unfolds", "Ultraviolet"),
						Release::new("Axel Rudi Pell", "The Ballads VI (compilation album)"),
						Release::new("Bell Witch", "Future's Shadow Part 1: The Clandestine Gate"),
						Release::new("Dorthia Cottrell", "Death Folk Country"),
						Release::new("Enter Shikari", "A Kiss for the Whole World"),
						Release::new("Liv Kristine", "River of Diamonds"),
						Release::new("Magnus Rosén Band", "It's Time to Rock the World Again"),
						Release::new("Texas Hippie Coalition", "The Name Lives On"),
					]),
					(28, vec![
						Release::new("Cradle of Filth", "Trouble and Their Double Lives (live album)"),
						Release::new("Crown the Empire", "Dogma"),
						Release::new("Disciple", "Skeleton Psalms"),
						Release::new("Elvenking", "Reader of the Runes – Rapture"),
						Release::new("Graveworm", "Killing Innocence"),
						Release::new("Iced Earth", "Hellrider/I Walk Among You (EP)"),
						Release::new("IGNEA", "Dreams of Lands Unseen"),
						Release::new("Majesty", "Back to Attack"),
						Release::new("Necronomicon", "Constant to Death"),
						Release::new("Runemagick", "Beyond the Cenotaph of Mankind"),
						Release::new("Smoulder", "Violent Creed of Vengeance"),
					]),
				])),
				(Month::May, Releases::from([
					(5, vec![
						Release::new("Blood Ceremony", "The Old Ways Remain"),
						Release::new("Burning Witches", "The Dark Tower"),
						Release::new("Currents", "The Death We Seek"),
						Release::new("Dave Lombardo", "Rites of Percussion"),
						Release::new("Deathstars", "Everything Destroys You"),
						Release::new("Enforcer", "Nostalgia"),
						Release::new("Haunt", "Golden Arm"),
						Release::new("Krallice", "Porous Resonance Abyss"),
						Release::new("Lumsk", "Fremmede Toner"),
						Release::new("Savage Grace", "Sign of the Cross"),
						Release::new("Scarlet Aura", "Rock in Sange si Vointa"),
						Release::new("Therapy?", "Hard Cold Fire"),
						Release::new("Tygers of Pan Tang", "Bloodlines"),
						Release::new("Unearth", "The Wretched; the Ruinous"),
						Release::new("Vintersea", "Woven into Ashes"),
						Release::new("Winger", "Seven"),
					]),
					(12, vec![
						Release::new("The Acacia Strain", "Step into the Light / Failure Will Follow"),
						Release::new("The Amity Affliction", "Not Without My Ghosts"),
						Release::new("Cattle Decapitation", "Terrasite"),
						Release::new("DevilDriver", "Dealing with Demons Vol. II"),
						Release::new("Heavens Edge", "Get It Right"),
						Release::new("Veil of Maya", "Mother"),
					]),
					(19, vec![
						Release::new("Alcatrazz", "Take No Prisoners"),
						Release::new("Arjen Lucassen's Supersonic Revolution", "Golden Age of Music"),
						Release::new("Botanist", "VIII: Selenotrope"),
						Release::new("Def Leppard with the Royal Philharmonic Orchestra", "Drastic Symphonies"),
						Release::new("Ghost", "Phantomime (EP)"),
						Release::new("Heretoir", "Wastelands (EP)"),
						Release::new("Mystic Prophecy", "Hellriot"),
						Release::new("The Ocean", "Holocene"),
						Release::new("Sleep Token", "Take Me Back to Eden"),
						Release::new("Sweet & Lynch", "Heart & Sacrifice"),
						Release::new("Thulcandra", "Hail the Abyss"),
						Release::new("Yakuza", "Sutra"),
					]),
					(26, vec![
						Release::new("Elegant Weapons", "Horns for a Halo"),
						Release::new("Godsticks", "This Is What a Winner Looks Like"),
						Release::new("Immortal", "War Against All"),
						Release::new("Kalmah", "Kalmah"),
						Release::new("Legion of the Damned", "The Poison Chalice"),
						Release::new("Magnus Rosén Band", "Outside the Rock Box (mini-album)"),
						Release::new("Metal Church", "Congregation of Annihilation"),
						Release::new("Sirenia", "1977"),
						Release::new("Tesla", "Full Throttle Live (live album)"),
						Release::new("Trespass", "Wolf at the Door"),
						Release::new("Vomitory", "All Heads Are Gonna Roll"),
					]),
				])),
				(Month::June, Releases::from([
					(2, vec![
						Release::new("Anubis Gate", "Interference"),
						Release::new("Avenged Sevenfold", "Life Is But a Dream..."),
						Release::new("Bongzilla", "Dab City"),
						Release::new("Buckcherry", "Vol. 10"),
						Release::new("Gloryhammer", "Return to the Kingdom of Fife"),
						Release::new("Omnium Gatherum", "Slasher (EP)"),
						Release::new("Wytch Hazel", "IV"),
					]),
					(9, vec![
						Release::new("Extreme", "Six"),
						Release::new("Glass Casket", "Glass Casket (EP)"),
						Release::new("Godflesh", "Purge"),
						Release::new("Ray Alder", "II"),
						Release::new("Rise to Fall", "The Fifth Dimension"),
						Release::new("Scar Symmetry", "The Singularity (Phase II – Xenotaph)"),
						Release::new("Shakra", "Invincible"),
						Release::new("Slipknot", "Adderall (EP)"),
					]),
					(16, vec![
						Release::new("Arkona", "Kob'"),
						Release::new("Church of Misery", "Born Under a Mad Sign"),
						Release::new("Fifth Angel", "When Angels Kill"),
						Release::new("Finger Eleven", "Greatest Hits (compilation album)"),
						Release::new("Joel Hoekstra's 13", "Crash of Life"),
						Release::new("King Gizzard & the Lizard Wizard", "PetroDragonic Apocalypse; or, Dawn of Eternal Night: An Annihilation of Planet Earth and the Beginning of Merciless Damnation"),
						Release::new("Queens of the Stone Age", "In Times New Roman..."),
						Release::new("Saturnus", "The Storm Within"),
						Release::new("Thy Catafalque", "Alföld"),
					]),
					(23, vec![
						Release::new("Jag Panzer", "The Hallowed"),
						Release::new("Nocturnal Breed", "Carry the Beast"),
						Release::new("Pyramaze", "Bloodlines"),
						Release::new("Tsjuder", "Helvegr"),
						Release::new("Xasthur", "Inevitably Dark"),
					]),
					(30, vec![
						Release::new("Before the Dawn", "Stormbringers"),
						Release::new("Death Ray Vision", "No Mercy from Electric Eyes"),
						Release::new("Divide and Dissolve", "Systemic"),
						Release::new("Raven", "All Hell's Breaking Loose"),
						Release::new("Slaughter to Prevail", "Live in Moscow (live album)"),
						Release::new("Virgin Steele", "The Passion of Dionysus"),
					]),
				])),
				(Month::July, Releases::from([
					(7, vec![
						Release::new("1476", "In Exile"),
						Release::new("Bangalore Choir", "Center Mass"),
						Release::new("Better Lovers", "God Made Me an Animal (EP)"),
						Release::new("Blackbraid", "Blackbraid II"),
						Release::new("Blaze Bayley", "Damaged Strange Different and Live (live album)"),
						Release::new("Bloodbound", "Tales from the North"),
						Release::new("Butcher Babies", "Eye for an Eye... / ...'Til the World's Blind"),
						Release::new("Fen", "Monuments to Absence"),
						Release::new("Nita Strauss", "The Call of the Void"),
						Release::new("The Raven Age", "Blood Omen"),
						Release::new("Will Haven", "VII"),
					]),
					(14, vec![
						Release::new("Edge of Paradise", "Hologram"),
						Release::new("Eleine", "We Shall Remain"),
						Release::new("Evile", "The Unknown"),
						Release::new("Fallstar", "Sacred Mirrors"),
						Release::new("Freedom Call", "The M.E.T.A.L. Fest (live album)"),
						Release::new("Kim Dracula", "A Gradual Decline in Morale"),
						Release::new("Vendetta", "Black as Coal"),
						Release::new("Voyager", "Fearless in Love"),
					]),
					(21, vec![
						Release::new("Akercocke", "Decades of Devil Worship (live album)"),
						Release::new("Cadaver", "The Age of the Offended"),
						Release::new("Soil", "Restoration (compilation album)"),
						Release::new("Voivod", "Morgöth Tales"),
						Release::new("The Zenith Passage", "Datalysium"),
					]),
					(28, vec![
						Release::new("Contrarian", "Demos & Oddities: 1995–1999 (compilation album)"),
						Release::new("From Ashes to New", "Blackout"),
						Release::new("Girlschool", "WTFortyfive?"),
						Release::new("Mutoid Man", "Mutants"),
						Release::new("Panzerchrist", "Last of a Kind"),
						Release::new("Sevendust", "Truth Killer"),
						Release::new("Signs of the Swarm", "Amongst the Low & Empty"),
						Release::new("Uncle Acid & the Deadbeats", "Slaughter on First Avenue (live album)"),
					]),
				])),
				(Month::August, Releases::from([
					(4, vec![
						Release::new("Crypta", "Shades of Sorrow"),
						Release::new("Skindred", "Smile"),
					]),
					(11, vec![
						Release::new("George Lynch & Jeff Pilson", "Heavy Hitters II (covers album)"),
						Release::new("Kataklysm", "Goliath"),
						Release::new("King Kobra", "We Are Warriors"),
						Release::new("Megaherz", "In Teufels Namen"),
						Release::new("Tarja", "Rocking Heels: Live at Metal Church (live album)"),
					]),
					(18, vec![
						Release::new("Atreyu", "The Moment You Find Your Flame (EP)"),
						Release::new("Cyhra", "The Vertigo Trigger"),
						Release::new("Horrendous", "Ontological Mysterium"),
						Release::new("Nocte Obducta", "Karwoche (Die Sonne Der Toten Pulsiert)"),
						Release::new("Orbit Culture", "Descent"),
						Release::new("Ringworm", "Seeing Through Fire"),
						Release::new("Skálmöld", "Ýdalir"),
						Release::new("Slipknot", "Live at MSG (live album)"),
						Release::new("Spirit Adrift", "Ghost at the Gallows"),
						Release::new("Warmen", "Here for None"),
					]),
					(22, vec![
						Release::new("Dethklok", "Dethalbum IV"),
					]),
					(25, vec![
						Release::new("Alice Cooper", "Road"),
						Release::new("The Armed", "Perfect Saviors"),
						Release::new("Asking Alexandria", "Where Do We Go from Here?"),
						Release::new("Blut Aus Nord", "Disharmonium – Nahab"),
						Release::new("Dethklok", "Metalocalypse: Army of the Doomstar (soundtrack album)"),
						Release::new("Endstille", "Detonation"),
						Release::new("Exmortus", "Necrophony"),
						Release::new("Filter", "The Algorithm"),
						Release::new("Hurricane", "Reconnected"),
						Release::new("Incantation", "Unholy Deification"),
						Release::new("Lions at the Gate", "The Excuses We Cannot Make"),
						Release::new("U.D.O.", "Touchdown"),
						Release::new("The Word Alive", "Hard Reset"),
					]),
					(28, vec![
						Release::new("Exhumed", "Beyond the Dead (EP)"),
					]),
				])),
				(Month::September, Releases::from([
					(1, vec![
						Release::new("Escape the Fate", "Out of the Shadows"),
						Release::new("Marduk", "Memento Mori"),
						Release::new("Phil Campbell and the Bastard Sons", "Kings of the Asylum"),
						Release::new("Polaris", "Fatalism"),
						Release::new("Primal Fear", "Code Red"),
						Release::new("Soen", "Memorial"),
						Release::new("Stitched Up Heart", "To the Wolves"),
						Release::new("Taake", "Et Hav Av Avstand"),
					]),
					(8, vec![
						Release::new("Bio-Cancer", "Revengeance"),
						Release::new("Conquer Divide", "Slow Burn"),
						Release::new("Cryptopsy", "As Gomorrah Burns"),
						Release::new("Dying Fetus", "Make Them Beg for Death"),
						Release::new("Finsterforst", "Jenseits (EP)"),
						Release::new("George Lynch", "Guitars at the End of the World"),
						Release::new("Kvelertak", "Endling"),
						Release::new("Oomph!", "Richter und Henker"),
						Release::new("Puddle of Mudd", "Ubiquitous"),
						Release::new("Saliva", "Revelation"),
						Release::new("Sylosis", "A Sign of Things to Come"),
					]),
					(15, vec![
						Release::new("Baroness", "Stone"),
						Release::new("Brujeria", "Esto Es Brujeria"),
						Release::new("Corey Taylor", "CMF2"),
						Release::new("Electric Boys", "Grand Explosivos"),
						Release::new("Gridlink", "Coronet Juniper"),
						Release::new("Mayhem", "Daemonic Rites (live album)"),
						Release::new("Molybaron", "Something Ominous"),
						Release::new("Night Verses", "Every Sound Has a Color in the Valley of Night: Part 1"),
						Release::new("Otep", "The God Slayer"),
						Release::new("Ronnie Romero", "Too Many Lies, Too Many Masters"),
						Release::new("Shade Empire", "Sunholy"),
						Release::new("Shining", "Shining"),
						Release::new("Tesseract", "War of Being"),
						Release::new("War of Ages", "Dominion"),
					]),
					(20, vec![
						Release::new("Kill Devil Hill", "Seas of Oblivion"),
					]),
					(22, vec![
						Release::new("3Teeth", "EndEx"),
						Release::new("Annisokay", "Abyss Pt I (EP)"),
						Release::new("Cannibal Corpse", "Chaos Horrific"),
						Release::new("Dayshell", "Pegasus"),
						Release::new("KEN mode", "Void"),
						Release::new("Mercenary", "Soundtrack to the End of Times"),
						Release::new("Profanatica", "Crux Simplex"),
						Release::new("Rebaelliun", "Under the Sign of Rebellion"),
						Release::new("Staind", "Confessions of the Fallen"),
						Release::new("Thy Art Is Murder", "Godlike"),
						Release::new("Wolves at the Gate", "Lost in Translation (covers album)"),
					]),
					(29, vec![
						Release::new("Black Stone Cherry", "Screamin' at the Sky"),
						Release::new("Blackbriar", "A Dark Euphony"),
						Release::new("Code Orange", "The Above"),
						Release::new("Dark the Suns", "Raven and the Nightsky"),
						Release::new("Harm's Way", "Common Suffering"),
						Release::new("KK's Priest", "The Sinner Rides Again"),
						Release::new("Nervosa", "Jailbreak"),
						Release::new("Nikki Stringfield", "Apocrypha"),
						Release::new("Primordial", "How It Ends"),
						Release::new("Red", "Rated R"),
						Release::new("Taproot", "SC\\SSRS"),
						Release::new("Wolves in the Throne Room", "Crypt of Ancestral Knowledge (EP)"),
					]),
				])),
				(Month::October, Releases::from([
					(6, vec![
						Release::new("Carnifex", "Necromanteum"),
						Release::new("Heavy Load", "Riders of the Ancient Storm"),
						Release::new("Heretoir", "Nightsphere"),
						Release::new("Iron Savior", "Firestar"),
						Release::new("October Tide", "The Cancer Pledge"),
						Release::new("Of Mice & Men", "Tether"),
						Release::new("Prong", "State of Emergency"),
						Release::new("Svalbard", "The Weight of the Mask"),
					]),
					(13, vec![
						Release::new("Amorphis", "Queen of Time (Live at Tavastia 2021) (live album)"),
						Release::new("Beartooth", "The Surface"),
						Release::new("Krieg", "Ruiner"),
						Release::new("Laster", "Andermans Mijne"),
						Release::new("On Thorns I Lay", "On Thorns I Lay"),
						Release::new("Oni", "The Silver Line"),
						Release::new("Ronnie Atkins", "Trinity"),
						Release::new("Sulphur Aeon", "Seven Crowns and Seven Seals"),
						Release::new("Sven Gali", "Bombs and Battlescars"),
						Release::new("Theocracy", "Mosaic"),
						Release::new("Varg", "Ewige Wacht"),
					]),
					(20, vec![
						Release::new("The Amenta", "Plague of Locus (EP)"),
						Release::new("Angelus Apatrida", "Aftermath"),
						Release::new("The Callous Daoboys", "God Smiles Upon the Callous Daoboys (EP)"),
						Release::new("Cirith Ungol", "Dark Parade"),
						Release::new("Dog Eat Dog", "Free Radicals"),
						Release::new("Lynch Mob", "Babylon"),
						Release::new("Myrkur", "Spine"),
						Release::new("Night Ranger", "40 Years and a Night with the Contemporary Youth Orchestra (live album)"),
						Release::new("Temperance", "Hermitage – Daruma's Eyes Pt. 2"),
						Release::new("Within Temptation", "Bleed Out"),
					]),
					(27, vec![
						Release::new("Autopsy", "Ashes, Organs, Blood and Crypts"),
						Release::new("Dokken", "Heaven Comes Down"),
						Release::new("Doro", "Conqueress Forever Strong and Proud"),
						Release::new("End", "The Sin of Human Frailty"),
						Release::new("Icarus Witch", "No Devil Lived On"),
						Release::new("In This Moment", "Godmode"),
						Release::new("King Gizzard & the Lizard Wizard", "The Silver Cord"),
						Release::new("Mark Tremonti", "Christmas Classics New & Old (covers album)"),
						Release::new("Obscura", "A Celebration I – Live in North America (live album)"),
						Release::new("Of Virtue", "Omen"),
						Release::new("Pigs Pigs Pigs Pigs Pigs Pigs Pigs", "Live in New York (live album)"),
						Release::new("Poppy", "Zig"),
						Release::new("Sorcerer", "Reign of the Reaper"),
					]),
				])),
				(Month::November, Releases::from([
					(3, vec![
						Release::new("Angra", "Cycles of Pain"),
						Release::new("Atreyu", "A Torch in the Dark (EP)"),
						Release::new("Bad Wolves", "Die About It"),
						Release::new("Dying Wish", "Symptoms of Survival"),
						Release::new("Green Lung", "This Heathen Land"),
						Release::new("Insomnium", "Songs of the Dusk (EP)"),
						Release::new("Kontrust", "Madworld"),
						Release::new("Mortuary Drape", "Black Mirror"),
						Release::new("Serenity", "Nemesis AD"),
						Release::new("Silent Planet", "Superbloom"),
						Release::new("Spiritbox", "The Fear of Fear (EP)"),
						Release::new("Suffocation", "Hymns from the Apocrypha"),
						Release::new("Watain", "Die in Fire – Live in Hell (Agony and Ecstasy Over Stockholm) (live album)"),
					]),
					(8, vec![
						Release::new("Nemophila", "The Initial Impulse (EP)"),
					]),
					(10, vec![
						Release::new("Diviner", "Avaton"),
						Release::new("Gama Bomb", "Bats"),
						Release::new("Helmet", "Left"),
						Release::new("Hinayana", "Shatter and Fall"),
						Release::new("Secret Sphere", "Blackened Heartbeat"),
						Release::new("Sodom", "1982 (EP)"),
						Release::new("Tarja", "Dark Christmas (cover album)"),
						Release::new("Vastum", "Inward to Gethsemane"),
					]),
					(11, vec![
						Release::new("Mike Mangini", "Invisible Signs"),
					]),
					(17, vec![
						Release::new("Aeternus", "Philosopher"),
						Release::new("Celeste", "Epilogue(s) (EP)"),
						Release::new("Corroded", "Plague"),
						Release::new("DGM", "Life"),
						Release::new("Eldritch", "Innervoid"),
						Release::new("Lacey Sturm", "Kenotic Metanoia"),
						Release::new("Nonpoint", "Heartless (EP)"),
						Release::new("Racetraitor", "Creation and the Timeless Order of Things"),
						Release::new("Sadus", "The Shadow Inside"),
						Release::new("Texas in July", "Without Reason (EP)"),
					]),
					(20, vec![
						Release::new("Impending Doom", "Last Days (EP)"),
					]),
					(24, vec![
						Release::new("Bernie Marsden", "Working Man"),
						Release::new("Cruciamentum", "Obsidian Refractions"),
					]),
				])),
				(Month::December, Releases::from([
					(1, vec![
						Release::new("Demoncy", "Black Star Gnosis"),
						Release::new("Extortionist", "Devoid (EP)"),
						Release::new("Ghost", "13 Commandments (compilation album)"),
						Release::new("Omega Diatribe", "Deviant"),
						Release::new("Paradise Lost", "Icon 30"),
						Release::new("Visions of Atlantis", "A Pirate's Symphony"),
					]),
					(4, vec![
						Release::new("Xibalba", "Aztlán (EP)"),
					]),
					(8, vec![
						Release::new("Atreyu", "The Beautiful Dark of Life"),
						Release::new("Dimmu Borgir", "Inspiratio Profanus (covers album)"),
						Release::new("Ektomorf", "Vivid Black"),
						Release::new("Judicator", "I Am the Void (EP)"),
						Release::new("Polkadot Cadaver", "Echoes Across the Hellscape"),
						Release::new("Porcupine Tree", "Closure/Continuation.Live (live album)"),
						Release::new("Trick or Treat", "A Creepy Night Live (live album)"),
					]),
					(15, vec![
						Release::new("Children of Bodom", "A Chapter Called Children of Bodom (The Final Show in Helsinki Ice Hall 2019) (live album)"),
						Release::new("Evergrey", "From Dark Discoveries to Heartless Portraits (compilation album)"),
						Release::new("Hed PE", "Detox"),
						Release::new("Rob Arnold", "Menace"),
						Release::new("Therion", "Leviathan III"),
						Release::new("Troll", "Trolldom"),
					]),
					(26, vec![
						Release::new("Die Apokalyptischen Reiter", "Die Mutter des Teufels (EP)")
					]),
					(29, vec![
						Release::new("Lord of the Lost", "Weapons of Mass Seduction (covers album)"),
					]),
				])),
			]),
		};
        _compare_calendars(got, want);
        //pretty_assertions::assert_eq!(got, want);
        Ok(())
    }

    #[tokio::test]
    async fn test_2024_calendar_ok() -> Result<()> {
        Ok(())
    }

    #[tokio::test]
    async fn test_2025_calendar_ok() -> Result<()> {
        Ok(())
    }

    fn _compare_calendars(got: Calendar, want: Calendar) {
        for (month, releases) in want.data.iter() {
            match got.data.get(month) {
                Some(got_releases) => {
                    for (day, want_day) in releases.iter() {
                        let got_day = match got_releases.get(day) {
                            Some(day) => day,
                            None => panic!("Missing day {:?} {day}", month),
                        };
                        pretty_assertions::assert_eq!(
                            got_day,
                            want_day,
                            "month: {:?} - day: {}",
                            month,
                            day
                        );
                    }
                }
                None => panic!("should have had month `{:?}`", month),
            }
        }
    }
}
