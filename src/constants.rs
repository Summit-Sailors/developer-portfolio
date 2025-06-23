#![allow(dead_code)] // Allow unused constants for now

use dioxus::prelude::*;
use crate::icons::{
    SiReact, SiNextdotjs, SiTailwindcss, SiTypescript, SiNodedotjs, SiPython,
    SiRust, SiSvelte, SiFastapi, SiNestjs, DiDojo,
    FiGithub, FiTwitter, FiLinkedin, FiMail, FiArrowDown, FiExternalLink, FiPhone, FiMapPin,
};


pub struct PersonalInfo {
    pub name: &'static str,
    pub title: &'static str,
    pub short_description: &'static str,
    pub email: &'static str,
    pub phone: &'static str,
    pub location: &'static str,
    pub story_intro: &'static str,
    pub story_detail: [&'static str; 3],
    pub github: &'static str,
    pub linkedin: &'static str,
    pub twitter: &'static str,
}

pub const PERSONAL_INFO_DATA: PersonalInfo = PersonalInfo {
    name: "Jeremy Meek",
    title: "Self-Starting Polymath Autodidact.",
    short_description: "I build robust and innovative solutions for the web and beyond. From cows, to calculus, to computers.",
    email: "jeremy.meek@example.com",
    phone: "+1 (555) 555-5555",
    location: "Southwest Virginia, USA",
    story_intro: "Hello! I'm Jeremy Meek, a software developer with a unique journey from farm life in Southwest Virginia, through a Bachelors in Mathematics at Virginia Tech, to a Computer Science education from Bloom Tech. I bring several years of full-stack web development experience from the corporate world, ready to tackle virtually any programming challenge.",
    story_detail: [
        "My path has been unconventional: from tending cows to unraveling calculus, and now, crafting complex computer systems. This diverse background has equipped me with a unique problem-solving perspective and a relentless drive for learning.",
        "While I'm proficient in mainstream frameworks and languages for professional projects, my passion lies with Rust for its performance and safety. My pet projects often involve Rust, sometimes combined with Python. Svelte is my go-to for frontend JavaScript work, and for non-Rust backends, I prefer FastAPI (Python) or NestJS (Node). When I get to use Rust for full-stack, Dioxus is my framework of choice.",
        "I consider myself an 'Artist Of All Trades' in the tech world, competent to satisfy a wide array of programming needs with creativity and precision.",
    ],
    github: "https://github.com/your-github-username", // Replace
    linkedin: "https://linkedin.com/in/your-linkedin-profile", // Replace
    twitter: "https://twitter.com/your-twitter-handle", // Replace
};

pub struct NavLink {
    pub name: &'static str,
    pub href_route: crate::Route, // Using the Route enum
}

pub const NAVIGATION_LINKS_DATA: [NavLink; 5] = [
    NavLink { name: "Home", href_route: crate::Route::HomePage {} },
    NavLink { name: "Projects", href_route: crate::Route::ProjectsPage {} },
    NavLink { name: "Skills", href_route: crate::Route::HomePage {} }, // Points to home, assuming skills section is there with #skills
    NavLink { name: "Story", href_route: crate::Route::StoryPage {} },
    NavLink { name: "Contact", href_route: crate::Route::ContactPage {} },
];


pub struct Stat {
    pub value: &'static str,
    pub label: &'static str,
}
pub const STATS_DATA: [Stat; 4] = [
    Stat { value: "N+", label: "Years of Experience" },
    Stat { value: "M+", label: "Projects Completed" },
    Stat { value: "B.S.", label: "Mathematics, Virginia Tech" },
    Stat { value: "Cert.", label: "Computer Science, Bloom Tech" },
];

pub enum SkillCategory {
    Favorite,
    Proficient,
    Backend,
}
impl SkillCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillCategory::Favorite => "Favorite",
            SkillCategory::Proficient => "Proficient",
            SkillCategory::Backend => "Backend",
        }
    }
}

pub struct SkillData {
    pub icon: fn() -> Element, // Function that returns an Element (the icon component)
    pub name: &'static str,
    pub color: &'static str,
    pub category: SkillCategory,
}

// Note: Icon components need to be defined and passed as functions.
// Example: fn() -> Element { rsx! { SiRust {} } }
pub const SKILLS_LIST: [SkillData; 11] = [
    SkillData { icon: || rsx!{ SiRust { size: 40 } }, name: "Rust", color: "text-orange-500", category: SkillCategory::Favorite },
    SkillData { icon: || rsx!{ SiSvelte { size: 40 } }, name: "Svelte", color: "text-orange-600", category: SkillCategory::Favorite },
    SkillData { icon: || rsx!{ DiDojo { size: 40 } }, name: "Dioxus", color: "text-purple-400", category: SkillCategory::Favorite },
    SkillData { icon: || rsx!{ SiPython { size: 40 } }, name: "Python", color: "text-yellow-500", category: SkillCategory::Proficient },
    SkillData { icon: || rsx!{ SiFastapi { size: 40 } }, name: "FastAPI", color: "text-green-500", category: SkillCategory::Backend },
    SkillData { icon: || rsx!{ SiNestjs { size: 40 } }, name: "NestJS", color: "text-red-500", category: SkillCategory::Backend },
    SkillData { icon: || rsx!{ SiReact { size: 40 } }, name: "React", color: "text-cyan-400", category: SkillCategory::Proficient },
    SkillData { icon: || rsx!{ SiNextdotjs { size: 40 } }, name: "Next.js", color: "text-white", category: SkillCategory::Proficient },
    SkillData { icon: || rsx!{ SiTailwindcss { size: 40 } }, name: "Tailwind", color: "text-cyan-500", category: SkillCategory::Proficient },
    SkillData { icon: || rsx!{ SiTypescript { size: 40 } }, name: "TypeScript", color: "text-blue-500", category: SkillCategory::Proficient },
    SkillData { icon: || rsx!{ SiNodedotjs { size: 40 } }, name: "Node.js", color: "text-green-600", category: SkillCategory::Proficient },
];


pub struct Project {
    pub title: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub image: &'static str,
    pub github_link: Option<&'static str>,
    pub live_link: Option<&'static str>,
    pub category: &'static str,
    pub details: Option<&'static [&'static str]>,
}

pub const PROJECTS_LIST: [Project; 4] = [
    Project {
        title: "EZPZ Polars Plugin Helper",
        description: "A Python package enhancing Polars plugin development with type hinting and IDE support. It parses plugin configurations, extracts metadata using libCST, and injects type hints into Polars classes.",
        tags: &["Python", "Rust", "Polars", "libCST", "CLI Tool"],
        image: "/placeholder.svg?height=400&width=600",
        github_link: Some("https://github.com/Summit-Sailors/EZPZ"),
        live_link: None,
        category: "Pet Project",
        details: Some(&[
            "Provides type hinting and IDE support for Polars plugins.",
            "Parses `ezpz_pluginz.toml` for plugin configurations.",
            "Uses libCST to analyze Python code and extract plugin information.",
            "Generates a lockfile for extracted plugin data.",
            "Modifies Polars files (with backups) to add type hints and imports within type checking blocks.",
        ]),
    },
    Project {
        title: "Lionheart Painting Website",
        description: "Pro bono website for a painting business that funds a recovery program for convicted felons, offering them work and support.",
        tags: &["Web Development", "Pro Bono", "Community"],
        image: "/placeholder.svg?height=400&width=600",
        github_link: None,
        live_link: Some("https://painting.lionheart.church/"),
        category: "Pro Bono Work",
        details: Some(&[
            "Developed a website to showcase the painting business and its mission.",
            "Aims to support a friend's program helping convicted felons reintegrate.",
            "The business provides employment and funding for the recovery program.",
        ]),
    },
    Project {
        title: "E-commerce Platform (Example)",
        description: "A full-featured e-commerce platform built with Next.js and Stripe integration.",
        tags: &["Next.js", "React", "Stripe", "Tailwind CSS"],
        image: "/placeholder.svg?height=400&width=600",
        github_link: Some("#"),
        live_link: Some("#"),
        category: "Corporate Experience",
        details: Some(&["Example detail 1 for e-commerce.", "Example detail 2."]),
    },
    Project {
        title: "Task Management App (Example)",
        description: "A collaborative task management application with real-time updates.",
        tags: &["React", "Firebase", "Tailwind CSS", "Redux"],
        image: "/placeholder.svg?height=400&width=600",
        github_link: Some("#"),
        live_link: Some("#"),
        category: "Corporate Experience",
        details: Some(&["Example detail 1 for task app.", "Example detail 2."]),
    },
];
