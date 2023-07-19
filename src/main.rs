use owo_colors::OwoColorize;
use rand::{prelude::*, seq::SliceRandom, Rng};
use tabled::{builder::Builder, settings::Style, Table};

fn gen_round(
    rng: &mut ThreadRng,
    solo: &'static str,
    gen_excerpts: &mut Vec<String>,
    high_excerpts: &[&'static str],
    round: i32,
) -> Vec<String> {
    let high_excerpt = high_excerpts[rng.gen_range(0..2)];

    // choose 5 random standard excerpts
    let len = gen_excerpts.len();
    let range = len - 5..len;
    let mut excerpts: Vec<_> = gen_excerpts.drain(range).collect();

    // choose one high, then randomize it in
    excerpts.push(high_excerpt.cyan().to_string());
    excerpts.shuffle(rng);

    let mut list = vec![];

    // only play solo in prelim and final
    if round != 1 {
        list.push(solo.bright_yellow().to_string());
    }
    list.append(&mut excerpts);

    list
}

fn build_table(rounds: [Vec<String>; 3]) -> Table {
    use std::cmp::max;

    let mut builder = Builder::default();
    builder.set_header([
        "PRELIMS".magenta().to_string(),
        "SEMIS".magenta().to_string(),
        "FINALS".magenta().to_string(),
    ]);

    let len = max(rounds[0].len(), max(rounds[1].len(), rounds[2].len()));
    let default = "".to_string();

    for i in 0..len {
        builder.push_record(rounds.iter().map(|r| r.get(i).unwrap_or(&default)));
    }

    let mut table = builder.build();
    table.with(Style::modern());
    table
}

fn generate_rounds(
    mut solos: Vec<&'static str>,
    mut excerpts: Vec<String>,
    high_excerpts: Vec<&'static str>,
) {
    let mut rng = rand::thread_rng();
    excerpts.shuffle(&mut rng);
    solos.shuffle(&mut rng);

    let prelims = gen_round(&mut rng, solos[0], &mut excerpts, &high_excerpts, 0);
    let semis = gen_round(&mut rng, solos[0], &mut excerpts, &high_excerpts, 1);
    let finals = gen_round(&mut rng, solos[1], &mut excerpts, &high_excerpts, 2);

    let table = build_table([prelims, semis, finals]);
    println!("{}", "Generating today's excerpt list:\n".green().bold());
    println!("{table}");
}

/*

Prelim: 1 solo, 1 high, 1 low, 4 standard
Semi: 1 high, 1 low, 5 standard
Finals: 1 solo, 1 high, 1 low, 5 standard

for the purposes of practice, don't duplicate solos or standard excerpts.
*/

fn richmond_3rd() {
    let solos = vec!["Norma", "Charlier 6"];
    let excerpts: Vec<_> = vec![
        "Leonore 3",
        "Outdoor Overture",
        "Mahler 5",
        "Promenade",
        "Petrushka",
        "Dvorak 8",
        "Scheherazade",
        "Schumann 2",
        "Nocturnes",
        "Bruckner 8",
        "Prokofiev 5",
        "Don Juan",
        "Don Quixote",
        "Till Eulenspiegel",
        "Ein Heldenleben",
    ]
    .into_iter()
    .map(String::from)
    .collect();

    let high_excerpts = vec!["Alpine Symphony", "Four Sea Interludes"];

    generate_rounds(solos, excerpts, high_excerpts);
}

fn main() {
    richmond_3rd();
}
