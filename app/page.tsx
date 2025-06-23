"use client"

import Image from "next/image"
import Link from "next/link"
import { FiArrowDown } from "react-icons/fi"
import { PERSONAL_INFO, SKILLS_DATA, PROJECTS_DATA } from "@/constants/data"
import SectionTitle from "@/components/ui/section-title"

export default function HomePage() {
  const featuredProjects = PROJECTS_DATA.slice(0, 2) // Show first 2 projects on homepage

  return (
    <>
      {/* Hero Section */}
      <section id="home" className="min-h-[calc(100vh-4rem)] flex items-center justify-center relative overflow-hidden">
        <div className="absolute inset-0 z-0">
          <div className="absolute top-1/4 left-1/4 size-96 bg-purple-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob"></div>
          <div className="absolute top-1/3 right-1/4 size-96 bg-cyan-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-2000"></div>
          <div className="absolute bottom-1/4 right-1/3 size-96 bg-pink-500 rounded-full mix-blend-multiply filter blur-3xl opacity-20 animate-blob animation-delay-4000"></div>
        </div>

        <div className="container mx-auto px-4 z-10">
          <div className="flex flex-col md:flex-row items-center justify-between gap-12">
            <div className="md:w-1/2 space-y-6 text-center md:text-left">
              <div className="text-sm font-mono text-purple-400 animate-fade-in-up animation-delay-100">
                Hello, my name is
              </div>
              <h1 className="text-5xl md:text-7xl font-bold animate-fade-in-up animation-delay-200">
                {PERSONAL_INFO.name}
              </h1>
              <h2 className="text-3xl md:text-5xl font-bold text-zinc-400 animate-fade-in-up animation-delay-300">
                {PERSONAL_INFO.title}
              </h2>
              <p className="text-zinc-300 max-w-lg mx-auto md:mx-0 animate-fade-in-up animation-delay-400">
                {PERSONAL_INFO.shortDescription}
              </p>
              <div className="flex space-x-4 justify-center md:justify-start animate-fade-in-up animation-delay-500">
                <Link
                  href="/projects"
                  className="px-6 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors"
                >
                  View My Work
                </Link>
                <Link
                  href="/contact"
                  className="px-6 py-3 border border-zinc-700 hover:border-purple-500 rounded-md font-medium transition-colors"
                >
                  Contact Me
                </Link>
              </div>
            </div>
            <div className="md:w-1/2 flex justify-center animate-fade-in-scale">
              <div className="relative size-64 md:size-80">
                <div className="absolute inset-0 rounded-full bg-gradient-to-r from-purple-500 to-cyan-500 blur-xl opacity-30 animate-pulse-slow"></div>
                <div className="relative w-full h-full rounded-full overflow-hidden border-4 border-zinc-800">
                  <Image
                    src={`/placeholder.svg?height=320&width=320&query=Jeremy+Meek+professional+headshot`}
                    alt={PERSONAL_INFO.name}
                    width={320}
                    height={320}
                    className="object-cover"
                    priority
                  />
                </div>
              </div>
            </div>
          </div>
          <div className="absolute bottom-10 left-1/2 transform -translate-x-1/2 animate-bounce hidden md:block">
            <Link href="#featured-projects" aria-label="Scroll to projects" className="text-zinc-400 hover:text-white">
              <FiArrowDown size={24} />
            </Link>
          </div>
        </div>
      </section>

      {/* Featured Projects Section (on Home Page) */}
      <section id="featured-projects" className="py-24">
        <div className="container mx-auto px-4">
          <SectionTitle title="Featured Projects" />
          <div className="grid md:grid-cols-2 gap-8">
            {featuredProjects.map((project, index) => (
              <div
                key={project.title}
                className={`group animate-fade-in-up`}
                style={{ animationDelay: `${index * 100}ms` }}
              >
                <div className="relative overflow-hidden rounded-lg border border-zinc-800 group-hover:border-purple-500 transition-colors">
                  <Image
                    src={project.image || "/placeholder.svg"}
                    alt={project.title}
                    width={600}
                    height={350} // Adjusted height for better aspect ratio
                    className="w-full h-64 object-cover transition-transform group-hover:scale-105"
                  />
                  <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity z-10 flex flex-col justify-end p-6">
                    <h3 className="text-2xl font-bold text-white mb-2">{project.title}</h3>
                    <p className="text-zinc-300 text-sm mb-3">{project.description.substring(0, 100)}...</p>
                    <Link
                      href={`/projects#${project.title.toLowerCase().replace(/\s+/g, "-")}`}
                      className="text-purple-400 hover:text-purple-300 font-medium self-start"
                    >
                      Learn More &rarr;
                    </Link>
                  </div>
                </div>
              </div>
            ))}
          </div>
          <div className="text-center mt-12 animate-fade-in-up animation-delay-300">
            <Link
              href="/projects"
              className="px-8 py-3 bg-purple-600 hover:bg-purple-700 rounded-md font-medium transition-colors text-lg"
            >
              View All Projects
            </Link>
          </div>
        </div>
      </section>

      {/* Skills Section (on Home Page) */}
      <section id="skills" className="py-24 bg-zinc-900/50">
        <div className="container mx-auto px-4">
          <SectionTitle title="My Skills" />
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-6">
            {SKILLS_DATA.map((skill, index) => (
              <div
                key={skill.name}
                className={`bg-zinc-900 p-6 rounded-lg border border-zinc-800 flex flex-col items-center justify-center text-center hover:border-purple-500 transition-all hover:-translate-y-1 animate-fade-in-up`}
                style={{ animationDelay: `${index * 75}ms` }}
              >
                <div className={`mb-3 ${skill.color}`}>{skill.icon}</div>
                <h3 className="text-md font-medium">{skill.name}</h3>
                <p className="text-xs text-zinc-400 mt-1">{skill.category}</p>
              </div>
            ))}
          </div>
        </div>
      </section>
    </>
  )
}
