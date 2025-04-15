mod invoker;
use invoker::Invoker;
use rand::seq::SliceRandom;
use rand::thread_rng;
const INVOKE_QUOTES: &[&str] = &[
    "One of my favorites.",
    "Spherical sorcery!",
    "Behold!",
    "A rich tradition.",
    "Drawn from deep within.",
    "From the great mystery.",
    "A spell I well remember.",
    "True Arcanery thrives!",
    "Plucked from the Arcanery.",
    "My mind is my Arcanery.",
    "An incantation long remembered.",
    "A charming hex.",
    "Enlightenment stands ready!",
    "Words of power.",
    "Witness true sorcery!",
    "Augury abounds.",
    "Arcana known only to me!",
];

const FAILED_QUOTES: &[&str] = &[
    "This spell works nowise!",
    "This magic disappoints.",
    "My memory fails me.",
    "Did I miscast?",
    "My concentration--shattered!",
    "Did I mix my magics?",
    "Words fail me?",
    "Ruinous, ill-fated spell.",
    "Perturbations!",
    "Lamentable!",
    "Infelicitous!",
    "The errant cosmos works against me!",
    "No matter.",
];
use crossterm::{
    cursor,
    event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::collections::VecDeque;
use std::io::{stdout, Write};

fn draw_input(buffer: &VecDeque<char>, invoked_spells: &VecDeque<String>, message: Option<&str>) {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
    println!("Invoker Trainer (Press Esc to Exit)\n");

    for i in 0..3 {
        if let Some(&ch) = buffer.get(i) {
            print!("[ {} ] ", ch);
        } else {
            print!("[   ] ");
        }
    }

    println!("\n\nSkill Slots:");
    for i in 0..2 {
        if let Some(spell) = invoked_spells.get(i) {
            print!("[ {} ] ", spell);
        } else {
            print!("[       ] ");
        }
    }

    if let Some(msg) = message {
        println!("\n\n{}", msg);
    } else {
        println!("\n\nPress Q, W, E for Ice, Thunder and Fire...");
    }
}


fn main() -> std::io::Result<()> {
    let mut info: Option<&str> = None;
    enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let mut buffer: VecDeque<char> = VecDeque::with_capacity(3);
    let mut invoked_spells: VecDeque<String> = VecDeque::with_capacity(2);
    let invoker = Invoker::new();

    draw_input(&buffer, &invoked_spells,info);
    let mut rng = thread_rng();
    loop {
        if let Event::Key(KeyEvent { code, kind, .. }) = read()? {
            if kind != KeyEventKind::Press {
                continue;
            }

            match code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    if buffer.len() == 3 {
                        buffer.pop_front();
                    }
                    buffer.push_back('Q');
                }
                KeyCode::Char('w') | KeyCode::Char('W') => {
                    if buffer.len() == 3 {
                        buffer.pop_front();
                    }
                    buffer.push_back('W');
                }
                KeyCode::Char('e') | KeyCode::Char('E') => {
                    if buffer.len() == 3 {
                        buffer.pop_front();
                    }
                    buffer.push_back('E');
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    if buffer.len() == 3 {
                        let combo: String = buffer.iter().collect();
                        if let Some(&spell_name) = invoker.get_spell(&combo) {
                            if invoked_spells.len() == 2 && invoked_spells[1] == spell_name {
                                invoked_spells.swap(0, 1);
                            } else if invoked_spells.front().map_or(false, |s| s == spell_name) {
                                info = FAILED_QUOTES.choose(&mut rng).copied();
                            } else {
                                if invoked_spells.len() == 2 {
                                    invoked_spells.pop_back();
                                }
                                invoked_spells.push_front(spell_name.to_string());
                                info = INVOKE_QUOTES.choose(&mut rng).copied();
                            }
                        }
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }

            draw_input(&buffer, &invoked_spells,info);
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), cursor::Show)?;
    println!("\nExit Trainer");
    Ok(())
}