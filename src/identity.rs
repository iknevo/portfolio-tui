pub const BIRTH_DATE: &str = "2002-09-21";

pub const EMAIL: &str = "iknevo.dev@gmail.com";

pub const RESUME_URL: &str = "https://nevo.is-a.dev/resume";

pub const SOCIALS: &[(&str, &str)] = &[
    ("github", "https://github.com/iknevo"),
    ("linkedin", "https://www.linkedin.com/in/ahmed-abdelhafiez"),
];

pub const TAGLINE: &str =
    "Frontend web developer based in Cairo, Egypt. I turn ideas into seamless, scalable, and \
     performant user experiences.";

pub fn age() -> u32 {
    let now = chrono::Utc::now().date_naive();
    let birth = chrono::NaiveDate::parse_from_str(BIRTH_DATE, "%Y-%m-%d")
        .unwrap_or_else(|_| chrono::NaiveDate::from_ymd_opt(2002, 9, 21).unwrap());
    now.years_since(birth).unwrap_or(0)
}
