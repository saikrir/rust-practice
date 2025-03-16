use chrono::prelude::*;

fn weeks_between(a: &str, b: &str) -> i32 {
    let dt_fmt = "%Y-%m-%d";
    let dt_a = NaiveDate::parse_from_str(a, dt_fmt).unwrap();
    let dt_b = NaiveDate::parse_from_str(b, dt_fmt).unwrap();

    if dt_b > dt_a {
        (dt_b - dt_a).num_weeks() as i32
    } else {
        -(dt_a - dt_b).num_weeks() as i32
    }
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
