//! # Game State Module
//!
//! ## Overview
//! This module manages all game state, rules, and logic for Press Your Luck.
//! It implements the authentic 1983-1986 CBS game show mechanics.
//!
//! ## Game Structure
//!
//! ### Two-Round Format
//! Each round consists of:
//! 1. **Question Round**: 4 questions, players earn spins
//! 2. **Board Round**: Players use spins on the Big Board
//!
//! ### Spin Mechanics
//! - **Earned Spins**: Won through correct answers or board bonuses
//! - **Passed Spins**: Received from opponents, must be used first
//! - Players can pass ONLY earned spins (not passed spins)
//!
//! ### Passing Rules (Authentic)
//! - Pass to the leader
//! - If you're leading, pass to 2nd place
//! - If tied for lead, pass to the tied opponent
//!
//! ### Whammy Rules
//! - Landing on Whammy: Score resets to $0
//! - 4th Whammy: Player is eliminated ("Whammied out")
//! - Passed spins convert to earned spins when hit by Whammy
//!
//! ## Prize Types
//! - **Cash**: Money values ($100 - $5000)
//! - **Cash + Spin**: Money plus extra spin
//! - **Prize**: Physical prizes (trips, cars)
//! - **Special**: Add-A-One, Double Your Money, Pick a Corner, etc.
//! - **Whammy**: Lose all money and prizes

use rand::prelude::*;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// AUDIO EVENTS
// ═══════════════════════════════════════════════════════════════════════════

/// Events sent to the audio engine for sound playback
#[derive(Debug, Clone, PartialEq)]
pub enum AudioEvent {
    /// Play board tone for the given square index (0-17)
    BoardTone(usize),
    /// Play Whammy foghorn sound
    WhammySound,
    /// Play cash register sound with big flag for large amounts
    CashSound { big: bool },
    /// Play prize fanfare
    PrizeSound,
    /// Play special square sound (magical sweep)
    SpecialSound,
    /// Play correct answer chime
    CorrectSound,
    /// Play wrong answer buzzer
    WrongSound,
    /// Play sad trombone (for elimination)
    SadTrombone,
    /// Play spin added bell
    SpinAddedBell,
    /// Play buzz-in sound
    BuzzInSound,
    /// Play winner celebration
    WinnerFanfare,
    /// Play board stop mechanical "chunk" sound
    BoardStopSound,
    /// Play audience cheer for big wins
    AudienceCheer,
    /// Play audience gasp for Whammy or close calls
    AudienceGasp,
    /// Start tension music loop (spinning)
    StartTensionMusic,
    /// Stop tension music loop
    StopTensionMusic,
}

// ═══════════════════════════════════════════════════════════════════════════
// INPUT ACTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Actions that can be triggered by user input
#[derive(Debug, Clone, PartialEq)]
pub enum InputAction {
    /// Start a new game
    StartGame,
    /// Start the board spin
    StartSpin,
    /// Stop the board spin
    StopSpin,
    /// Pass earned spins to opponent
    Pass,
    /// Buzz in during question round (3 spins if correct)
    BuzzIn,
    /// Select answer during question round (0-3)
    SelectAnswer(usize),
    /// Select corner for Pick a Corner (0-3)
    SelectCorner(usize),
    /// Select choice for special squares like "$2000 or Lose Whammy"
    SelectSpecialChoice(usize),
    /// Continue to next phase/screen
    Continue,
    /// Quit the game
    Quit,
}

// ═══════════════════════════════════════════════════════════════════════════
// GAME PHASE
// ═══════════════════════════════════════════════════════════════════════════

/// Current phase of the game
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GamePhase {
    /// Initial start screen
    Start,
    /// Question round - players earning spins
    Questions,
    /// Board round - players using spins
    Board,
    /// Game over - showing winner
    GameOver,
}

// ═══════════════════════════════════════════════════════════════════════════
// PRIZE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Type of prize on a board square
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrizeType {
    /// Cash amount with optional bonus spin
    Cash { value: u32, bonus_spin: bool },
    /// Physical prize with name and value
    Prize { name: String, value: u32 },
    /// The dreaded Whammy!
    Whammy,
    /// Special action square
    Special(SpecialAction),
}

/// Special square actions (authentic 1983-1986 CBS mechanics)
///
/// Special squares were added throughout the show's run to increase excitement.
/// Most are removed after being hit once (except Big Bucks and Pick a Corner).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpecialAction {
    /// Add a "1" to the front of your score
    /// - $0 becomes $10 (authentic behavior)
    /// - $500 becomes $1,500
    /// - $1,000 becomes $11,000
    ///
    /// Premiered: Episode 508 (September 5, 1985)
    AddAOne,

    /// Move one space in the current direction
    MoveOneSpace,

    /// Go back two spaces
    BackTwo,

    /// Advance two spaces
    AdvanceTwo,

    /// Pick one of the four corners (squares 0, 5, 9, 14)
    /// Premiered: Episode 115 (February 28, 1984)
    /// Always appeared in square #6 in round two
    PickACorner,

    /// Double your money and get one free spin
    /// Originally just "Double Your Money" but changed to include spin
    /// because players with $0 would get nothing
    /// Debuted: March 8, 1984
    DoubleYourMoney,

    /// Choose: Take $2000 OR remove one Whammy
    /// Known as "Money or Lose a Whammy"
    TwoThousandOrLoseWhammy,

    /// Big Bucks - transfers to the highest amount on the board (square #4)
    /// Always in square #12, awards the dollar amount in square #4
    /// Round 1: $1,000-$1,250, Round 2: $5,000+
    BigBucks,

    /// Take the Lead + One Spin
    /// Gives you exactly enough to be in first place, plus one spin
    TakeTheLead,
}

// ═══════════════════════════════════════════════════════════════════════════
// BOARD SQUARE
// ═══════════════════════════════════════════════════════════════════════════

/// A single square on the Big Board
///
/// Each square cycles through 3 different prizes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSquare {
    /// The three prizes that cycle on this square
    pub prizes: [Prize; 3],
    /// Current displayed prize index (0-2)
    pub current_index: usize,
    /// Whether this square is currently lit
    pub is_active: bool,
}

/// Individual prize definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prize {
    /// Type of prize
    pub prize_type: PrizeType,
    /// Display text (main line)
    pub display_main: String,
    /// Display text (sub line, optional)
    pub display_sub: Option<String>,
}

impl Prize {
    /// Create a new cash prize
    pub fn cash(value: u32) -> Self {
        Self {
            prize_type: PrizeType::Cash {
                value,
                bonus_spin: false,
            },
            display_main: format!("${}", Self::format_money(value)),
            display_sub: None,
        }
    }

    /// Create a cash prize with bonus spin
    pub fn cash_with_spin(value: u32) -> Self {
        Self {
            prize_type: PrizeType::Cash {
                value,
                bonus_spin: true,
            },
            display_main: format!("${}", Self::format_money(value)),
            display_sub: Some("+SPIN".to_string()),
        }
    }

    /// Create a big bucks prize (special display)
    pub fn big_bucks(value: u32) -> Self {
        Self {
            prize_type: PrizeType::Cash {
                value,
                bonus_spin: false,
            },
            display_main: "BIG".to_string(),
            display_sub: Some("BUCKS!".to_string()),
        }
    }

    /// Create a physical prize item
    pub fn physical_prize(name: &str, value: u32, display: &str, sub: Option<&str>) -> Self {
        Self {
            prize_type: PrizeType::Prize {
                name: name.to_string(),
                value,
            },
            display_main: display.to_string(),
            display_sub: sub.map(String::from),
        }
    }

    /// Create a Whammy
    pub fn whammy() -> Self {
        Self {
            prize_type: PrizeType::Whammy,
            display_main: "WHAMMY".to_string(),
            display_sub: None,
        }
    }

    /// Create a special action prize
    pub fn special(action: SpecialAction, main: &str, sub: Option<&str>) -> Self {
        Self {
            prize_type: PrizeType::Special(action),
            display_main: main.to_string(),
            display_sub: sub.map(String::from),
        }
    }

    /// Format money with commas
    fn format_money(value: u32) -> String {
        if value >= 1000 {
            format!("{},{:03}", value / 1000, value % 1000)
        } else {
            value.to_string()
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONTESTANT
// ═══════════════════════════════════════════════════════════════════════════

/// A contestant in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contestant {
    /// Player name
    pub name: String,
    /// Current score (money won)
    pub score: u32,
    /// Spins earned through questions/board
    pub earned_spins: u32,
    /// Spins passed from other players
    pub passed_spins: u32,
    /// Number of whammies hit (4 = elimination)
    pub whammies: u32,
    /// Whether player is eliminated
    pub eliminated: bool,
    /// Player color for display
    pub color: (u8, u8, u8),
}

impl Contestant {
    /// Create a new contestant with a name and display color
    pub fn create(name: &str, color: (u8, u8, u8)) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
            earned_spins: 0,
            passed_spins: 0,
            whammies: 0,
            eliminated: false,
            color,
        }
    }

    /// Get total available spins
    pub fn total_spins(&self) -> u32 {
        self.earned_spins + self.passed_spins
    }

    /// Reset for new game
    pub fn reset(&mut self) {
        self.score = 0;
        self.earned_spins = 0;
        self.passed_spins = 0;
        self.whammies = 0;
        self.eliminated = false;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WHAMMY ANIMATION
// ═══════════════════════════════════════════════════════════════════════════

/// Whammy animation types (authentic 1983-1986 CBS animations)
///
/// Animations by Savage Steve Holland, personally selected by director Bill Carruthers.
/// 79 total animations were used during the show's run (66 regular, 13 special occasion).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub enum WhammyAnimationType {
    // ═══════════════════════════════════════════════════════════════════════
    // SET 1 (September 1983) - Original pilot animations
    // ═══════════════════════════════════════════════════════════════════════
    /// Original animation - emerges with "WHAM!" sound, pulls out hammer
    Hammer,
    /// Bouncing on a pogo stick, vanishes in smoke
    Pogo,
    /// Lights TNT fuse, says "this oughta do it," explodes
    TNT,
    /// Pushes lawn mower erasing digits from score
    LawnMower,
    /// Walking dog Fang who attacks score - "Get 'em, Fang!"
    Fang,
    /// Tap-dancing, says "Watch this!" then hooked by cane
    Dance,
    /// Hawaiian hula dancer, ends up in water offscreen
    Hula,
    /// Jaws parody - riding shark with scuba mask
    Jaws,
    /// Flying plane in nosedive, crashes offscreen
    Pilot,
    /// Bulldozer erasing score digit-by-digit
    Bulldozer,
    /// Magician with top hat, makes score disappear
    Magician,
    /// Bouncing Whammy, squeaky sounds, first designed animation
    Jumping,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 2 (October 1983)
    // ═══════════════════════════════════════════════════════════════════════
    /// Rocket "PYL83" countdown "3-2-1-Big Bucks!" then blasts off
    RocketShip,
    /// Roller skating out of control - "Look out! LOOK OOOOOUUT!!!"
    RollerSkating,
    /// Rocky-style boxer, struck by spring-loaded boxing glove
    Boxer,
    /// Constructor with jackhammer - "YES! YES! YES!"
    Jackhammer,
    /// Surfer on surfboard, taunts about money
    Surfer,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 3 (January 1984)
    // ═══════════════════════════════════════════════════════════════════════
    /// Skateboard out of control, crashes into tree
    Skateboarder,
    /// Paul Revere on Fang - "The WHAMMIES are COMING to STEAL your STASH!"
    PaulRevere,
    /// Skier shouting "Woohoo!" collides with snowman
    Skier,
    /// Dollar bill impersonating founding father - "I CANNOT tell a lie! YOU LOSE!"
    DollarBill,
    /// Magic carpet with turban - "A THOUSAND PARDONS for taking your MONEY!"
    FlyingCarpet,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 4-5 (March-June 1984)
    // ═══════════════════════════════════════════════════════════════════════
    /// UFO arrival, gets zapped
    UFO,
    /// Michael Jackson parody dancing to "Billie Jean"
    MichaelJackson,
    /// Breakdancing to boom box, boom box explodes
    Breakdancing,
    /// Picnic with Tammy Whammette, Fang eats all food
    Picnic,
    /// Boy George parody, struck by giant hammer with "OW!"
    BoyGeorge,
    /// Water skiing - "Hit it, Tammy! I'm UP!" shark swallows everything
    WaterSkiing,
    /// Dixieland band, cymbals crash into member's head
    DixielandBand,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 6 (September 1984)
    // ═══════════════════════════════════════════════════════════════════════
    /// Weightlifter succeeds then breaks through floor
    Weightlifter,
    /// Pizza guy twirling dough, lands on head
    PizzaGuy,
    /// Umpire - "You're out!"
    Umpire,
    /// Elvis impersonator - "I want money!" throws out back
    Elvis,
    /// Football player - "I'm open! Throw me the bomb!" hit by explosive
    FootballPlayer,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 7-8 (1985)
    // ═══════════════════════════════════════════════════════════════════════
    /// Whammy Supremes trio singing parody
    Supremes,
    /// Beatles parody, four characters singing
    Beatles,
    /// Ben Franklin with kite, struck by lightning - "OW! UAAAAAAAH!!!"
    Franklin,
    /// Tarzan on vine doing yell, crashes into elephant
    Tarzan,
    /// Astronaut floating - "Houston, we have a problem!"
    Astronaut,
    /// Cruise ship waving - "Hasta luego! Arrivederci! Bon Voyage!"
    CruiseShip,
    /// Cyndi Lauper parody - "I want money! Cash! Whoo! Whoo!"
    CyndiLauper,
    /// Sherlock Holmes with magnifying glass and Fang
    SherlockHolmes,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 9-10 (Late 1985)
    // ═══════════════════════════════════════════════════════════════════════
    /// Bowler declares "it's a STRIIIIIIIIIIIIKE!" then struck by larger ball
    Bowler,
    /// Photographer - "Say cheese!" flashbulb explodes
    Photographer,
    /// Big Buck joke - tired of "Big Bucks," shows actual deer
    BigBuck,
    /// Rock star with electric guitar, electrocuted
    RockStar,
    /// Human cannonball "the Great Whamini" cannon explodes
    HumanCannonball,
    /// Aerobics instructor slips into pretzel
    AerobicsInstructor,
    /// Liberace at piano, candelabra crashes on head
    Liberace,
    /// Barbershop quartet - "You're out..." in harmony
    BarbershopQuartet,

    // ═══════════════════════════════════════════════════════════════════════
    // SET 11 (June 1986) - Final regular set
    // ═══════════════════════════════════════════════════════════════════════
    /// Orchestra conductor, cannon destroys entire group
    Orchestra,
    /// Eye doctor with chart spelling "U, L, O, S, E!"
    EyeDoctor,
    /// Judge - "I sentence you to poverty!" polka-dot underwear
    Judge,
    /// Clown car with firecracker - "FIRE IN THE HOLE!!!!!"
    ClownCar,

    // ═══════════════════════════════════════════════════════════════════════
    // SPECIAL OCCASION WHAMMIES (13 total - used only during holidays)
    // ═══════════════════════════════════════════════════════════════════════
    /// Thanksgiving 1983-1985: Whammy shoots turkey with musket, explodes on himself
    /// "Sweet potatoes, cranberries and... the turkey!" "Better luck next time, pilgrim!"
    ThanksgivingTurkey,
    /// Christmas 1983-1985: Scrooge Whammy with cane - "Bah! Humbug!"
    ScroogeWhammy,
    /// Christmas 1983-1985: Whammy decorates tree, tree falls on him
    ChristmasTree,
    /// Valentine's Day: Cupid Whammy shoots arrow, misses and hits himself
    ValentineCupid,
    /// Easter: Bunny Whammy with eggs, basket falls apart
    EasterBunny,
    /// St. Patrick's Day: Leprechaun Whammy - "Top o' the mornin'! Bottom o' yer wallet!"
    Leprechaun,
    /// Fourth of July: Patriotic Whammy with sparkler, firecracker mishap
    FourthOfJuly,
    /// Halloween: Vampire Whammy - "I vant to suck your... cash!"
    HalloweenVampire,
    /// 1984 Olympics: Whammy as Olympic athlete, trips over hurdle
    OlympicsWhammy,
    /// New Year's: Baby New Year Whammy
    NewYearBaby,

    // ═══════════════════════════════════════════════════════════════════════
    // WHAMMY-OUT ANIMATIONS (3 special - only shown on 4th Whammy)
    // ═══════════════════════════════════════════════════════════════════════
    /// Grim Reaper Whammy escorts contestant out
    GrimReaper,
    /// Firing squad Whammy - "Ready! Aim! FIRE!"
    FiringSquad,
    /// Angel Whammy floats away with contestant's money to heaven
    AngelWhammy,
}

impl WhammyAnimationType {
    /// Get the authentic catchphrase for this animation
    ///
    /// These catchphrases are recreated from the original 1983-1986 CBS show.
    /// Voice of the Whammy: Bill Carruthers (show director)
    pub fn catchphrase(&self) -> &'static str {
        match self {
            // Set 1 (September 1983)
            Self::Hammer => "Hee hee hee! WHAM!",
            Self::Pogo => "Whoo! Whoo! Hee hee hee!",
            Self::TNT => "This oughta do it! ...Oh no!",
            Self::LawnMower => "Time to mow down your money!",
            Self::Fang => "Get 'em, Fang! GET 'EM!",
            Self::Dance => "Watch this! ...Get off the stage!",
            Self::Hula => "Aloha means goodbye to your money!",
            Self::Jaws => "Chomp chomp chomp!",
            Self::Pilot => "Mayday! MAYDAY!",
            Self::Bulldozer => "Move it or lose it!",
            Self::Magician => "Now you see it... now you don't!",
            Self::Jumping => "Boing! Boing! Hee hee hee!",

            // Set 2 (October 1983)
            Self::RocketShip => "3-2-1-Big Bucks! ...I thought I had the right stuff!",
            Self::RollerSkating => "Look out! LOOK OOOOOUUT!!!",
            Self::Boxer => "Float like a butterfly... OW!",
            Self::Jackhammer => "YES! YES! YES!",
            Self::Surfer => "Surf's up, money's down! Wipeout!",

            // Set 3 (January 1984)
            Self::Skateboarder => "Totally rad, dude! ...OUCH!",
            Self::PaulRevere => "The WHAMMIES are COMING to STEAL your STASH!",
            Self::Skier => "Woohoo! ...Uh oh!",
            Self::DollarBill => "I CANNOT tell a lie! YOU LOSE!",
            Self::FlyingCarpet => "A THOUSAND AND ONE PARDONS for taking your MONEY!",

            // Set 4-5 (March-June 1984)
            Self::UFO => "Take me to your wallet!",
            Self::MichaelJackson => "Hee hee! Shamone! Your money's not mine!",
            Self::Breakdancing => "Break it down! Pop! Lock! Drop your cash!",
            Self::Picnic => "Isn't this romantic? ...FANG, NO!",
            Self::BoyGeorge => "Do you really want to hurt me? OW!",
            Self::WaterSkiing => "Hit it, Tammy! I'm UP, I'm UP, I'm UUUUP!!!!",
            Self::DixielandBand => "When the Saints Go Marching... OUT!",

            // Set 6 (September 1984)
            Self::Weightlifter => "I can do it! ...Not again...",
            Self::PizzaGuy => "Extra cheese, hold the cash!",
            Self::Umpire => "You're OUT!",
            Self::Elvis => "I want money! ...my achin' back!",
            Self::FootballPlayer => "I'm open! Throw me the bomb! ...BOOM!",

            // Set 7-8 (1985)
            Self::Supremes => "Stop! In the name of Whammy!",
            Self::Beatles => "All you need is... NOTHING!",
            Self::Franklin => "I cannot tell a lie! OW! UAAAAAAAH!!!",
            Self::Tarzan => "Ah-ee-ah-ee-ah! ...THUD!",
            Self::Astronaut => "Houston, we have a problem!",
            Self::CruiseShip => "Hasta luego! Arrivederci! Bon Voyage!",
            Self::CyndiLauper => "I want money! Cash! Whoo! Whoo!",
            Self::SherlockHolmes => "Elementary, my dear Fang!",

            // Set 9-10 (Late 1985)
            Self::Bowler => "It's a STRIIIIIIIIIIIIKE! ...OW!",
            Self::Photographer => "Say cheese! ...FLASH!",
            Self::BigBuck => "I got your Big Buck right here!",
            Self::RockStar => "ROCK AND ROLL! ...bzzzzzzt!",
            Self::HumanCannonball => "Ladies and gentlemen, the Great Whamini!",
            Self::AerobicsInstructor => "I want to see sweat! ...oops!",
            Self::Liberace => "I wish my brother George was here! CRASH!",
            Self::BarbershopQuartet => "You're out... You're out... YOU'RE OUT!",

            // Set 11 (June 1986)
            Self::Orchestra => "And now, a symphony of sadness!",
            Self::EyeDoctor => "Read the chart: U-L-O-S-E!",
            Self::Judge => "I sentence you to POVERTY!",
            Self::ClownCar => "FIRE IN THE HOLE!!!!!",

            // Special Occasion Whammies
            Self::ThanksgivingTurkey => "Sweet potatoes, cranberries and... the turkey!",
            Self::ScroogeWhammy => "Bah! Humbug! Give me your money! No presents this year!",
            Self::ChristmasTree => "I wish me a Merry Christmas! ...TIMBER!",
            Self::ValentineCupid => "Be my valentine! ...OW! Wrong target!",
            Self::EasterBunny => "Hoppy Easter! ...whoops, there go your eggs!",
            Self::Leprechaun => "Top o' the mornin'! Bottom o' yer wallet!",
            Self::FourthOfJuly => "Happy Independence Day! ...KABOOM!",
            Self::HalloweenVampire => "I vant to suck your... CASH!",
            Self::OlympicsWhammy => "Going for the gold! ...TRIP!",
            Self::NewYearBaby => "Happy New Year! Same old Whammy!",

            // Whammy-Out Animations (4th Whammy)
            Self::GrimReaper => "Your time has come... say goodbye!",
            Self::FiringSquad => "Ready! Aim! FIRE! Game over!",
            Self::AngelWhammy => "Your money's going to a better place!",
        }
    }

    /// Get the duration of this animation in seconds
    pub fn duration(&self) -> f32 {
        match self {
            // Longer animations with more elaborate setups
            Self::TNT | Self::FlyingCarpet | Self::Orchestra | Self::WaterSkiing => 3.0,
            Self::RocketShip | Self::DixielandBand | Self::BarbershopQuartet => 3.0,
            Self::Supremes | Self::Beatles => 3.5,
            // Whammy-out animations are longer for dramatic effect
            Self::GrimReaper | Self::FiringSquad | Self::AngelWhammy => 4.0,
            // Holiday specials
            Self::ThanksgivingTurkey | Self::ScroogeWhammy | Self::ChristmasTree => 3.0,
            // Standard animations
            _ => 2.5,
        }
    }

    /// Check if this is a special occasion (holiday) animation
    /// Reserved for future seasonal theme support
    #[allow(dead_code)]
    pub fn is_holiday_animation(&self) -> bool {
        matches!(
            self,
            Self::ThanksgivingTurkey
                | Self::ScroogeWhammy
                | Self::ChristmasTree
                | Self::ValentineCupid
                | Self::EasterBunny
                | Self::Leprechaun
                | Self::FourthOfJuly
                | Self::HalloweenVampire
                | Self::OlympicsWhammy
                | Self::NewYearBaby
        )
    }

    /// Check if this is a Whammy-out animation (4th Whammy special)
    /// Reserved for future special elimination effects
    #[allow(dead_code)]
    pub fn is_whammy_out_animation(&self) -> bool {
        matches!(
            self,
            Self::GrimReaper | Self::FiringSquad | Self::AngelWhammy
        )
    }

    /// Pick a random Whammy-out animation (for 4th Whammy elimination)
    pub fn random_whammy_out() -> Self {
        let animations = [Self::GrimReaper, Self::FiringSquad, Self::AngelWhammy];
        *animations
            .choose(&mut rand::rng())
            .unwrap_or(&Self::GrimReaper)
    }

    /// Pick a random animation (weighted towards classic favorites)
    pub fn random() -> Self {
        let animations = [
            // Set 1 - Original classics (higher weight)
            Self::Hammer,
            Self::Hammer,
            Self::Pogo,
            Self::Pogo,
            Self::TNT,
            Self::TNT,
            Self::LawnMower,
            Self::Fang,
            Self::Fang,
            Self::Dance,
            Self::Hula,
            Self::Jaws,
            Self::Pilot,
            Self::Bulldozer,
            Self::Magician,
            Self::Jumping,
            // Set 2
            Self::RocketShip,
            Self::RollerSkating,
            Self::Boxer,
            Self::Jackhammer,
            Self::Surfer,
            // Set 3
            Self::Skateboarder,
            Self::PaulRevere,
            Self::Skier,
            Self::DollarBill,
            Self::FlyingCarpet,
            // Set 4-5
            Self::UFO,
            Self::MichaelJackson,
            Self::Breakdancing,
            Self::Breakdancing, // Fan favorite
            Self::Picnic,
            Self::BoyGeorge,
            Self::WaterSkiing,
            Self::DixielandBand,
            // Set 6
            Self::Weightlifter,
            Self::PizzaGuy,
            Self::Umpire,
            Self::Elvis,
            Self::Elvis, // Fan favorite
            Self::FootballPlayer,
            // Set 7-8
            Self::Supremes,
            Self::Beatles,
            Self::Franklin,
            Self::Tarzan,
            Self::Astronaut,
            Self::CruiseShip,
            Self::CyndiLauper,
            Self::SherlockHolmes,
            // Set 9-10
            Self::Bowler,
            Self::Photographer,
            Self::BigBuck,
            Self::RockStar,
            Self::HumanCannonball,
            Self::AerobicsInstructor,
            Self::Liberace,
            Self::BarbershopQuartet,
            // Set 11 - Final set
            Self::Orchestra,
            Self::EyeDoctor,
            Self::Judge,
            Self::ClownCar,
        ];
        *animations.choose(&mut rand::rng()).unwrap()
    }
}

/// State for Whammy animation display
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhammyAnimation {
    /// Whether animation is active
    pub active: bool,
    /// Animation type
    pub animation_type: WhammyAnimationType,
    /// Animation progress (0.0 - 1.0)
    pub progress: f32,
    /// Animation duration
    pub duration: f32,
    /// Catchphrase to display
    pub catchphrase: String,
}

impl Default for WhammyAnimation {
    fn default() -> Self {
        Self {
            active: false,
            animation_type: WhammyAnimationType::Hammer,
            progress: 0.0,
            duration: 2.5,
            catchphrase: String::new(),
        }
    }
}

impl WhammyAnimation {
    /// Start a new random Whammy animation
    pub fn start_random(&mut self) {
        let anim_type = WhammyAnimationType::random();
        self.active = true;
        self.animation_type = anim_type;
        self.progress = 0.0;
        self.duration = anim_type.duration();
        self.catchphrase = anim_type.catchphrase().to_string();
    }

    /// Start a Whammy-out animation (for 4th Whammy elimination)
    /// These are special dramatic animations only shown when a player is eliminated.
    pub fn start_whammy_out(&mut self) {
        let anim_type = WhammyAnimationType::random_whammy_out();
        self.active = true;
        self.animation_type = anim_type;
        self.progress = 0.0;
        self.duration = anim_type.duration();
        self.catchphrase = anim_type.catchphrase().to_string();
    }

    /// Update animation progress
    pub fn update(&mut self, delta: f32) {
        if self.active {
            self.progress += delta / self.duration;
            if self.progress >= 1.0 {
                self.active = false;
                self.progress = 0.0;
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRIVIA QUESTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// A trivia question
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriviaQuestion {
    /// The question text
    pub question: String,
    /// The correct answer
    pub correct_answer: String,
    /// Wrong answers
    pub wrong_answers: Vec<String>,
}

impl TriviaQuestion {
    /// Get shuffled answer choices
    pub fn get_shuffled_choices(&self) -> (Vec<String>, usize) {
        let mut choices: Vec<String> = self.wrong_answers.clone();
        choices.push(self.correct_answer.clone());

        // Shuffle and find correct index
        let mut rng = rand::rng();
        choices.shuffle(&mut rng);

        let correct_index = choices
            .iter()
            .position(|c| c == &self.correct_answer)
            .unwrap();

        (choices, correct_index)
    }
}

/// Get all trivia questions
pub fn get_trivia_questions() -> Vec<TriviaQuestion> {
    vec![
        TriviaQuestion {
            question: "What planet is known as the Red Planet?".to_string(),
            correct_answer: "Mars".to_string(),
            wrong_answers: vec![
                "Venus".to_string(),
                "Jupiter".to_string(),
                "Saturn".to_string(),
            ],
        },
        TriviaQuestion {
            question: "How many sides does a hexagon have?".to_string(),
            correct_answer: "Six".to_string(),
            wrong_answers: vec!["Five".to_string(), "Eight".to_string(), "Seven".to_string()],
        },
        TriviaQuestion {
            question: "What is the capital of France?".to_string(),
            correct_answer: "Paris".to_string(),
            wrong_answers: vec![
                "London".to_string(),
                "Rome".to_string(),
                "Berlin".to_string(),
            ],
        },
        TriviaQuestion {
            question: "In what year did World War II end?".to_string(),
            correct_answer: "1945".to_string(),
            wrong_answers: vec!["1944".to_string(), "1946".to_string(), "1943".to_string()],
        },
        TriviaQuestion {
            question: "What is the largest mammal?".to_string(),
            correct_answer: "Blue Whale".to_string(),
            wrong_answers: vec![
                "Elephant".to_string(),
                "Giraffe".to_string(),
                "Hippopotamus".to_string(),
            ],
        },
        TriviaQuestion {
            question: "Who painted the Mona Lisa?".to_string(),
            correct_answer: "Leonardo da Vinci".to_string(),
            wrong_answers: vec![
                "Michelangelo".to_string(),
                "Picasso".to_string(),
                "Van Gogh".to_string(),
            ],
        },
        TriviaQuestion {
            question: "What is the chemical symbol for gold?".to_string(),
            correct_answer: "Au".to_string(),
            wrong_answers: vec!["Ag".to_string(), "Fe".to_string(), "Go".to_string()],
        },
        TriviaQuestion {
            question: "How many continents are there?".to_string(),
            correct_answer: "Seven".to_string(),
            wrong_answers: vec!["Six".to_string(), "Five".to_string(), "Eight".to_string()],
        },
        TriviaQuestion {
            question: "What is the largest ocean?".to_string(),
            correct_answer: "Pacific".to_string(),
            wrong_answers: vec![
                "Atlantic".to_string(),
                "Indian".to_string(),
                "Arctic".to_string(),
            ],
        },
        TriviaQuestion {
            question: "Who wrote Romeo and Juliet?".to_string(),
            correct_answer: "Shakespeare".to_string(),
            wrong_answers: vec![
                "Dickens".to_string(),
                "Hemingway".to_string(),
                "Twain".to_string(),
            ],
        },
        TriviaQuestion {
            question: "What is the square root of 144?".to_string(),
            correct_answer: "12".to_string(),
            wrong_answers: vec!["14".to_string(), "11".to_string(), "13".to_string()],
        },
        TriviaQuestion {
            question: "What gas do plants absorb from the atmosphere?".to_string(),
            correct_answer: "Carbon Dioxide".to_string(),
            wrong_answers: vec![
                "Oxygen".to_string(),
                "Nitrogen".to_string(),
                "Helium".to_string(),
            ],
        },
        TriviaQuestion {
            question: "How many stripes are on the American flag?".to_string(),
            correct_answer: "13".to_string(),
            wrong_answers: vec!["50".to_string(), "15".to_string(), "12".to_string()],
        },
        TriviaQuestion {
            question: "What is the hardest natural substance?".to_string(),
            correct_answer: "Diamond".to_string(),
            wrong_answers: vec![
                "Gold".to_string(),
                "Iron".to_string(),
                "Platinum".to_string(),
            ],
        },
        TriviaQuestion {
            question: "Who invented the telephone?".to_string(),
            correct_answer: "Alexander Graham Bell".to_string(),
            wrong_answers: vec![
                "Thomas Edison".to_string(),
                "Nikola Tesla".to_string(),
                "Benjamin Franklin".to_string(),
            ],
        },
        TriviaQuestion {
            question: "What is the smallest planet in our solar system?".to_string(),
            correct_answer: "Mercury".to_string(),
            wrong_answers: vec!["Mars".to_string(), "Venus".to_string(), "Pluto".to_string()],
        },
        TriviaQuestion {
            question: "What year did the Titanic sink?".to_string(),
            correct_answer: "1912".to_string(),
            wrong_answers: vec!["1905".to_string(), "1920".to_string(), "1898".to_string()],
        },
        TriviaQuestion {
            question: "What is the largest organ in the human body?".to_string(),
            correct_answer: "Skin".to_string(),
            wrong_answers: vec![
                "Liver".to_string(),
                "Brain".to_string(),
                "Heart".to_string(),
            ],
        },
        TriviaQuestion {
            question: "In what city is the Eiffel Tower located?".to_string(),
            correct_answer: "Paris".to_string(),
            wrong_answers: vec![
                "London".to_string(),
                "Rome".to_string(),
                "Madrid".to_string(),
            ],
        },
        TriviaQuestion {
            question: "What is the boiling point of water in Fahrenheit?".to_string(),
            correct_answer: "212".to_string(),
            wrong_answers: vec!["100".to_string(), "200".to_string(), "180".to_string()],
        },
    ]
}

// ═══════════════════════════════════════════════════════════════════════════
// QUESTION STATE
// ═══════════════════════════════════════════════════════════════════════════

/// State for the question round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionState {
    /// Current question being asked
    pub current_question: Option<TriviaQuestion>,
    /// Shuffled answer choices
    pub choices: Vec<String>,
    /// Index of correct answer
    pub correct_index: usize,
    /// Time remaining for buzz-in (seconds)
    pub buzz_timer: f32,
    /// Whether waiting for someone to buzz in
    pub waiting_for_buzz: bool,
    /// Whether showing answer choices
    pub showing_choices: bool,
    /// Whether answer has been revealed
    pub answer_revealed: bool,
    /// Player who buzzed in (if any)
    pub buzzed_player: Option<usize>,
    /// Questions asked this round
    pub questions_this_round: u32,
    /// Questions used this game (indices)
    pub used_questions: Vec<usize>,
}

impl Default for QuestionState {
    fn default() -> Self {
        Self {
            current_question: None,
            choices: Vec::new(),
            correct_index: 0,
            buzz_timer: 5.0,
            waiting_for_buzz: false,
            showing_choices: false,
            answer_revealed: false,
            buzzed_player: None,
            questions_this_round: 0,
            used_questions: Vec::new(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RESULT STATE
// ═══════════════════════════════════════════════════════════════════════════

/// State for displaying spin results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultState {
    /// Whether showing result
    pub showing: bool,
    /// Main result text
    pub main_text: String,
    /// Sub result text
    pub sub_text: String,
    /// Display timer
    pub timer: f32,
}

impl Default for ResultState {
    fn default() -> Self {
        Self {
            showing: false,
            main_text: String::new(),
            sub_text: String::new(),
            timer: 0.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// GAME STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Board positions (clockwise from top-left)
///
/// Layout:
/// ```text
///   0  1  2  3  4  5    (top row)
///  17 [CENTER STAGE] 6
///  16 [   DISPLAY  ] 7
///  15 [    AREA    ] 8
///  14 13 12 11 10  9    (bottom row)
/// ```
///
/// Used by graphics module for board layout and by game logic for
/// position-based special effects (corners, Big Bucks target, etc.)
pub const BOARD_POSITIONS: [(u8, u8); 18] = [
    // Top row (left to right)
    (0, 0),
    (1, 0),
    (2, 0),
    (3, 0),
    (4, 0),
    (5, 0),
    // Right column (top to bottom)
    (5, 1),
    (5, 2),
    (5, 3),
    // Bottom row (right to left)
    (5, 4),
    (4, 4),
    (3, 4),
    (2, 4),
    (1, 4),
    (0, 4),
    // Left column (bottom to top)
    (0, 3),
    (0, 2),
    (0, 1),
];

/// Corner square indices for Pick a Corner special action
/// These are the four corner positions on the board
pub const CORNER_SQUARES: [usize; 4] = [0, 5, 9, 14];

/// Big Bucks target square (square #4 always has highest value)
pub const BIG_BUCKS_TARGET: usize = 4;

/// Big Bucks source square (square #12 where the Big Bucks special appears)
pub const BIG_BUCKS_SOURCE: usize = 12;

/// Get the grid position (column, row) for a given square index
pub fn get_square_position(square_index: usize) -> (u8, u8) {
    BOARD_POSITIONS[square_index % 18]
}

/// Check if a square is a corner square
pub fn is_corner_square(square_index: usize) -> bool {
    CORNER_SQUARES.contains(&square_index)
}

/// Main game state structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Current game phase
    pub phase: GamePhase,
    /// Current round (1 or 2)
    pub round: u8,
    /// The three contestants
    pub contestants: [Contestant; 3],
    /// Index of current player (0-2)
    pub current_player: usize,
    /// The Big Board (18 squares)
    pub board: [BoardSquare; 18],
    /// Currently lit square index (0-17)
    pub lit_square: usize,
    /// Whether the board is spinning
    pub is_spinning: bool,
    /// Light movement speed (squares per second)
    pub light_speed: f32,
    /// Accumulated time for light movement
    pub light_timer: f32,
    /// Normalized spin speed for deceleration (1.0 = full speed, 0.0 = stopped)
    pub spin_speed: f32,
    /// Whether the board is decelerating
    pub spin_decelerating: bool,
    /// Target square when decelerating (for authentic timing)
    spin_target_square: Option<usize>,
    /// Prize rotation timer
    pub rotation_timer: f32,
    /// Question state
    pub question_state: QuestionState,
    /// Result display state
    pub result_state: ResultState,
    /// Whammy animation state
    pub whammy_animation: WhammyAnimation,
    /// Current message to display
    pub message: String,
    /// Message excitement flag
    pub message_excited: bool,
    /// Awaiting corner selection (Pick a Corner)
    pub awaiting_corner_selection: bool,
    /// Awaiting special choice ($2000 or Lose Whammy)
    pub awaiting_special_choice: bool,
    /// All trivia questions
    #[serde(skip)]
    trivia_questions: Vec<TriviaQuestion>,
}

impl GameState {
    /// Create a new game state
    pub fn new() -> Self {
        let mut state = Self {
            phase: GamePhase::Start,
            round: 1,
            contestants: [
                Contestant::create("PLAYER 1", (255, 100, 100)), // Red
                Contestant::create("PLAYER 2", (100, 255, 100)), // Green
                Contestant::create("PLAYER 3", (100, 100, 255)), // Blue
            ],
            current_player: 0,
            board: Self::create_round1_board(),
            lit_square: 0,
            is_spinning: false,
            light_speed: 15.0, // Squares per second
            light_timer: 0.0,
            spin_speed: 0.0, // Normalized spin speed (0.0 = stopped, 1.0 = full)
            spin_decelerating: false,
            spin_target_square: None,
            rotation_timer: 0.0,
            question_state: QuestionState::default(),
            result_state: ResultState::default(),
            whammy_animation: WhammyAnimation::default(),
            message: "Welcome to Press Your Luck! Press SPACE to begin.".to_string(),
            message_excited: false,
            awaiting_corner_selection: false,
            awaiting_special_choice: false,
            trivia_questions: get_trivia_questions(),
        };

        // Initialize random prize indices
        let mut rng = rand::rng();
        for square in &mut state.board {
            square.current_index = rng.random_range(0..3);
        }

        state
    }

    /// Create Round 1 board prizes (authentic 1983-1986 CBS values)
    ///
    /// Round 1 prize values: $100-$1,250, with Big Bucks always in square #12
    /// transferring to square #4 (which contains the highest value: $1,000-$1,250)
    fn create_round1_board() -> [BoardSquare; 18] {
        [
            // Square 0 - Top left corner
            BoardSquare {
                prizes: [
                    Prize::cash(300),
                    Prize::whammy(),
                    Prize::cash_with_spin(500),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 1
            BoardSquare {
                prizes: [Prize::cash(400), Prize::cash(750), Prize::whammy()],
                current_index: 0,
                is_active: false,
            },
            // Square 2 - Add-A-One location (premiered Sept 1985)
            BoardSquare {
                prizes: [
                    Prize::cash(500),
                    Prize::special(SpecialAction::AddAOne, "ADD-A-ONE", Some("Special!")),
                    Prize::cash(350),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 3
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(1000),
                    Prize::whammy(),
                    Prize::cash(600),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 4 - BIG BUCKS TARGET (highest value: $1,000-$1,250)
            // This is where Big Bucks (square #12) transfers to
            // Uses Prize::big_bucks() for authentic "BIG BUCKS!" display
            BoardSquare {
                prizes: [
                    Prize::big_bucks(1250),
                    Prize::cash_with_spin(1000),
                    Prize::big_bucks(1100),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 5 - Top right corner
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(500),
                    Prize::cash(800),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 6 - Pick a Corner appears here in Round 2
            BoardSquare {
                prizes: [Prize::whammy(), Prize::cash(350), Prize::cash(1000)],
                current_index: 0,
                is_active: false,
            },
            // Square 7
            BoardSquare {
                prizes: [Prize::cash(600), Prize::whammy(), Prize::cash(450)],
                current_index: 0,
                is_active: false,
            },
            // Square 8
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(750),
                    Prize::cash(500),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 9 - Bottom right corner
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::MoveOneSpace, "MOVE 1", Some("SPACE")),
                    Prize::cash(400),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 10
            BoardSquare {
                prizes: [Prize::whammy(), Prize::cash(550), Prize::cash(900)],
                current_index: 0,
                is_active: false,
            },
            // Square 11
            BoardSquare {
                prizes: [Prize::cash(1250), Prize::whammy(), Prize::cash(650)],
                current_index: 0,
                is_active: false,
            },
            // Square 12 - BIG BUCKS (authentic position!)
            // Transfers to square #4 which has the highest value
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::BigBucks, "BIG", Some("BUCKS!")),
                    Prize::whammy(),
                    Prize::cash_with_spin(800),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 13
            BoardSquare {
                prizes: [Prize::whammy(), Prize::cash(700), Prize::cash(450)],
                current_index: 0,
                is_active: false,
            },
            // Square 14 - Bottom left corner
            BoardSquare {
                prizes: [
                    Prize::cash(1000),
                    Prize::special(SpecialAction::AddAOne, "ADD-A-ONE", Some("Special!")),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 15
            BoardSquare {
                prizes: [Prize::cash(600), Prize::whammy(), Prize::cash(850)],
                current_index: 0,
                is_active: false,
            },
            // Square 16
            BoardSquare {
                prizes: [
                    Prize::whammy(),
                    Prize::cash_with_spin(500),
                    Prize::cash(750),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 17
            BoardSquare {
                prizes: [Prize::cash(400), Prize::cash(1100), Prize::whammy()],
                current_index: 0,
                is_active: false,
            },
        ]
    }

    /// Create Round 2 board prizes (authentic 1983-1986 CBS values)
    ///
    /// Round 2 prize values: $1,500-$5,000+, with authentic special squares.
    /// Big Bucks always in square #12 transferring to square #4 ($5,000)
    /// Pick a Corner always in square #6 (premiered Feb 28, 1984)
    /// Double Your $$$ + One Spin debuted March 8, 1984
    fn create_round2_board() -> [BoardSquare; 18] {
        [
            // Square 0 - Top left corner (Pick a Corner option)
            BoardSquare {
                prizes: [Prize::whammy(), Prize::cash(1500), Prize::cash(2000)],
                current_index: 0,
                is_active: false,
            },
            // Square 1
            BoardSquare {
                prizes: [
                    Prize::cash(2500),
                    Prize::whammy(),
                    Prize::cash_with_spin(1500),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 2
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(3000),
                    Prize::cash(1750),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 3 - Add-A-One (removed from Round 1 as it appeared later in show's run)
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::AddAOne, "ADD-A-ONE", Some("Special!")),
                    Prize::whammy(),
                    Prize::cash_with_spin(2500),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 4 - BIG BUCKS TARGET (always highest value: $5,000)
            // This is where Big Bucks (square #12) transfers to
            // Uses Prize::big_bucks() for authentic "BIG BUCKS!" display
            BoardSquare {
                prizes: [
                    Prize::big_bucks(5000),
                    Prize::cash_with_spin(4500),
                    Prize::big_bucks(4000),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 5 - Top right corner (Pick a Corner option)
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(4000),
                    Prize::whammy(),
                    Prize::cash(1750),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 6 - PICK A CORNER (authentic position!)
            // Premiered Episode 115 (February 28, 1984)
            // Allows choice between corners: squares 0, 5, 9, 14
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::PickACorner, "PICK A", Some("CORNER!")),
                    Prize::cash(2250),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 7 - PRIZE: New Car (authentic 1980s car value ~$4,500)
            BoardSquare {
                prizes: [
                    Prize::physical_prize("Chevy Cavalier", 4500, "NEW CAR!", Some("CHEVY")),
                    Prize::whammy(),
                    Prize::cash_with_spin(3000),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 8
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(5000),
                    Prize::cash(2000),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 9 - Bottom right corner (Pick a Corner option)
            // DOUBLE YOUR $$$ + ONE SPIN (debuted March 8, 1984)
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::DoubleYourMoney, "DOUBLE", Some("YOUR $$$!")),
                    Prize::whammy(),
                    Prize::cash(2500),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 10
            BoardSquare {
                prizes: [
                    Prize::whammy(),
                    Prize::cash_with_spin(1500),
                    Prize::cash(3000),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 11 - PRIZE: Hawaii Trip
            BoardSquare {
                prizes: [
                    Prize::physical_prize("Trip to Hawaii", 3500, "HAWAII", Some("TRIP!")),
                    Prize::cash(2000),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 12 - BIG BUCKS (authentic position!)
            // Transfers to square #4 which has $5,000
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::BigBucks, "BIG", Some("BUCKS!")),
                    Prize::whammy(),
                    Prize::cash_with_spin(4000),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 13 - $2000 OR LOSE ONE WHAMMY
            BoardSquare {
                prizes: [
                    Prize::special(
                        SpecialAction::TwoThousandOrLoseWhammy,
                        "$2,000 OR",
                        Some("LOSE 1 W"),
                    ),
                    Prize::whammy(),
                    Prize::cash(2750),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 14 - Bottom left corner (Pick a Corner option)
            BoardSquare {
                prizes: [
                    Prize::cash_with_spin(3500),
                    Prize::whammy(),
                    Prize::cash(1500),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 15 - PRIZE: Bahamas Cruise
            BoardSquare {
                prizes: [
                    Prize::physical_prize("Bahamas Cruise", 3000, "CRUISE!", Some("BAHAMAS")),
                    Prize::whammy(),
                    Prize::cash_with_spin(2000),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 16 - Take the Lead + One Spin
            BoardSquare {
                prizes: [
                    Prize::special(SpecialAction::TakeTheLead, "TAKE THE", Some("LEAD!")),
                    Prize::cash(4500),
                    Prize::whammy(),
                ],
                current_index: 0,
                is_active: false,
            },
            // Square 17
            BoardSquare {
                prizes: [
                    Prize::cash(2000),
                    Prize::whammy(),
                    Prize::cash_with_spin(3000),
                ],
                current_index: 0,
                is_active: false,
            },
        ]
    }

    /// Handle input action and return audio events to play
    pub fn handle_input(&mut self, action: InputAction) -> Vec<AudioEvent> {
        match action {
            InputAction::StartGame => {
                self.start_game();
                Vec::new()
            }
            InputAction::StartSpin => {
                let was_spinning = self.is_spinning;
                self.start_spin();
                if self.is_spinning && !was_spinning {
                    vec![AudioEvent::StartTensionMusic]
                } else {
                    Vec::new()
                }
            }
            InputAction::StopSpin => self.stop_spin(),
            InputAction::Pass => {
                self.handle_pass();
                Vec::new()
            }
            InputAction::BuzzIn => {
                self.handle_buzz_in();
                vec![AudioEvent::BuzzInSound]
            }
            InputAction::SelectAnswer(idx) => self.handle_answer(idx),
            InputAction::SelectCorner(idx) => self.handle_corner_selection(idx),
            InputAction::SelectSpecialChoice(idx) => self.handle_special_choice(idx),
            InputAction::Continue => {
                self.handle_continue();
                Vec::new()
            }
            InputAction::Quit => Vec::new(), // Handled in main
        }
    }

    /// Update game state, returns audio events
    pub fn update(&mut self, delta_time: f32) -> Vec<AudioEvent> {
        let mut audio_events = Vec::new();

        match self.phase {
            GamePhase::Questions => {
                self.update_questions(delta_time, &mut audio_events);
            }
            GamePhase::Board => {
                self.update_board(delta_time, &mut audio_events);
            }
            _ => {}
        }

        audio_events
    }

    /// Update question round
    fn update_questions(&mut self, delta_time: f32, audio_events: &mut Vec<AudioEvent>) {
        let qs = &mut self.question_state;

        // Update buzz timer
        if qs.waiting_for_buzz && !qs.answer_revealed {
            let old_timer = qs.buzz_timer;
            qs.buzz_timer -= delta_time;

            // Play tick sounds as timer counts down (every half second in the last 3 seconds)
            if qs.buzz_timer <= 3.0 && qs.buzz_timer > 0.0 {
                let old_half_sec = (old_timer * 2.0).floor() as i32;
                let new_half_sec = (qs.buzz_timer * 2.0).floor() as i32;
                if old_half_sec != new_half_sec {
                    // Use board tone as tick sound (square 0 for low tone)
                    audio_events.push(AudioEvent::BoardTone(0));
                }
            }

            if qs.buzz_timer <= 0.0 {
                // Time's up - go to multiple choice, play wrong sound to indicate time ran out
                qs.waiting_for_buzz = false;
                qs.showing_choices = true;
                audio_events.push(AudioEvent::WrongSound);
                self.message = "Time's up! Multiple choice for 1 spin each.".to_string();
            }
        }
    }

    /// Update board round
    fn update_board(&mut self, delta_time: f32, audio_events: &mut Vec<AudioEvent>) {
        // Update prize rotation (every second)
        self.rotation_timer += delta_time;
        if self.rotation_timer >= 1.0 {
            self.rotation_timer -= 1.0;
            for square in &mut self.board {
                square.current_index = (square.current_index + 1) % 3;
            }
        }

        // Update light movement with authentic deceleration curve
        if self.is_spinning {
            // Apply deceleration if player has pressed stop
            if self.spin_decelerating {
                // Authentic exponential deceleration curve
                // The show's board slowed down over ~1.5-2 seconds with increasing intervals
                let decel_rate = 2.5; // Higher = faster deceleration
                self.spin_speed -= self.spin_speed * decel_rate * delta_time;

                // Minimum speed threshold before stopping
                const MIN_SPIN_SPEED: f32 = 0.08;

                if self.spin_speed <= MIN_SPIN_SPEED {
                    // Final stop - trigger prize processing
                    self.spin_speed = 0.0;
                    self.is_spinning = false;
                    self.spin_decelerating = false;

                    // Process the prize now that we've stopped
                    let prize_events = self.process_prize_on_stop();
                    audio_events.extend(prize_events);
                    return;
                }
            }

            // Calculate effective speed: base speed * normalized spin speed
            let effective_speed = self.light_speed * self.spin_speed;

            // Only move if we have enough speed
            if effective_speed > 0.1 {
                self.light_timer += delta_time;
                let step_time = 1.0 / effective_speed;

                if self.light_timer >= step_time {
                    self.light_timer -= step_time;

                    // Move light
                    self.board[self.lit_square].is_active = false;
                    self.lit_square = (self.lit_square + 1) % 18;
                    self.board[self.lit_square].is_active = true;

                    // Play board tone - pitch decreases as spin slows (authentic effect)
                    // Corner squares and Big Bucks source get special treatment
                    let tone_index = if self.is_current_square_corner() {
                        // Higher tone for corner squares
                        (self.lit_square + 9) % 18
                    } else if self.lit_square == BIG_BUCKS_SOURCE {
                        // Distinctive tone for Big Bucks source square
                        17
                    } else {
                        self.lit_square
                    };
                    audio_events.push(AudioEvent::BoardTone(tone_index));
                }
            }
        }

        // Update Whammy animation
        if self.whammy_animation.active {
            self.whammy_animation.update(delta_time);
            if !self.whammy_animation.active {
                self.check_turn_end();
            }
        }

        // Update result display
        if self.result_state.showing {
            self.result_state.timer -= delta_time;
            if self.result_state.timer <= 0.0 {
                self.result_state.showing = false;
                self.check_turn_end();
            }
        }
    }

    /// Start a new game
    fn start_game(&mut self) {
        // Reset all contestants
        for contestant in &mut self.contestants {
            contestant.reset();
        }

        // Reset game state
        self.phase = GamePhase::Questions;
        self.round = 1;
        self.board = Self::create_round1_board();
        self.question_state = QuestionState::default();
        self.result_state = ResultState::default();
        self.whammy_animation = WhammyAnimation::default();

        // Validate board setup - ensure Big Bucks is in authentic position
        debug_assert!(
            self.validate_big_bucks_position(),
            "Big Bucks special should be in square #{} (BIG_BUCKS_SOURCE)",
            BIG_BUCKS_SOURCE
        );

        // Start first question
        self.start_next_question();
    }

    /// Start the next trivia question
    fn start_next_question(&mut self) {
        let all_questions = &self.trivia_questions;
        let qs = &mut self.question_state;

        // Find an unused question
        let mut available: Vec<usize> = (0..all_questions.len())
            .filter(|i| !qs.used_questions.contains(i))
            .collect();

        if available.is_empty() {
            // All questions used, reset
            qs.used_questions.clear();
            available = (0..all_questions.len()).collect();
        }

        let idx = *available.choose(&mut rand::rng()).unwrap();
        qs.used_questions.push(idx);

        let question = all_questions[idx].clone();
        let (choices, correct_idx) = question.get_shuffled_choices();

        qs.current_question = Some(question);
        qs.choices = choices;
        qs.correct_index = correct_idx;
        qs.buzz_timer = 5.0;
        qs.waiting_for_buzz = true;
        qs.showing_choices = false;
        qs.answer_revealed = false;
        qs.buzzed_player = None;
        qs.questions_this_round += 1;

        self.message = "Buzz in for 3 spins, or wait for multiple choice!".to_string();
    }

    /// Handle buzz in
    fn handle_buzz_in(&mut self) {
        if self.phase != GamePhase::Questions {
            return;
        }

        let qs = &mut self.question_state;
        if !qs.waiting_for_buzz || qs.answer_revealed {
            return;
        }

        qs.waiting_for_buzz = false;
        qs.showing_choices = true;
        qs.buzzed_player = Some(self.current_player);

        self.message = format!(
            "{} buzzed in! Select your answer (1-4).",
            self.contestants[self.current_player].name
        );
    }

    /// Handle answer selection and return audio events
    fn handle_answer(&mut self, answer_idx: usize) -> Vec<AudioEvent> {
        if self.phase != GamePhase::Questions {
            return Vec::new();
        }

        let qs = &mut self.question_state;
        if !qs.showing_choices || qs.answer_revealed || answer_idx >= qs.choices.len() {
            return Vec::new();
        }

        qs.answer_revealed = true;
        let correct = answer_idx == qs.correct_index;

        let audio_event = if correct {
            AudioEvent::CorrectSound
        } else {
            AudioEvent::WrongSound
        };

        if let Some(buzzed) = qs.buzzed_player {
            // Buzz-in: 3 spins if correct
            if correct {
                self.contestants[buzzed].earned_spins += 3;
                self.message = format!("CORRECT! {} earns 3 spins!", self.contestants[buzzed].name);
            } else {
                self.message = format!(
                    "Sorry, that's wrong! The answer was: {}",
                    qs.current_question.as_ref().unwrap().correct_answer
                );
            }
        } else {
            // Multiple choice: 1 spin each if correct
            if correct {
                for contestant in &mut self.contestants {
                    if !contestant.eliminated {
                        contestant.earned_spins += 1;
                    }
                }
                self.message = "CORRECT! Everyone earns 1 spin!".to_string();
            } else {
                self.message = format!(
                    "Sorry! The answer was: {}",
                    qs.current_question.as_ref().unwrap().correct_answer
                );
            }
        }

        vec![audio_event]
    }

    /// Handle continue action
    fn handle_continue(&mut self) {
        match self.phase {
            GamePhase::Questions => {
                let qs = &self.question_state;
                if qs.answer_revealed {
                    // Move to next question or board round
                    if self.question_state.questions_this_round >= 4 {
                        self.start_board_round();
                    } else {
                        self.start_next_question();
                    }
                }
            }
            GamePhase::Board
                if !self.is_spinning
                    && !self.whammy_animation.active
                    && !self.result_state.showing =>
            {
                self.check_turn_end();
            }
            _ => {}
        }
    }

    /// Start the board round
    fn start_board_round(&mut self) {
        self.phase = GamePhase::Board;
        self.question_state.questions_this_round = 0;

        // Find player with lowest score to go first
        let mut lowest_idx = 0;
        let mut lowest_spins = u32::MAX;
        for (i, c) in self.contestants.iter().enumerate() {
            if !c.eliminated && c.total_spins() < lowest_spins {
                lowest_spins = c.total_spins();
                lowest_idx = i;
            }
        }
        self.current_player = lowest_idx;

        // Update board for round 2 if needed
        if self.round == 2 {
            self.board = Self::create_round2_board();
        }

        self.update_board_message();
    }

    /// Start spinning the board
    fn start_spin(&mut self) {
        if self.phase != GamePhase::Board || self.is_spinning {
            return;
        }

        let player = &self.contestants[self.current_player];
        if player.eliminated || player.total_spins() == 0 {
            return;
        }

        // Deduct spin (use passed first)
        if self.contestants[self.current_player].passed_spins > 0 {
            self.contestants[self.current_player].passed_spins -= 1;
        } else {
            self.contestants[self.current_player].earned_spins -= 1;
        }

        self.is_spinning = true;
        self.spin_speed = 1.0; // Full speed
        self.spin_decelerating = false;
        self.spin_target_square = None;
        self.light_speed = 15.0 + rand::rng().random_range(0.0..5.0);
        self.message = "Press SPACE to stop!".to_string();
    }

    /// Stop spinning and process result
    fn stop_spin(&mut self) -> Vec<AudioEvent> {
        if !self.is_spinning || self.spin_decelerating {
            return Vec::new();
        }

        // Start deceleration instead of immediate stop (authentic behavior)
        self.spin_decelerating = true;
        self.message = "Slowing down...".to_string();

        // No audio events yet - they play when we actually stop
        Vec::new()
    }

    /// Process prize when the board finally stops (after deceleration)
    fn process_prize_on_stop(&mut self) -> Vec<AudioEvent> {
        // Stop tension music first
        let mut events = vec![AudioEvent::StopTensionMusic];

        // Get the current square position for display purposes
        let pos = self.get_current_square_position();
        let is_corner = self.is_current_square_corner();

        // Get the prize on the current square
        let square = &self.board[self.lit_square];
        let prize = square.prizes[square.current_index].clone();

        // Log corner landing for potential bonus effects
        if is_corner {
            // Corner squares are at grid positions (0,0), (5,0), (5,4), (0,4)
            // This could be used for special corner-landing bonuses
            let _corner_pos = pos; // Available for future corner-specific features
        }

        // Add prize processing events
        events.extend(self.process_prize(prize));
        events
    }

    /// Process a prize (returns audio events to play)
    fn process_prize(&mut self, prize: Prize) -> Vec<AudioEvent> {
        let mut audio_events = Vec::new();

        // Always play the mechanical board stop sound first
        audio_events.push(AudioEvent::BoardStopSound);

        let player = &mut self.contestants[self.current_player];

        match &prize.prize_type {
            PrizeType::Cash { value, bonus_spin } => {
                player.score += value;
                if *bonus_spin {
                    player.earned_spins += 1;
                    audio_events.push(AudioEvent::SpinAddedBell);
                }

                // Big cash sound for amounts >= $1000
                audio_events.push(AudioEvent::CashSound {
                    big: *value >= 1000,
                });

                // Audience cheers for big wins ($2000+)
                if *value >= 2000 {
                    audio_events.push(AudioEvent::AudienceCheer);
                }

                self.result_state = ResultState {
                    showing: true,
                    main_text: prize.display_main.clone(),
                    sub_text: if *bonus_spin {
                        "+ 1 SPIN!".to_string()
                    } else {
                        String::new()
                    },
                    timer: 2.0,
                };

                self.message = format!(
                    "{} wins {}{}!",
                    player.name,
                    prize.display_main,
                    if *bonus_spin { " + 1 spin" } else { "" }
                );
            }

            PrizeType::Prize { name, value } => {
                player.score += value;

                audio_events.push(AudioEvent::PrizeSound);

                // Audience cheers for prizes
                audio_events.push(AudioEvent::AudienceCheer);

                self.result_state = ResultState {
                    showing: true,
                    main_text: name.clone(),
                    sub_text: format!("${}", value),
                    timer: 2.5,
                };

                self.message = format!("{} wins a {}!", player.name, name);
            }

            PrizeType::Whammy => {
                let old_score = player.score;
                player.score = 0;
                player.whammies += 1;

                // Convert passed spins to earned (they're now "stuck" with them)
                player.earned_spins += player.passed_spins;
                player.passed_spins = 0;

                // Audience gasps at Whammy
                audio_events.push(AudioEvent::AudienceGasp);

                // Play Whammy sound
                audio_events.push(AudioEvent::WhammySound);

                // Check if this is the 4th (eliminating) Whammy
                if player.whammies >= 4 {
                    // Use special Whammy-out animation for 4th Whammy
                    self.whammy_animation.start_whammy_out();
                    player.eliminated = true;
                    audio_events.push(AudioEvent::SadTrombone);
                    self.message = format!("WHAMMY! {} is OUT with 4 Whammies!", player.name);
                } else {
                    // Start regular random Whammy animation
                    self.whammy_animation.start_random();
                    self.message = format!(
                        "WHAMMY! {} loses ${} (Whammy #{})!",
                        player.name, old_score, player.whammies
                    );
                }
            }

            PrizeType::Special(action) => {
                audio_events.push(AudioEvent::SpecialSound);
                let special_events = self.process_special_action(action.clone());
                audio_events.extend(special_events);
            }
        }

        audio_events
    }

    /// Process special action (authentic 1983-1986 CBS mechanics)
    /// Returns audio events to play
    fn process_special_action(&mut self, action: SpecialAction) -> Vec<AudioEvent> {
        let mut audio_events = Vec::new();
        let player = &mut self.contestants[self.current_player];

        match action {
            SpecialAction::AddAOne => {
                // Authentic Add-A-One: Add a "1" to the front of the score
                // $0 becomes $10 (not $1 - this matches the authentic show)
                // $500 becomes $1,500
                // $1,000 becomes $11,000
                let old_score = player.score;
                let new_score = if old_score == 0 {
                    10 // Authentic: $0 becomes $10
                } else {
                    let digits = (old_score as f64).log10() as u32 + 1;
                    let multiplier = 10u32.pow(digits);
                    multiplier + old_score
                };
                player.score = new_score;

                audio_events.push(AudioEvent::CashSound {
                    big: new_score >= 10000,
                });

                self.result_state = ResultState {
                    showing: true,
                    main_text: "ADD-A-ONE!".to_string(),
                    sub_text: format!("${} -> ${}", old_score, new_score),
                    timer: 2.5,
                };

                self.message = format!(
                    "Add-A-One! {} goes from ${} to ${}!",
                    player.name, old_score, new_score
                );
            }

            SpecialAction::MoveOneSpace => {
                // Move to next square and process
                self.lit_square = (self.lit_square + 1) % 18;
                self.board[self.lit_square].is_active = true;

                let square = &self.board[self.lit_square];
                let prize = square.prizes[square.current_index].clone();

                self.message = "Move One Space!".to_string();
                let prize_events = self.process_prize(prize);
                audio_events.extend(prize_events);
            }

            SpecialAction::BackTwo => {
                // Move back two squares
                self.lit_square = (self.lit_square + 18 - 2) % 18;
                self.board[self.lit_square].is_active = true;

                let square = &self.board[self.lit_square];
                let prize = square.prizes[square.current_index].clone();

                self.message = "Back Two Spaces!".to_string();
                let prize_events = self.process_prize(prize);
                audio_events.extend(prize_events);
            }

            SpecialAction::AdvanceTwo => {
                // Move forward two squares
                self.lit_square = (self.lit_square + 2) % 18;
                self.board[self.lit_square].is_active = true;

                let square = &self.board[self.lit_square];
                let prize = square.prizes[square.current_index].clone();

                self.message = "Advance Two Spaces!".to_string();
                let prize_events = self.process_prize(prize);
                audio_events.extend(prize_events);
            }

            SpecialAction::PickACorner => {
                // Authentic Pick a Corner: Choose from corner squares
                // Use is_corner_square to validate the current square setup
                // The four corners are validated using BOARD_POSITIONS
                self.awaiting_corner_selection = true;

                // Verify all corner squares are valid using is_corner_square
                // and show their grid positions from BOARD_POSITIONS
                let corner_info: Vec<String> = CORNER_SQUARES
                    .iter()
                    .enumerate()
                    .filter(|(_, &idx)| is_corner_square(idx))
                    .map(|(num, &idx)| {
                        let pos = get_square_position(idx);
                        format!("[{}] pos({},{})", num + 1, pos.0, pos.1)
                    })
                    .collect();

                // Display message with corner count for authenticity
                self.message = format!(
                    "PICK A CORNER! {} corners available. Press 1-4.",
                    corner_info.len()
                );
            }

            SpecialAction::DoubleYourMoney => {
                // Authentic: Double Your $$$ + One Spin
                // The "+ One Spin" was added because $0 doubled = $0
                player.score *= 2;
                player.earned_spins += 1;

                audio_events.push(AudioEvent::CashSound {
                    big: player.score >= 5000,
                });
                audio_events.push(AudioEvent::SpinAddedBell);

                self.result_state = ResultState {
                    showing: true,
                    main_text: "DOUBLE YOUR $$$!".to_string(),
                    sub_text: format!("${} + 1 SPIN!", player.score),
                    timer: 2.5,
                };

                self.message = format!(
                    "Double Your Money! {} now has ${} + 1 spin!",
                    player.name, player.score
                );
            }

            SpecialAction::TwoThousandOrLoseWhammy => {
                if player.whammies > 0 {
                    self.awaiting_special_choice = true;
                    self.message = "Choose: [1] Take $2,000  |  [2] Lose one Whammy".to_string();
                } else {
                    // No whammies to lose, just take the money
                    player.score += 2000;
                    audio_events.push(AudioEvent::CashSound { big: true });
                    self.result_state = ResultState {
                        showing: true,
                        main_text: "$2,000!".to_string(),
                        sub_text: "(No Whammies to lose)".to_string(),
                        timer: 2.0,
                    };
                    self.message = format!("{} takes $2,000!", player.name);
                }
            }

            SpecialAction::BigBucks => {
                // Authentic Big Bucks: Transfer to the highest amount on board
                // Always in square #12 (BIG_BUCKS_SOURCE), awards the prize in square #4 (BIG_BUCKS_TARGET)
                // Round 1: $1,000-$1,250, Round 2: $5,000+
                let big_bucks_value = if self.round == 1 { 1250 } else { 5000 };

                // Move light to Big Bucks target square using the constant
                self.board[self.lit_square].is_active = false;
                self.lit_square = BIG_BUCKS_TARGET;
                self.board[BIG_BUCKS_TARGET].is_active = true;

                player.score += big_bucks_value;

                audio_events.push(AudioEvent::CashSound { big: true });

                self.result_state = ResultState {
                    showing: true,
                    main_text: "BIG BUCKS!".to_string(),
                    sub_text: format!("${}", big_bucks_value),
                    timer: 2.5,
                };

                self.message = format!("BIG BUCKS! {} wins ${}!", player.name, big_bucks_value);
            }

            SpecialAction::TakeTheLead => {
                // Take the Lead + One Spin: Match the leader's score + 1 spin
                let leader_score = self
                    .contestants
                    .iter()
                    .filter(|c| !c.eliminated)
                    .map(|c| c.score)
                    .max()
                    .unwrap_or(0);

                // Only useful if not already leading
                let player = &mut self.contestants[self.current_player];
                if leader_score > player.score {
                    let amount_added = leader_score - player.score;
                    player.score = leader_score;
                    player.earned_spins += 1;

                    audio_events.push(AudioEvent::CashSound {
                        big: amount_added >= 1000,
                    });
                    audio_events.push(AudioEvent::SpinAddedBell);

                    self.result_state = ResultState {
                        showing: true,
                        main_text: "TAKE THE LEAD!".to_string(),
                        sub_text: format!("+${} + 1 SPIN!", amount_added),
                        timer: 2.5,
                    };

                    self.message = format!(
                        "Take the Lead! {} is now tied at ${} + 1 spin!",
                        player.name, player.score
                    );
                } else {
                    // Already leading - just get the spin
                    player.earned_spins += 1;

                    audio_events.push(AudioEvent::SpinAddedBell);

                    self.result_state = ResultState {
                        showing: true,
                        main_text: "TAKE THE LEAD!".to_string(),
                        sub_text: "+ 1 SPIN! (Already leading!)".to_string(),
                        timer: 2.0,
                    };

                    self.message = format!(
                        "Take the Lead! {} is already winning, + 1 spin!",
                        player.name
                    );
                }
            }
        }

        audio_events
    }

    /// Handle corner selection and return audio events
    fn handle_corner_selection(&mut self, corner: usize) -> Vec<AudioEvent> {
        if !self.awaiting_corner_selection || corner > 3 {
            return Vec::new();
        }

        self.awaiting_corner_selection = false;

        // Use the CORNER_SQUARES constant for corner indices
        self.lit_square = CORNER_SQUARES[corner];
        self.board[self.lit_square].is_active = true;

        let square = &self.board[self.lit_square];
        let prize = square.prizes[square.current_index].clone();

        self.process_prize(prize)
    }

    /// Handle special choice ($2000 or Lose Whammy) and return audio events
    fn handle_special_choice(&mut self, choice: usize) -> Vec<AudioEvent> {
        if !self.awaiting_special_choice {
            return Vec::new();
        }

        self.awaiting_special_choice = false;
        let player = &mut self.contestants[self.current_player];

        if choice == 0 {
            // Take $2,000
            player.score += 2000;
            self.result_state = ResultState {
                showing: true,
                main_text: "$2,000!".to_string(),
                sub_text: String::new(),
                timer: 2.0,
            };
            self.message = format!("{} takes $2,000!", player.name);
            vec![AudioEvent::CashSound { big: true }]
        } else {
            // Lose one Whammy
            player.whammies = player.whammies.saturating_sub(1);
            self.result_state = ResultState {
                showing: true,
                main_text: "WHAMMY GONE!".to_string(),
                sub_text: format!("{} Whammies left", player.whammies),
                timer: 2.0,
            };
            self.message = format!("{} loses a Whammy!", player.name);
            vec![AudioEvent::CorrectSound] // Happy chime for removing a whammy
        }
    }

    /// Handle passing spins
    fn handle_pass(&mut self) {
        if self.phase != GamePhase::Board || self.is_spinning {
            return;
        }

        let player = &self.contestants[self.current_player];
        if player.earned_spins == 0 {
            self.message = "No earned spins to pass!".to_string();
            return;
        }

        // Find pass target
        if let Some(target) = self.find_pass_target() {
            let spins_to_pass = self.contestants[self.current_player].earned_spins;

            self.contestants[target].passed_spins += spins_to_pass;
            self.contestants[self.current_player].earned_spins = 0;

            let player_name = self.contestants[self.current_player].name.clone();
            let target_name = self.contestants[target].name.clone();

            self.message = format!(
                "{} passes {} spin{} to {}!",
                player_name,
                spins_to_pass,
                if spins_to_pass != 1 { "s" } else { "" },
                target_name
            );

            // If current player has no more spins, switch to target
            if self.contestants[self.current_player].total_spins() == 0 {
                self.current_player = target;
            }
        }
    }

    /// Find the player to pass spins to
    ///
    /// Rules:
    /// - Pass to the leader
    /// - If you're leading, pass to 2nd place
    fn find_pass_target(&self) -> Option<usize> {
        let current = &self.contestants[self.current_player];

        // Find scores of other non-eliminated players
        let mut others: Vec<(usize, u32)> = self
            .contestants
            .iter()
            .enumerate()
            .filter(|(i, c)| *i != self.current_player && !c.eliminated)
            .map(|(i, c)| (i, c.score))
            .collect();

        if others.is_empty() {
            return None;
        }

        // Sort by score descending
        others.sort_by_key(|a| std::cmp::Reverse(a.1));

        // If current player is leading or tied for lead, pass to 2nd place
        if current.score >= others[0].1 && others.len() > 1 {
            Some(others[1].0)
        } else {
            Some(others[0].0)
        }
    }

    /// Check if turn should end and find next player
    fn check_turn_end(&mut self) {
        let player = &self.contestants[self.current_player];

        // If current player has spins and isn't eliminated, continue their turn
        if !player.eliminated && player.total_spins() > 0 {
            self.update_board_message();
            return;
        }

        // Find next player with spins
        for i in 1..=3 {
            let next_idx = (self.current_player + i) % 3;
            let next = &self.contestants[next_idx];
            if !next.eliminated && next.total_spins() > 0 {
                self.current_player = next_idx;
                self.update_board_message();
                return;
            }
        }

        // No one has spins - end round
        self.end_board_round();
    }

    /// Update the message for current board state
    fn update_board_message(&mut self) {
        let player = &self.contestants[self.current_player];
        let total = player.total_spins();
        self.message = format!(
            "{}: {} spin{} ({}E/{}P). Press SPACE to spin or P to pass.",
            player.name,
            total,
            if total != 1 { "s" } else { "" },
            player.earned_spins,
            player.passed_spins
        );
    }

    /// End the current board round
    fn end_board_round(&mut self) {
        if self.round == 1 {
            // Start Round 2
            self.round = 2;
            self.phase = GamePhase::Questions;
            self.question_state = QuestionState::default();
            self.message = "End of Round 1! Round 2 has BIGGER PRIZES!".to_string();

            // Start round 2 questions
            self.start_next_question();
        } else {
            // End game
            self.end_game();
        }
    }

    /// End the game and determine winner, returns audio events
    fn end_game(&mut self) -> Vec<AudioEvent> {
        self.phase = GamePhase::GameOver;

        // Find winner (highest score among non-eliminated)
        let winner = self
            .contestants
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.eliminated)
            .max_by_key(|(_, c)| c.score);

        if let Some((_idx, contestant)) = winner {
            self.message = format!("{} WINS with ${}!", contestant.name, contestant.score);
            self.message_excited = true;

            self.result_state = ResultState {
                showing: true,
                main_text: "WINNER!".to_string(),
                sub_text: format!("{}: ${}", contestant.name, contestant.score),
                timer: 10.0,
            };

            vec![AudioEvent::WinnerFanfare]
        } else {
            self.message = "GAME OVER - Everyone whammied out!".to_string();
            self.result_state = ResultState {
                showing: true,
                main_text: "GAME OVER".to_string(),
                sub_text: "No winner this time!".to_string(),
                timer: 10.0,
            };

            vec![AudioEvent::SadTrombone]
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    /// Get the grid position of the currently lit square
    /// Returns (column, row) tuple using the BOARD_POSITIONS layout
    pub fn get_current_square_position(&self) -> (u8, u8) {
        get_square_position(self.lit_square)
    }

    /// Check if the currently lit square is a corner
    /// Useful for UI highlighting and special effects
    pub fn is_current_square_corner(&self) -> bool {
        is_corner_square(self.lit_square)
    }

    /// Validate that the Big Bucks special is in its authentic position
    /// Big Bucks should always be in square #12 (BIG_BUCKS_SOURCE)
    pub fn validate_big_bucks_position(&self) -> bool {
        let square = &self.board[BIG_BUCKS_SOURCE];
        square.prizes.iter().any(|prize| {
            matches!(
                &prize.prize_type,
                PrizeType::Special(SpecialAction::BigBucks)
            )
        })
    }
}
