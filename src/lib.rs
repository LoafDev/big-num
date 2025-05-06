use std::{cmp::Ordering, collections::VecDeque, fmt::{self, Display, Formatter}, ops::{Add, Mul, Sub}};

type BigNumType = u8;
#[derive(Debug, Clone)]
pub struct BigNum{
    num: VecDeque<BigNumType>,
    neg: bool
}

impl Display for BigNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.neg { write!(f, "-")? }
        for num in &self.num { write!(f, "{}", num)?; }
        Ok(())
    }
}

fn compare_bignum(a: &BigNum, b: &BigNum) -> Ordering {
    if a.len() < b.len() { Ordering::Less }
    else if a.len() > b.len() { Ordering::Greater }
    else {
        for i in 0..a.len() {
            if a.num[i] < b.num[i] { return Ordering::Less; }
            else if a.num[i] > b.num[i] { return Ordering::Greater; }
        }
        Ordering::Equal
    }
}

impl BigNum {
    fn new_zero() -> Self {
        BigNum {
            num: VecDeque::from([0]),
            neg: false
        }
    }
    pub fn new_with(s: String) -> Self {
        let s = s.trim();
        let mut neg=false;
        let mut i = 0;
        if s.chars().nth(0) == Some('-') { neg = true; i += 1; }
        while s.chars().nth(i) == Some('0') { i+=1; }

        if i >= s.len() { BigNum::new_zero() }
        else {
            BigNum {
                num: s[i..].chars().map(|x| x.to_digit(10).unwrap() as BigNumType).collect(),
                neg
            }
        }
    }
    pub fn len(&self) -> usize { self.num.len() }
    pub fn with_neg(&self, neg: bool) -> Self {
        BigNum {
            num: self.num.clone(),
            neg
        }
    }
}

impl Add for &BigNum {
    type Output = BigNum;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.num[0], rhs.num[0]) {
            (0,0) => return BigNum::new_zero(),
            (_,0) => return self.clone(),
            (0,_) => return rhs.clone(),
            _ => {}
        }

        let mut neg = false;
        match (self.neg, rhs.neg) {
            (true, true) => neg = true,
            (true, false) => return rhs-&self.with_neg(false),
            (false, true) => return self-&rhs.with_neg(false),
            _ => {}
        }

        let (mut i, mut j, mut rem) = (self.len() as isize -1,rhs.len() as isize -1,0);
        let (mut tempo, mut a, mut b);
        let mut num = VecDeque::new();

        while i >= 0 || j >= 0 || rem != 0 {
            a = if i >= 0 { self.num[i as usize] } else { 0 };
            b = if j >= 0 { rhs.num[j as usize] } else { 0 };

            tempo = a + b + rem;

            num.push_front(tempo%10);
            rem = tempo/10;

            i -= 1;
            j -= 1;
        }

        BigNum {
            num,
            neg
        }
    }
}

impl Sub for &BigNum {
    type Output = BigNum;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.num[0], rhs.num[0]) {
            (0,0) => return BigNum::new_zero(),
            (_,0) => return self.clone(),
            (0,_) => { return rhs.with_neg(!rhs.neg); }
            _ => {}
        }

        match (self.neg, rhs.neg) {
            (true, true) => return &rhs.with_neg(false)-&self.with_neg(false),
            (true, false) => return self+&rhs.with_neg(true),
            (false, true) => return self+&rhs.with_neg(false),
            _ => {}
        }

        let mut big = self.clone();
        let mut small = rhs.clone();
        let mut neg = false;

        match compare_bignum(self, rhs) {
            Ordering::Less => { std::mem::swap(&mut big, &mut small); neg = true; },
            Ordering::Equal => return BigNum::new_zero(),
            Ordering::Greater => {}
        }

        let mut num = VecDeque::new();
        let (mut tempo, mut a, mut b): (isize,isize,isize);
        let (mut i, mut j, mut rem) = (big.len() as isize -1, small.len() as isize -1, 0);

        while i >= 0 || j >= 0 {
            a = if i >= 0 { big.num[i as usize] as isize } else { 0 }; 
            b = if j >= 0 { small.num[j as usize] as isize } else { 0 };

            tempo = a - b - rem;

            num.push_front( if tempo < 0 { (tempo + 10) as BigNumType } else { tempo as BigNumType } );
            rem = if tempo < 0 { 1 } else { 0 };

            i -= 1;
            j -= 1;
        }

        while num.front() == Some(&0) { num.pop_front(); }
        BigNum {
            num,
            neg
        }
    }
}

impl Mul for &BigNum {
    type Output = BigNum;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.num[0] == 0 || rhs.num[0] == 0 { return BigNum::new_zero(); }

        let mut c = self.clone(); c.neg = false;
        let mut res = BigNum::new_zero();
        let mut tempo;
        let (mut times,mut rem) = (0,0);

        for i in rhs.num.iter().rev() {
            for j in (0..c.len()).rev() {
                tempo = c.num[j] * i + rem;
                c.num[j] = tempo%10;
                rem = tempo/10;
            }
            if rem != 0 { c.num.push_front(rem); rem = 0; }
            for _ in 0..times { c.num.push_back(0); }

            res = &res + &c;
            c = self.clone(); c.neg = false;
            times += 1;
        }
        res.neg = self.neg ^ rhs.neg;
        res
    }
}
