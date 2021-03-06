#[macro_use]
extern crate nom;

use std::str::{self, FromStr};
use nom::{alphanumeric, digit, space, eol};

named!(name<&str>,
    map_res!(
        alphanumeric,
        str::from_utf8
    )
);

named!(number<usize>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

#[derive(Debug, PartialEq, Eq)]
pub struct Reindeer<'a> {
    name: &'a str,
    speed: usize,
    fly_time: usize,
    rest_time: usize,
}

named!(pub reindeer<Reindeer>,
    chain!(
        name: name ~
        space ~ tag!("can fly") ~ space ~
        speed: number ~
        space ~ tag!("km/s for") ~ space ~
        fly_time: number ~
        space ~ tag!("seconds, but then must rest for") ~ space ~
        rest_time: number ~
        space ~ tag!("seconds."),
        || Reindeer { name: name, speed: speed, fly_time: fly_time, rest_time: rest_time }
    )
);

impl<'a> Reindeer<'a> {
    fn distance_after_time(&self, t: usize) -> usize {
        let cycle_time = self.fly_time + self.rest_time;
        let full_cycles = t / cycle_time;
        let full_cycles_time = full_cycles * cycle_time;
        let remaining_time = t - full_cycles_time;
        let remaining_fly_time = *[remaining_time, self.fly_time].iter().min().unwrap();
        let fly_time = full_cycles * self.fly_time + remaining_fly_time;
        fly_time * self.speed
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Race<'a>(Vec<Reindeer<'a>>);

named!(pub race<Vec<Reindeer> >,
    complete!(
        separated_list!(
            eol,
            reindeer
        )
    )
);

impl<'a> Race<'a> {
    fn new(input: &str) -> Race {
        Race(race(input.as_bytes()).unwrap().1)
    }

    fn distance_after_time(&self, t: usize) -> Vec<(&Reindeer, usize)> {
        self.0.iter().map(|r| (r, r.distance_after_time(t))).collect()
    }

    fn max_distance_after_time(&self, t: usize) -> (&Reindeer, usize) {
        self.distance_after_time(t).into_iter().max_by_key(|&(_, dist)| dist).unwrap()
    }

    fn points_after_time(&self, t: usize) -> Vec<(&Reindeer, usize)> {
        if t == 0 {
            self.0.iter().map(|r| (r, 0)).collect()
        } else {
            let dists = self.distance_after_time(t);
            let max_dist = dists.iter().max_by_key(|&&(_, dist)| dist).unwrap().1;
            self.points_after_time(t - 1).iter().enumerate().map(|(i, &(r, mut pts))| {
                assert_eq!(dists[i].0, r);
                if dists[i].1 == max_dist { pts += 1 }
                (r, pts)
            }).collect()
        }
    }

    fn max_points_after_time(&self, t: usize) -> (&Reindeer, usize) {
        self.points_after_time(t).into_iter().max_by_key(|&(_, pts)| pts).unwrap()
    }
}

fn main() {
    let race = Race::new(include_str!("day14.txt"));
    println!("Distance of winning reindeer after 2503s: {}", race.max_distance_after_time(2503).1);
    println!("Points of winning reindeer after 2503s: {}", race.max_points_after_time(2503).1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        assert_eq!(reindeer(b"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.").unwrap(),
            (&b""[..], Reindeer { name: "Comet", speed: 14, fly_time: 10, rest_time: 127 }));
    }

    #[test]
    fn distance_calculation() {
        let comet = Reindeer { name: "Comet", speed: 14, fly_time: 10, rest_time: 127 };
        let dancer = Reindeer { name: "Dancer", speed: 16, fly_time: 11, rest_time: 162 };
        assert_eq!(comet.distance_after_time(1), 14);
        assert_eq!(dancer.distance_after_time(1), 16);
        assert_eq!(comet.distance_after_time(10), 140);
        assert_eq!(dancer.distance_after_time(10), 160);
        assert_eq!(comet.distance_after_time(11), 140);
        assert_eq!(dancer.distance_after_time(11), 176);
        assert_eq!(comet.distance_after_time(12), 140);
        assert_eq!(dancer.distance_after_time(12), 176);
        assert_eq!(comet.distance_after_time(137), 140);
        assert_eq!(comet.distance_after_time(138), 154);
        assert_eq!(dancer.distance_after_time(173), 176);
        assert_eq!(dancer.distance_after_time(174), 192);
        assert_eq!(comet.distance_after_time(1000), 1120);
        assert_eq!(dancer.distance_after_time(1000), 1056);
    }

    #[test]
    fn race_parsing() {
        let race = Race::new("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");
        assert_eq!(race, Race(vec![
            Reindeer { name: "Comet", speed: 14, fly_time: 10, rest_time: 127 },
            Reindeer { name: "Dancer", speed: 16, fly_time: 11, rest_time: 162 },
        ]));
    }

    #[test]
    fn racing() {
        let race = Race::new("Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nDancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");
        let comet = &race.0[0];
        let dancer = &race.0[1];
        assert_eq!(race.distance_after_time(1000), [(comet, 1120), (dancer, 1056)]);
        assert_eq!(race.max_distance_after_time(1000), (comet, 1120));
        assert_eq!(race.points_after_time(1000), [(comet, 312), (dancer, 689)]);
        assert_eq!(race.max_points_after_time(1000), (dancer, 689));
    }
}
