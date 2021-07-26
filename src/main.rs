use itertools::Itertools;
use indicatif::ProgressBar;

struct House {
    owner: Person,
    color: Color,
    gift: Gift,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Person {
    Amber,
    Bill,
    Chris,
    Diana,
    Emma,
    Fred,
    Gina,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Color {
    Yellow,
    Red,
    Blue,
    Gray,
    Green,
    Orange,
    Purple,
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Gift {
    Palette,
    Binoculars,
    Compass,
    PhoneBook,
    None,
}


// Rules Function Definitions
fn rule1(street: &Vec<House>) -> bool {
    for i in 0..3 {
        if (*street)[i].owner == Person::Gina && (*street)[i + 4].owner == Person::Chris {
            return true;
        }
    }
    false
}

fn rule2(street: &Vec<House>) -> bool {
    for i in 0..7 {
        if (*street)[i].owner == Person::Amber && (*street)[i].gift == Gift::None {
            return true;
        }
    }
    false
}

fn rule3(street: &Vec<House>) -> bool {
    for i in 0..1 {
        if (*street)[i].gift == Gift::PhoneBook && (*street)[i + 6].color == Color::Red {
            return true;
        }
    }
    false
}

fn rule4(street: &Vec<House>) -> bool {
    for i in 0..7 {
        if (*street)[i].owner == Person::Bill && (*street)[i].gift == Gift::Binoculars {
            return true;
        }
    }
    false
}

fn rule5(street: &Vec<House>) -> bool {
    for i in 0..5 {
        if (*street)[i].owner == Person::Bill && (*street)[i + 2].color == Color::Yellow {
            return true;
        }
    }
    false
}

fn rule6(street: &Vec<House>) -> bool {
    for i in 0..5 {
        if (*street)[i].owner == Person::Diana && (*street)[i].color == Color::Red {
            return true;
        }
    }
    false
}

fn rule7(street: &Vec<House>) -> bool {
    for i in 2..7 {
        if (*street)[i].owner == Person::Gina {
            let mut c: usize = 0;
            for j in 0..i {
                if (*street)[j].gift != Gift::None {
                    c += 1;
                }
            }
            return c == 2;
        }
    }
    false
}


fn main() {
    let rules: Vec<fn(&Vec<House>) -> bool> = vec![
        rule1,
        rule2,
        rule3,
        rule4,
        rule5,
        rule6,
        rule7,
    ];
    let mut street: Vec<House> = vec![House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},
                                      House{owner: Person::Amber, color: Color::Yellow, gift: Gift::Palette,},];
    let people: Vec<Person> = vec![Person::Amber,
                                   Person::Bill,
                                   Person::Chris,
                                   Person::Diana,
                                   Person::Emma,
                                   Person::Fred,
                                   Person::Gina,];
    let colors: Vec<Color> = vec![Color::Yellow,
                                  Color::Red,
                                  Color::Blue,
                                  Color::Gray,
                                  Color::Green,
                                  Color::Orange,
                                  Color::Purple,];
    let gifts: Vec<Gift> = vec![Gift::Palette,
                                Gift::Binoculars,
                                Gift::Compass,
                                Gift::PhoneBook,
                                Gift::None,
                                Gift::None,
                                Gift::None,];

    let people_bar = ProgressBar::new(7*6*5*4*3*2);
    let colors_bar = ProgressBar::new(7*6*5*4*3*2);
    'outer: for people_perm in people.iter().permutations(people.len()).unique() {

        for i in 0..7 {
            street[i].owner = *people_perm[i];
        }

        for colors_perm in colors.iter().permutations(colors.len()).unique() {

            for i in 0..7 {
                street[i].color = *colors_perm[i];
            }

            for gifts_perm in gifts.iter().permutations(gifts.len()).unique() {

                for i in 0..7 {
                    street[i].gift = *gifts_perm[i];
                }

                let mut correct: bool = true;
                'rules_testing: for func in &rules {
                    if !(*func)(&street) {
                        correct = false;
                        break 'rules_testing;
                    }
                }

               if correct {
                   println!("{:?} || {:?} || {:?}", people_perm, colors_perm, gifts_perm);
                   for i in 0..7 {
                       println!("House {:?}: {:?} || {:?} || {:?}", i, street[i].owner, street[i].color, street[i].gift);
                   }
                   break 'outer;
               }
            }
            colors_bar.inc(1);
        }
        colors_bar.finish();
        people_bar.inc(1);
    }
    people_bar.finish();
}
