import Image from "next/image"
import { PROJECTS_DATA } from "@/constants/data"
import SectionTitle from "@/components/ui/section-title"
import { FiGithub, FiExternalLink } from "react-icons/fi"

export default function ProjectsPage() {
  return (
    <div className="py-24">
      <div className="container mx-auto px-4">
        <SectionTitle title="My Projects" />
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-10">
          {PROJECTS_DATA.map((project, index) => (
            <div
              key={project.title}
              id={project.title.toLowerCase().replace(/\s+/g, "-")}
              className="group bg-zinc-900 rounded-lg border border-zinc-800 overflow-hidden shadow-lg hover:shadow-purple-500/20 transition-all duration-300 ease-in-out flex flex-col animate-fade-in-up"
              style={{ animationDelay: `${index * 100}ms` }}
            >
              <div className="relative w-full h-56">
                <Image
                  src={project.image || "/placeholder.svg"}
                  alt={project.title}
                  fill
                  sizes="(max-width: 768px) 100vw, (max-width: 1200px) 50vw, 33vw"
                  className="object-cover transition-transform group-hover:scale-105 duration-300"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/70 to-transparent"></div>
                <div className="absolute top-2 right-2 bg-purple-600 text-white text-xs px-2 py-1 rounded">
                  {project.category}
                </div>
              </div>
              <div className="p-6 flex flex-col flex-grow">
                <h3 className="text-2xl font-bold mb-2 text-white">{project.title}</h3>
                <p className="text-zinc-400 text-sm mb-4 flex-grow">{project.description}</p>

                {project.details && project.details.length > 0 && (
                  <div className="mb-4">
                    <h4 className="text-sm font-semibold text-purple-400 mb-1">Key Features:</h4>
                    <ul className="list-disc list-inside text-zinc-400 text-xs space-y-1">
                      {project.details.slice(0, 3).map(
                        (
                          detail,
                          i, // Show max 3 details
                        ) => (
                          <li key={i}>{detail}</li>
                        ),
                      )}
                    </ul>
                  </div>
                )}

                <div className="flex flex-wrap gap-2 mb-6">
                  {project.tags.map((tag) => (
                    <span key={tag} className="px-3 py-1 bg-zinc-800 rounded-full text-xs text-zinc-300">
                      {tag}
                    </span>
                  ))}
                </div>
                <div className="mt-auto flex justify-between items-center">
                  <div className="flex space-x-3">
                    {project.githubLink && (
                      <a
                        href={project.githubLink}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-zinc-400 hover:text-purple-500 transition-colors"
                        aria-label={`${project.title} GitHub repository`}
                      >
                        <FiGithub size={22} />
                      </a>
                    )}
                    {project.liveLink && (
                      <a
                        href={project.liveLink}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-zinc-400 hover:text-purple-500 transition-colors"
                        aria-label={`${project.title} live demo`}
                      >
                        <FiExternalLink size={22} />
                      </a>
                    )}
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
