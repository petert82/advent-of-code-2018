use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Checksum {
    two: i64,
    three: i64,
}

impl Checksum {
    fn new() -> Checksum {
        Checksum {
            two: 0,
            three: 0,
        }
    }

    fn set_two(mut self) -> Checksum {
        match self.two {
            0 => self.two = 1,
            _ => ()
        }

        self
    }

    fn set_three(mut self) -> Checksum {
        match self.three {
            0 => self.three = 1,
            _ => ()
        }

        self
    }

    fn has_two_and_three(&self) -> bool {
        self.two > 0 && self.three > 0
    }

    fn checksum(&self) -> i64 {
        self.two * self.three
    }
}

impl Add for Checksum {
    type Output = Checksum;

    fn add(self, other: Checksum) -> Checksum {
        Checksum {
            two: self.two + other.two,
            three: self.three + other.three,
        }
    }
}

// Entry point for day 1, part 1
pub fn calculate_checksum(lines: impl AsRef<str>) -> i64 {
    lines
        .as_ref()
        .lines()
        .map(|line| parse_line(line))
        .fold(Checksum::new(), |acc, c| acc + c)
        .checksum()
}

fn parse_line(line: &str) -> Checksum {
    let mut checksum = Checksum::new();
    let mut letter_freqs = HashMap::new();

    for char in line.chars() {
        let count = letter_freqs.entry(char).or_insert(0);
        *count += 1;
    }

    for count in letter_freqs.values() {
        checksum = match count {
            2 => checksum.set_two(),
            3 => checksum.set_three(),
            _ => checksum
        };
        if checksum.has_two_and_three() {
            break;
        }
    }

    checksum
}

// Entry point for day 2, part 2
pub fn find_similar_ids(lines: impl AsRef<str>) -> Option<String> {
    for id_a in lines.as_ref().lines() {
        for id_b in lines.as_ref().lines() {
            match get_similar(id_a, id_b) {
                Some(similar) => return Some(similar),
                None => (),
            }
        }
    }
    None
}

// Checks if the strings are similar apart from one character, if yes, returns a string
// containing the common characters.
fn get_similar(id_a: &str, id_b: &str) -> Option<String> {
    if id_a.len() != id_b.len() {
        return None;
    }

    let mut found_diff = false;
    let mut similar = String::with_capacity(id_a.len());

    for (a, b) in id_a.chars().zip(id_b.chars()) {
        match a == b {
            true => similar.push(a),
            false if found_diff => return None,
            false => found_diff = true,
        }
    }

    match id_a.len() == similar.len() {
        true => None,
        false => Some(similar),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("abcdef"), Checksum{two: 0, three: 0});
        assert_eq!(parse_line("bababc"), Checksum{two: 1, three: 1});
        assert_eq!(parse_line("abbcde"), Checksum{two: 1, three: 0});
        assert_eq!(parse_line("abcccd"), Checksum{two: 0, three: 1});
        assert_eq!(parse_line("aabcdd"), Checksum{two: 1, three: 0});
        assert_eq!(parse_line("abcdee"), Checksum{two: 1, three: 0});
        assert_eq!(parse_line("ababab"), Checksum{two: 0, three: 1});
    }

    #[test]
    fn test_calculate_checksum() {
        let lines = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        assert_eq!(calculate_checksum(lines), 12);
    }

    #[test]
    fn test_get_similar() {
        assert_eq!(get_similar("a", ""), None);
        assert_eq!(get_similar("a", "a"), None);
        assert_eq!(get_similar("abcde", "axcye"), None);
        assert_eq!(get_similar("fghij", "fguij"), Some("fgij".to_string()));
    }

    #[test]
    fn test_find_similar_ids() {
        assert_eq!(find_similar_ids("abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz"), Some("fgij".to_string()));
    }
}

pub fn get_input() -> &'static str {
    "tjxmoewpqkyaiqvmndgflunszc
tjxmobwpqkyaihvrndgfjubszm
tjxmzewpqkyaihvrydgflrbszc
tjxmoeypqkyvihvrndgflubsza
tjcmoewpqkytihvrndgflgbszc
tjvmoewpqkyanevrndgflubszc
tjxmoewpqkdiihirndgflubszc
tjxboewpqkyaihbrnogflubszc
ojpmoewpqkyaihvjndgflubszc
tjxyoewpqkyaiuvrndgflutszc
tjxmoewpqkyalhvrndmflebszc
tjxmoewpqzyaihhrndgflubszf
tjxmrewpqkyaihirndgfjubszc
pjxmoewpqkyaihvendgfbubszc
txxmkewpqkyjihvrndgflubszc
tjxmoewcqkyaihvrnmgflubczc
tjxmoewkqkyaghvrndgfluvszc
tjxmoewfqkhaihvrndgflubhzc
jjqmoewpqkyaihvrndzflubszc
tjxmoewmqksaihvcndgflubszc
tjrmoewpqkyaihvrvdgflubzzc
tjxxoewpqkyaiiwrndgflubszc
cjxmoawxqkyaihvrndgflubszc
tjxdoewpvkyaihvrndgflubsoc
tjxmsewpqkyaihvrndgfluzozc
tjxmoewpqkyafhvrnyeflubszc
tjxmlewpqkyawhvondgflubszc
tjxmonwpqkyaiqvrnxgflubszc
tjxmoewcqkyaihvrnjgflumszc
tjvmoewpqkyaihveadgflubszc
tjxmogfpqkyaigvrndgflubszc
tybmoewpqkyaihvrndgllubszc
tjxmoewpdkyaihvrndgfluwbzc
etxmbewpqkyaihvrndgflubszc
tjxmoeapqcynihvrndgflubszc
tbxmoewpqkyaihvrndgfdebszc
haxmoewpqyyaihvrndgflubszc
ojxmoewpqkyaihvrnegflubszr
tjxmoewpqkyaihvrndoflubarc
ljxmoewpqkykihvrndgflvbszc
tjxmovwpqkyaihvrndgfluzsyc
tvxmoewpqkyanhvrkdgflubszc
tjxmoewpqkyaihkrndgfluwwzc
zjxmoewpfkyaihvrndgfrubszc
tjxyoegpqkyaihvrndlflubszc
tjxmoewpqkyamhvrnsgflubmzc
tjmmoewpqkyaihvrndgftuwszc
tjxmoewpqbraihvrncgflubszc
tjxmeeepqkyainvrndgflubszc
tjemoegpqkyaihvredgflubszc
tjxmoewpqkyaihvdndgfzubqzc
tjxmoegrqkyaihfrndgflubszc
tjxmoewpqxyaihvrndgfluyvzc
qjxmoewpqkyaiwvrnfgflubszc
tjxwoewpqkyashkrndgflubszc
tjzmoewiqkyaihurndgflubszc
tjumuewpqkyaihvrndgflubssc
tyxooewpukyaihvrndgflubszc
tjxvoewpqkyaiivindgflubszc
ijxmoqwpqkyaihvradgflubszc
tjxmlewpqkyaihvrhdgflubwzc
tjxmkewpqkyajhqrndgflubszc
tjxmoewpqkqaiherndgflurszc
tjamoewpqkyaizvondgflubszc
tjxgogwpqkyalhvrndgflubszc
tjxmoewpqkyachvrndgflubuzq
tjxmowqpqkyaihvrnegflubszc
mjxmoewpwkyaihvrndgfkubszc
tpbmoewpqkyaihvrzdgflubszc
tjbmoewpqkyaiuvrndgflsbszc
tjxmoewpqklaghvrndgflubazc
tjxmoewpqkyrihvrndgwlpbszc
tjcmoewpqksaiyvrndgflubszc
tjxmoeapqkymihvindgflubszc
tjxmdewpqkyafhvrndgflqbszc
tjxmoewpqxyaihvrndsflubszi
tjxmoeppqkyaihvrcdgflubszd
tjxmomwpqkyainvrmdgflubszc
tjxmovwpqkyaihvrndgfdubdzc
tjxmoewwqkiaihvrjdgflubszc
tmxmoewpqkyaifvrndgflubszs
tbxmoewpqkyaihvrbdgflunszc
tjxmoewrqkyaihvxndgflubszp
ujxmoewpqkyaihvxndgflubpzc
tdxmotwpqkyaihvdndgflubszc
tjxmvewpqkyaihfrndgtlubszc
tjfmoewpqkyaihvrnyqflubszc
tjxfolwzqkyaihvrndgflubszc
ojrmoiwpqkyaihvrndgflubszc
tjsmoqwpqkyqihvrndgflubszc
tjxmohwpqkyaihvrudgflubslc
tjxtoiwpqkyaihvrnogflubszc
taxmoewpqkyaiyvrndgfwubszc
tjxwnezpqkyaihvrndgflubszc
tjxmyevpqkyaivvrndgflubszc
tjxdoeopqkyaihvgndgflubszc
tjxaoewpqkmaihvrndgflufszc
tjxmoewpqkyaxhvrndgflubncc
tjxmoewpqkyaihurndgflubbjc
tjxmjewpqgyaihvrnngflubszc
tjxmogwpqkyaihvrndgflubbcc
tjxmoewplkyaihvrnpgflibszc
tjwmoewpqkyaohvrndgfbubszc
tjwmoewpqkyaihvrndgfsubszm
tjxmogwpqkyaihvrndiflubqzc
tjxmoewpqkyaihvrndgflopshc
rjxmlewpvkyaihvrndgflubszc
tjxmogwpakyaihvrndgflzbszc
tjxmoeppqkyaihvrndgflmxszc
tjxmoewpqkyhihgrndgfzubszc
tjxqoewpqkyaihtrndgwlubszc
tjxnoespqkyaihvrndgflubsuc
tjmmoewpqkraihvrndgflfbszc
tjxmoewnqkwaihvrndgflubstc
tjxmoewpqqyaihvrndgfljbszi
tjxmoewpqkyaihkrkdgalubszc
tjxmoewpqkyaihvradgjlurszc
tvxmoewpqkybihvrndbflubszc
tjxvoewpqkyaihvradgfoubszc
tjxmoewpqfyaihvlodgflubszc
tjxmoewmnkyaiivrndgflubszc
kjxmoewpqkyaihprndgflcbszc
hjxmoewpqkcaihvrndgvlubszc
tjxmoewcqkyaihvrncgfllbszc
tuxmoewpckyaihvrndoflubszc
tjxmdewpokyaihvrndgflubszn
mjxmaewpqkyaqhvrndgflubszc
tjxmoewpmzyaihvrndgfiubszc
tjxmoewnqkyvihvrndgflubszk
tjxmoewpmnyaihvrndgftubszc
zjxmoewpqkysihvrndgfmubszc
tjxmoewpqkyaihzrntgflubbzc
tjxmoewpqkgaihwrndsflubszc
tjxjoewpqkyaihvrndgflgbizc
oqxmoewpqkyaihvrndgfldbszc
wjamoewpqkyaihvfndgflubszc
tjxmoewtmkyvihvrndgflubszc
tjlmojwpqkyaihvrndgfludszc
tjxmowwpqkyaihvrndefludszc
tjxmoewpqkbaihvrndgfluaszt
tjxmoewpqkzaahvrodgflubszc
tjxmoewpqkgaihvrndgflubtpc
tjxmoenpqkyaihcrndgfqubszc
tbxmoewpqbyaihvrndgalubszc
tjvmoewqqkyaihvrndvflubszc
tjxmoewpqkeaihvundgfaubszc
txxmoewpqkyaihvrwdgflpbszc
tzxmoewpqkijihvrndgflubszc
tjxmoewoqkytiuvrndgflubszc
tjxmrejplkyaihvrndgflubszc
tjxmoewpqkynihvrpxgflubszc
tjxmoewpqkvanhvrndgvlubszc
tjxmoewpdkyiihvrndgflubszs
tpxmyewpqkyaihvrndgfeubszc
tpxmoewpqyyaihvrndhflubszc
tjsmoewpqkyaihvrndhflubnzc
tjxmoewpukyaihvrnmgflubwzc
txxmoewpqlyaihwrndgflubszc
tjxmoewprkyaiovrndgflubxzc
tjxmouwpqkyaihzrodgflubszc
tjxmojwpqkywimvrndgflubszc
tjxsoewpqkyaihvrzdgqlubszc
tfxmoewpakyaihvrndgllubszc
tjhmoewpqiyaihvrndgflubsac
tjxmoewpqkoaihvrndoflubsxc
tjxmoewpqkyzpjvrndgflubszc
tjxmoewpqkyaiharndgzlnbszc
tjimoevpqkyaihvrndgflubbzc
tjxsoewpqkyahhvrndgfzubszc
txxmoewpqkyaimvrrdgflubszc
tjxmoewpwkyaihvrndpylubszc
tjxmoewpskyaghvrndgfbubszc
tjxmuewpqmyaihvrndgfyubszc
tjxmoewpqkyaihvdndgglubsxc
xjxmoewpqkyjiovrndgflubszc
gjxmoewpqkyaihvrndodlubszc
tjbmoewpqkyaihvridgflvbszc
tjxmozwpqkyapbvrndgflubszc
tjxeoewpqkyqihvrndgflubhzc
tjxdoewpqzyaihvrndgflubsmc
tjxmwewpqkyathvcndgflubszc
tjxmoewpszyaihvrndgflusszc
tuxmoewpqkyaihvrndgfluasxc
tjemoewpnvyaihvrndgflubszc
tjxmoewpjkyaihvrndgjlufszc
tjomoewppkyaihvzndgflubszc
tjxmvewpqkyaimvrntgflubszc
rjxmoewpqkyaihvpndgflubszq
hjxzoewpqkyaihvridgflubszc
texmoejpqkyaihvrndgflubszx
tjxcoewpqkyaihbrxdgflubszc
tjxmoewpnkyaihvrndgfltbsze
tjxmoewpdkyaihvrndwfluwbzc
tjxmoewpqryjihkrndgflubszc
tjlmoewpqkhaihvrndgflubsnc
tjxmovapqkjaihvrndgflubszc
tjxvoewpqkyaihqrndgfluyszc
tjxwoewnqkyaihvrndgfgubszc
tjdmoewpqklaihvcndgflubszc
tjxmoewpvkynihvrndgflubskc
tjxmtewpqkyaihvhndgfluaszc
tjxmoewpqkyanhvrnpgfluvszc
tjxmoewpqkyaifvbndgflubspc
tjxmoexpqknaihvrndgxlubszc
qjxmoewqqkyaihvrndgflubpzc
tjxmoewppkyaihvaxdgflubszc
myxmoewpqkyaihvrudgflubszc
tjxmwewpmkyaihvrndgflubssc
tjxmoewpqkyaihvrndgfxqbszq
tjxmoewhqkyaahvrndgflubbzc
tbxmoewmqkyaihvrndgflubszu
toxmolwpqkyaihvrndnflubszc
tjxmoewhqkyaihvrnrgflubvzc
yjxmoewcqkyaihvrndgflubfzc
tjxmoewpqkyamhvrgdgflmbszc
tjxmtewpqkyaizvrndgfluoszc
tjxmoewpqzyaihvrntsflubszc
fjxmoewpqkyaihyrmdgflubszc
tjxwoewpqcyaihbrndgflubszc
tjxmoebpqkzaihvrndcflubszc
tjxmoewpqkyaihvrndnlmubszc
tjxmoewpqkyaihvrndgeyubskc
tfxmoewpqryaihvrndgfluiszc
tjxmoewpqkjaihvynngflubszc
tjxmoewpqkqaihvonjgflubszc
tjfmokwpqkyeihvrndgflubszc
djxmoewpkkyaihvrnmgflubszc
tjxmiewpqkyaihvrndgflubhlc
tjxmmejpqkyaihvrnhgflubszc
djxmoewmqkyaihvrnggflubszc
tjxmoewpqkyaihvrkggflubsze
gjxmoewpqkyaihjrndgflvbszc
tjxmoewpqkyaidvrndgkzubszc
tjxmoewpqkyaedvrnpgflubszc
sjxmoewpqkyaihvrnngfluhszc
tjxmoewpqkuaihvrndghlubxzc
tjxmoewgqkyuihvrndgftubszc
tjxmoewpqsyaifvrkdgflubszc
tjxrrewpqkyaihvrnpgflubszc
tjxmoezpqkyaihvrwdgflabszc
tjfmoewpqknaihvrndgflubkzc
tjxmoewnqkxaihvrndgflubtzc
tjxmoewpkkyaihvrndgfrnbszc
tjxmorwpnkqaihvrndgflubszc
tsxmoewwqkyathvrndgflubszc
tjxmoeupqkyaihvrndyflubszp
bjxmoewdqkyaihvrndgflurszc
tjxmoewpvkyaihvrndoflubszq
sjxmoewpqkyaihvrndgflubyec
tjxmoewpqkyaizcrndgfnubszc"
}