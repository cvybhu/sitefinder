#![allow(clippy::needless_return)]
use rand::{distributions::Uniform, prelude::ThreadRng, thread_rng, Rng};
use tokio::sync::mpsc;

struct Rander {
    rng: ThreadRng,
    vowels_dist: Uniform<usize>,
    nonvowels_dist: Uniform<usize>,
    name_len_dist: Uniform<usize>,
}

const VOWELS: [char; 4] = ['a', 'o', 'e', 'i'];
const NONVOWELS: [char; 8] = ['t', 'p', 's', 'd', 'l', 'b', 'n', 'm'];

impl Rander {
    fn new() -> Self {
        return Rander {
            rng: thread_rng(),
            vowels_dist: Uniform::new(0, VOWELS.len()),
            nonvowels_dist: Uniform::new(0, NONVOWELS.len()),
            name_len_dist: Uniform::new(6, 8),
        };
    }

    fn get_rand_vowel(&mut self) -> char {
        return VOWELS[self.rng.sample(self.vowels_dist)];
    }

    fn get_rand_nonvowel(&mut self) -> char {
        return NONVOWELS[self.rng.sample(self.nonvowels_dist)];
    }

    fn get_rand_name(&mut self) -> String {
        let name_len: usize = self.rng.sample(self.name_len_dist);
        let mut letters: Vec<char> = Vec::with_capacity(name_len);

        for i in 0..name_len {
            if i % 2 == 0 {
                letters.push(self.get_rand_nonvowel());
            } else {
                letters.push(self.get_rand_vowel());
            }
        }

        return letters.into_iter().collect();
    }
}

async fn is_name_taken(name: &str) -> bool {
    let dns_lookup_future = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        tokio::net::lookup_host(name),
    );

    match dns_lookup_future.await {
        Ok(Ok(mut addr_iter)) => match addr_iter.next() {
            Some(_) => return true,
            None => return false,
        },
        _ => return false,
    };
}

async fn find_name(results_sender: mpsc::Sender<String>) {
    loop {
        let cur_name: String = Rander::new().get_rand_name();

        if !is_name_taken(&(cur_name.clone() + ".com:80")).await {
            let _ = results_sender.send(cur_name).await;
        }
    }
}

async fn find_site_names(wanted_names_number: usize, concurrent_dns_lookups: usize) {
    let (results_sender, mut results_receiver) = mpsc::channel(1);

    for _ in 0..concurrent_dns_lookups {
        let results_sender = results_sender.clone();
        tokio::spawn(async move { find_name(results_sender).await });
    }

    println!("\nResults:");

    let mut found_good_names = 0;
    while let Some(good_name) = results_receiver.recv().await {
        println!("{}.com", good_name);

        found_good_names += 1;
        if found_good_names >= wanted_names_number {
            break;
        }
    }
}

fn main() {
    let wanted_names_number = 64;
    let concurrent_dns_lookups = 8;

    let runtime = tokio::runtime::Runtime::new().expect("Tokio runtime creation failed!");
    runtime.block_on(find_site_names(wanted_names_number, concurrent_dns_lookups));

    std::process::exit(0);
    // TODO - is there a better way to drop all things in runtime without waiting for them to finish?
    // runtime.shutdown_background(); still waits for DNS lookups to finish :/
}
