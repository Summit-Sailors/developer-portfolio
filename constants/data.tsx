import type { ReactNode } from "react"
import {
  SiReact,
  SiNextdotjs,
  SiTailwindcss,
  SiTypescript,
  SiNodedotjs,
  SiPython,
  SiRust,
  SiSvelte,
  SiFastapi,
  SiNestjs,
} from "react-icons/si"
import { DiDojo } from "react-icons/di" // For Dioxus, using a generic Dojo icon as placeholder

export const PERSONAL_INFO = {
  name: "Jeremy Meek",
  title: "Self-Starting Polymath Autodidact.",
  shortDescription:
    "I build robust and innovative solutions for the web and beyond. From cows, to calculus, to computers.",
  email: "jeremy.meek@example.com", // Replace with actual email
  phone: "+1 (555) 555-5555", // Replace with actual phone
  location: "Southwest Virginia, USA",
  storyIntro:
    "Hello! I'm Jeremy Meek, a software developer with a unique journey from farm life in Southwest Virginia, through a Bachelors in Mathematics at Virginia Tech, to a Computer Science education from Bloom Tech. I bring several years of full-stack web development experience from the corporate world, ready to tackle virtually any programming challenge.",
  storyDetail: [
    "My path has been unconventional: from tending cows to unraveling calculus, and now, crafting complex computer systems. This diverse background has equipped me with a unique problem-solving perspective and a relentless drive for learning.",
    "While I'm proficient in mainstream frameworks and languages for professional projects, my passion lies with Rust for its performance and safety. My pet projects often involve Rust, sometimes combined with Python. Svelte is my go-to for frontend JavaScript work, and for non-Rust backends, I prefer FastAPI (Python) or NestJS (Node). When I get to use Rust for full-stack, Dioxus is my framework of choice.",
    "I consider myself an 'Artist Of All Trades' in the tech world, competent to satisfy a wide array of programming needs with creativity and precision.",
  ],
  github: "https://github.com/your-github-username", // Replace
  linkedin: "https://linkedin.com/in/your-linkedin-profile", // Replace
  twitter: "https://twitter.com/your-twitter-handle", // Replace
}

export const NAVIGATION_LINKS = [
  { name: "Home", href: "/" },
  { name: "Projects", href: "/projects" },
  { name: "Skills", href: "#skills" }, // Assuming skills remain on homepage for now
  { name: "Story", href: "/story" },
  { name: "Contact", href: "/contact" },
]

export const STATS = [
  { value: "N+", label: "Years of Experience" }, // Update N
  { value: "M+", label: "Projects Completed" }, // Update M
  { value: "B.S.", label: "Mathematics, Virginia Tech" },
  { value: "Cert.", label: "Computer Science, Bloom Tech" },
]

export const SKILLS_DATA: {
  icon: ReactNode
  name: string
  color: string
  category: "Favorite" | "Proficient" | "Backend"
}[] = [
  { icon: <SiRust size={40} />, name: "Rust", color: "text-orange-500", category: "Favorite" },
  { icon: <SiSvelte size={40} />, name: "Svelte", color: "text-orange-600", category: "Favorite" },
  { icon: <DiDojo size={40} />, name: "Dioxus", color: "text-purple-400", category: "Favorite" },
  { icon: <SiPython size={40} />, name: "Python", color: "text-yellow-500", category: "Proficient" },
  { icon: <SiFastapi size={40} />, name: "FastAPI", color: "text-green-500", category: "Backend" },
  { icon: <SiNestjs size={40} />, name: "NestJS", color: "text-red-500", category: "Backend" },
  { icon: <SiReact size={40} />, name: "React", color: "text-cyan-400", category: "Proficient" },
  { icon: <SiNextdotjs size={40} />, name: "Next.js", color: "text-white", category: "Proficient" },
  { icon: <SiTailwindcss size={40} />, name: "Tailwind", color: "text-cyan-500", category: "Proficient" },
  { icon: <SiTypescript size={40} />, name: "TypeScript", color: "text-blue-500", category: "Proficient" },
  { icon: <SiNodedotjs size={40} />, name: "Node.js", color: "text-green-600", category: "Proficient" },
]

export const PROJECTS_DATA = [
  {
    title: "EZPZ Polars Plugin Helper",
    description:
      "A Python package enhancing Polars plugin development with type hinting and IDE support. It parses plugin configurations, extracts metadata using libCST, and injects type hints into Polars classes.",
    tags: ["Python", "Rust", "Polars", "libCST", "CLI Tool"],
    image: "/placeholder.svg?height=400&width=600",
    githubLink: "https://github.com/Summit-Sailors/EZPZ",
    liveLink: "", // If applicable, e.g., PyPI link
    category: "Pet Project",
    details: [
      "Provides type hinting and IDE support for Polars plugins.",
      "Parses `ezpz_pluginz.toml` for plugin configurations.",
      "Uses libCST to analyze Python code and extract plugin information.",
      "Generates a lockfile for extracted plugin data.",
      "Modifies Polars files (with backups) to add type hints and imports within type checking blocks.",
    ],
  },
  {
    title: "Lionheart Painting Website",
    description:
      "Pro bono website for a painting business that funds a recovery program for convicted felons, offering them work and support.",
    tags: ["Web Development", "Pro Bono", "Community"],
    image: "/placeholder.svg?height=400&width=600",
    liveLink: "https://painting.lionheart.church/",
    githubLink: "", // If private or not applicable
    category: "Pro Bono Work",
    details: [
      "Developed a website to showcase the painting business and its mission.",
      "Aims to support a friend's program helping convicted felons reintegrate.",
      "The business provides employment and funding for the recovery program.",
    ],
  },
  {
    title: "E-commerce Platform (Example)",
    description: "A full-featured e-commerce platform built with Next.js and Stripe integration.",
    tags: ["Next.js", "React", "Stripe", "Tailwind CSS"],
    image: "/placeholder.svg?height=400&width=600",
    category: "Corporate Experience",
    details: ["Example detail 1 for e-commerce.", "Example detail 2."],
  },
  {
    title: "Task Management App (Example)",
    description: "A collaborative task management application with real-time updates.",
    tags: ["React", "Firebase", "Tailwind CSS", "Redux"],
    image: "/placeholder.svg?height=400&width=600",
    category: "Corporate Experience",
    details: ["Example detail 1 for task app.", "Example detail 2."],
  },
]
