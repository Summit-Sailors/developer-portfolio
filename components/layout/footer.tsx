import { PERSONAL_INFO } from "@/constants/data"
import Link from "next/link"
import { FiGithub, FiLinkedin, FiTwitter } from "react-icons/fi"

export default function Footer() {
  return (
    <footer className="py-8 border-t border-zinc-800">
      <div className="container mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center gap-6">
          <div className="text-center md:text-left">
            <Link
              href="/"
              className="text-xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-purple-500 to-cyan-500 mb-4 md:mb-0"
            >
              {PERSONAL_INFO.name.split(" ")[0]}.dev
            </Link>
            <p className="text-zinc-400 text-sm mt-1">
              © {new Date().getFullYear()} {PERSONAL_INFO.name}. All rights reserved.
            </p>
          </div>
          <div className="flex space-x-4">
            <a
              href={PERSONAL_INFO.github}
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-400 hover:text-purple-500 transition-colors"
            >
              <FiGithub size={24} />
            </a>
            <a
              href={PERSONAL_INFO.linkedin}
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-400 hover:text-purple-500 transition-colors"
            >
              <FiLinkedin size={24} />
            </a>
            <a
              href={PERSONAL_INFO.twitter}
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-400 hover:text-purple-500 transition-colors"
            >
              <FiTwitter size={24} />
            </a>
          </div>
        </div>
      </div>
    </footer>
  )
}
